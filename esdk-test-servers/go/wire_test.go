// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Credential-free tests that drive the real rpcv2Cbor wire path over HTTP
// against an ephemeral localhost port. No KMS or network beyond the loopback.

package main

import (
	"bytes"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/aws/smithy-go/encoding/cbor"
)

func startServer(t *testing.T) *httptest.Server {
	t.Helper()
	state, err := newAppState()
	if err != nil {
		t.Fatalf("newAppState: %v", err)
	}
	server := httptest.NewServer(newHandler(state))
	t.Cleanup(server.Close)
	return server
}

// post sends one rpcv2Cbor request and returns the HTTP status and decoded
// CBOR body. Entries in headers override the protocol defaults; an empty value
// removes the header. Asserts the response protocol headers on every exchange.
func post(t *testing.T, server *httptest.Server, operation string, body cbor.Value, headers map[string]string) (int, cbor.Map) {
	t.Helper()
	url := server.URL + "/service/ESDKTestServer/operation/" + operation
	req, err := http.NewRequest(http.MethodPost, url, bytes.NewReader(cbor.Encode(body)))
	if err != nil {
		t.Fatalf("build request: %v", err)
	}
	req.Header.Set("smithy-protocol", smithyProtocol)
	req.Header.Set("Content-Type", cborContentType)
	for key, value := range headers {
		if value == "" {
			req.Header.Del(key)
		} else {
			req.Header.Set(key, value)
		}
	}
	resp, err := server.Client().Do(req)
	if err != nil {
		t.Fatalf("%s request: %v", operation, err)
	}
	defer resp.Body.Close()
	if got := resp.Header.Get("smithy-protocol"); got != smithyProtocol {
		t.Fatalf("response smithy-protocol = %q, want %q", got, smithyProtocol)
	}
	if got := resp.Header.Get("Content-Type"); got != cborContentType {
		t.Fatalf("response content-type = %q, want %q", got, cborContentType)
	}
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("read response body: %v", err)
	}
	decoded, err := cbor.Decode(raw)
	if err != nil {
		t.Fatalf("decode response CBOR: %v", err)
	}
	m, ok := decoded.(cbor.Map)
	if !ok {
		t.Fatalf("response body is not a CBOR map: %T", decoded)
	}
	return resp.StatusCode, m
}

func text(t *testing.T, m cbor.Map, key string) string {
	t.Helper()
	s, ok := m[key].(cbor.String)
	if !ok {
		t.Fatalf("member %s missing or not a text string: %#v", key, m[key])
	}
	return string(s)
}

func blob(t *testing.T, m cbor.Map, key string) []byte {
	t.Helper()
	b, ok := m[key].(cbor.Slice)
	if !ok {
		t.Fatalf("member %s missing or not a byte string: %#v", key, m[key])
	}
	return []byte(b)
}

// requireGenericError asserts the modeled GenericServerError wire form and
// returns its message.
func requireGenericError(t *testing.T, status int, m cbor.Map) string {
	t.Helper()
	if status != http.StatusBadRequest {
		t.Fatalf("status = %d, want 400; body %#v", status, m)
	}
	if got := text(t, m, "__type"); got != genericErrorType {
		t.Fatalf("__type = %q, want %q", got, genericErrorType)
	}
	return text(t, m, "message")
}

// rawAesCreateClientBody is the offline Raw-AES / Default-CMM CreateClient
// request the commons BlobRoundTrip behaviors use.
func rawAesCreateClientBody(maxEncryptedDataKeys *int64) cbor.Map {
	wrappingKey := make([]byte, 32)
	for i := range wrappingKey {
		wrappingKey[i] = byte(i)
	}
	config := cbor.Map{
		"commitmentPolicy": cbor.String("REQUIRE_ENCRYPT_REQUIRE_DECRYPT"),
		"cmm": cbor.Map{
			"Default": cbor.Map{
				"keyring": cbor.Map{
					"RawAes": cbor.Map{
						"keyNamespace": cbor.String("esdk-test-server"),
						"keyName":      cbor.String("raw-aes-round-trip-key"),
						"wrappingKey":  cbor.Slice(wrappingKey),
						"wrappingAlg":  cbor.String("ALG_AES256_GCM_IV12_TAG16"),
					},
				},
			},
		},
	}
	if maxEncryptedDataKeys != nil {
		config["maxEncryptedDataKeys"] = cbor.Uint(*maxEncryptedDataKeys)
	}
	return cbor.Map{"config": config}
}

func createRawAesClient(t *testing.T, server *httptest.Server) string {
	t.Helper()
	status, body := post(t, server, "CreateClient", rawAesCreateClientBody(nil), nil)
	if status != http.StatusOK {
		t.Fatalf("CreateClient status = %d, body %#v", status, body)
	}
	clientID := text(t, body, "clientId")
	if clientID == "" {
		t.Fatal("CreateClient returned an empty clientId")
	}
	return clientID
}

func TestRawAesRoundTrip(t *testing.T) {
	server := startServer(t)
	clientID := createRawAesClient(t, server)

	plaintext := []byte("Hello ESDK TestServer round trip.")
	suite := "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY"
	status, encrypted := post(t, server, "Encrypt", cbor.Map{
		"clientId":          cbor.String(clientID),
		"plaintext":         cbor.Slice(plaintext),
		"encryptionContext": cbor.Map{"purpose": cbor.String("round-trip")},
		"algorithmSuiteId":  cbor.String(suite),
		"frameLength":       cbor.Uint(1024),
	}, nil)
	if status != http.StatusOK {
		t.Fatalf("Encrypt status = %d, body %#v", status, encrypted)
	}
	ciphertext := blob(t, encrypted, "ciphertext")
	if bytes.Equal(ciphertext, plaintext) {
		t.Fatal("ciphertext equals the plaintext")
	}

	status, decrypted := post(t, server, "Decrypt", cbor.Map{
		"clientId":   cbor.String(clientID),
		"ciphertext": cbor.Slice(ciphertext),
	}, nil)
	if status != http.StatusOK {
		t.Fatalf("Decrypt status = %d, body %#v", status, decrypted)
	}
	if got := blob(t, decrypted, "plaintext"); !bytes.Equal(got, plaintext) {
		t.Fatalf("decrypted plaintext = %q, want %q", got, plaintext)
	}
	ec, ok := decrypted["encryptionContext"].(cbor.Map)
	if !ok {
		t.Fatalf("decrypt response encryptionContext missing or not a map: %#v", decrypted["encryptionContext"])
	}
	if got := text(t, ec, "purpose"); got != "round-trip" {
		t.Fatalf(`encryptionContext["purpose"] = %q, want "round-trip"`, got)
	}
	if got := text(t, decrypted, "algorithmSuiteId"); got != suite {
		t.Fatalf("decrypt response algorithmSuiteId = %q, want %q", got, suite)
	}

	// A second client built from the same config decrypts the same message
	// without being given the encryption context: it travels in the header.
	secondID := createRawAesClient(t, server)
	status, second := post(t, server, "Decrypt", cbor.Map{
		"clientId":   cbor.String(secondID),
		"ciphertext": cbor.Slice(ciphertext),
	}, nil)
	if status != http.StatusOK {
		t.Fatalf("second-client Decrypt status = %d, body %#v", status, second)
	}
	if got := blob(t, second, "plaintext"); !bytes.Equal(got, plaintext) {
		t.Fatalf("second-client plaintext = %q, want %q", got, plaintext)
	}
}

func TestMissingOrWrongSmithyProtocolHeader(t *testing.T) {
	server := startServer(t)

	status, body := post(t, server, "CreateClient", rawAesCreateClientBody(nil),
		map[string]string{"smithy-protocol": ""})
	requireGenericError(t, status, body)

	status, body = post(t, server, "CreateClient", rawAesCreateClientBody(nil),
		map[string]string{"smithy-protocol": "rpc-v2-json"})
	requireGenericError(t, status, body)
}

func TestUnknownOperation(t *testing.T) {
	server := startServer(t)
	status, body := post(t, server, "Frobnicate", cbor.Map{}, nil)
	requireGenericError(t, status, body)
}

func TestEncryptStreamUnsupported(t *testing.T) {
	server := startServer(t)
	status, body := post(t, server, "EncryptStream", cbor.Map{}, nil)
	message := requireGenericError(t, status, body)
	want := "streaming operations are not supported by the go language server"
	if message != want {
		t.Fatalf("message = %q, want %q", message, want)
	}
}

func TestEncryptUnknownClientID(t *testing.T) {
	server := startServer(t)
	status, body := post(t, server, "Encrypt", cbor.Map{
		"clientId":  cbor.String("no-such-client"),
		"plaintext": cbor.Slice([]byte("plaintext")),
	}, nil)
	requireGenericError(t, status, body)
}

func TestCreateClientRejectsZeroMaxEncryptedDataKeys(t *testing.T) {
	server := startServer(t)
	zero := int64(0)
	status, body := post(t, server, "CreateClient", rawAesCreateClientBody(&zero), nil)
	requireGenericError(t, status, body)
}
