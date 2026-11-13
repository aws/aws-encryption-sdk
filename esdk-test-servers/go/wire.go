// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// rpcv2Cbor HTTP wire layer: routes POST /service/{Service}/operation/{Operation},
// validates the protocol headers, decodes the CBOR request body, dispatches to
// a handler, and encodes the CBOR response or a modeled error.

package main

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"strconv"
	"strings"

	"github.com/aws/smithy-go/encoding/cbor"
)

const (
	errorNamespace      = "aws.cryptography.esdk.testserver"
	genericErrorType    = errorNamespace + "#GenericServerError"
	esdkClientErrorType = errorNamespace + "#ESDKClientError"
	smithyProtocol      = "rpc-v2-cbor"
	cborContentType     = "application/cbor"
)

// serverError is one of the two modeled TestServer errors.
type serverError struct {
	typeID  string
	message string
}

// generic builds a GenericServerError: a failure originating in the TestServer
// framework itself (bad headers, unknown clientId, unset union variant,
// client-construction failure, streaming on a non-streaming server).
func generic(format string, args ...any) *serverError {
	return &serverError{typeID: genericErrorType, message: fmt.Sprintf(format, args...)}
}

// esdkClientError builds an ESDKClientError: a failure forwarded from the
// underlying ESDK, carrying the library error's message.
func esdkClientError(message string) *serverError {
	return &serverError{typeID: esdkClientErrorType, message: message}
}

// newHandler builds the HTTP handler. Shared with tests so they drive the real
// wire path.
func newHandler(state *appState) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		status, body := dispatch(state, r)
		w.Header().Set("smithy-protocol", smithyProtocol)
		w.Header().Set("Content-Type", cborContentType)
		// An explicit Content-Length keeps every response eligible for
		// HTTP/1.1 keep-alive (the generated Test_Client pools connections).
		w.Header().Set("Content-Length", strconv.Itoa(len(body)))
		w.WriteHeader(status)
		_, _ = w.Write(body)
	})
}

// dispatch runs one operation and returns the HTTP status and CBOR body. Every
// outcome is a modeled success response, a GenericServerError, or an
// ESDKClientError; panics (the generated Dafny bindings panic on some
// construction failures) become GenericServerError.
func dispatch(state *appState, r *http.Request) (status int, body []byte) {
	defer func() {
		if p := recover(); p != nil {
			status, body = errorResponse(generic("unexpected server error: %v", p))
		}
	}()

	if r.Method != http.MethodPost {
		return errorResponse(generic("unsupported method %s; expected POST", r.Method))
	}
	service, operation, ok := parsePath(r.URL.Path)
	if !ok {
		return errorResponse(generic("unknown operation: %s", r.URL.Path))
	}
	if service != "ESDKTestServer" {
		return errorResponse(generic("unknown service: %s; expected ESDKTestServer", service))
	}
	if r.Header.Get("smithy-protocol") != smithyProtocol {
		return errorResponse(generic("missing or invalid smithy-protocol header; expected %s", smithyProtocol))
	}
	if r.Header.Get("Content-Type") != cborContentType {
		return errorResponse(generic("missing or invalid content-type; expected %s", cborContentType))
	}
	payload, err := io.ReadAll(r.Body)
	if err != nil {
		return errorResponse(generic("failed to read request body: %v", err))
	}

	var out cbor.Value
	var serr *serverError
	switch operation {
	case "CreateClient":
		out, serr = createClientOp(r.Context(), state, payload)
	case "Encrypt":
		out, serr = encryptOp(r.Context(), state, payload)
	case "Decrypt":
		out, serr = decryptOp(r.Context(), state, payload)
	case "EncryptStream", "DecryptStream":
		serr = generic("streaming operations are not supported by the go language server")
	default:
		serr = generic("unknown operation: %s", operation)
	}
	if serr != nil {
		return errorResponse(serr)
	}
	return http.StatusOK, cbor.Encode(out)
}

func createClientOp(ctx context.Context, state *appState, payload []byte) (cbor.Value, *serverError) {
	req, err := decodeCreateClientRequest(payload)
	if err != nil {
		return nil, generic("failed to decode CBOR request: %v", err)
	}
	resp, serr := state.createClient(ctx, req)
	if serr != nil {
		return nil, serr
	}
	return resp.toCBOR(), nil
}

func encryptOp(ctx context.Context, state *appState, payload []byte) (cbor.Value, *serverError) {
	req, err := decodeEncryptRequest(payload)
	if err != nil {
		return nil, generic("failed to decode CBOR request: %v", err)
	}
	resp, serr := state.encrypt(ctx, req)
	if serr != nil {
		return nil, serr
	}
	return resp.toCBOR(), nil
}

func decryptOp(ctx context.Context, state *appState, payload []byte) (cbor.Value, *serverError) {
	req, err := decodeDecryptRequest(payload)
	if err != nil {
		return nil, generic("failed to decode CBOR request: %v", err)
	}
	resp, serr := state.decrypt(ctx, req)
	if serr != nil {
		return nil, serr
	}
	return resp.toCBOR(), nil
}

// parsePath extracts the service and operation from /service/{s}/operation/{o}.
func parsePath(path string) (service, operation string, ok bool) {
	parts := strings.Split(strings.Trim(path, "/"), "/")
	if len(parts) != 4 || parts[0] != "service" || parts[2] != "operation" {
		return "", "", false
	}
	return parts[1], parts[3], true
}

// errorResponse serializes a modeled error to its rpcv2Cbor wire form: HTTP 400
// (both errors carry @error("client")) with a CBOR map carrying the __type
// discriminator so the generated Test_Client maps it back to the modeled type.
func errorResponse(e *serverError) (int, []byte) {
	return http.StatusBadRequest, cbor.Encode(cbor.Map{
		"__type":  cbor.String(e.typeID),
		"message": cbor.String(e.message),
	})
}
