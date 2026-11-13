// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Operation handlers and the clientId registry. CreateClient builds the
// materials manager from the modeled config (delegating to the MPL), constructs
// the real ESDK client eagerly, and registers both under a fresh UUID. Encrypt
// and Decrypt resolve the registered client by clientId and forward to the real
// library, always through the materials manager.

package main

import (
	"context"
	"fmt"
	"reflect"
	"strings"
	"sync"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	esdkclient "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/google/uuid"
)

// clientEntry is one registered ESDK client and the materials manager it
// encrypts and decrypts with.
type clientEntry struct {
	esdk *esdkclient.Client
	cmm  mpltypes.ICryptographicMaterialsManager
}

// appState is the shared server state: one MPL client for keyring/CMM
// construction and the thread-safe clientId -> clientEntry registry.
type appState struct {
	matProv *mpl.Client
	mu      sync.Mutex
	clients map[string]*clientEntry
}

func newAppState() (*appState, error) {
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		return nil, err
	}
	return &appState{matProv: matProv, clients: make(map[string]*clientEntry)}, nil
}

// createClient eagerly builds the materials manager and the real ESDK client;
// any construction or validation failure is a GenericServerError.
func (s *appState) createClient(ctx context.Context, req *CreateClientRequest) (*CreateClientResponse, *serverError) {
	cmm, err := buildCMM(ctx, s.matProv, req.Config.CMM)
	if err != nil {
		return nil, generic("CreateClient failed to construct the ESDK client: %s", describeError(err))
	}
	policy := req.Config.CommitmentPolicy
	esdkConfig := esdktypes.AwsEncryptionSdkConfig{
		CommitmentPolicy:     &policy,
		MaxEncryptedDataKeys: req.Config.MaxEncryptedDataKeys,
	}
	// NewClient does not run the config's own validation, so run it here:
	// config failures (e.g. maxEncryptedDataKeys < 1) must surface at
	// CreateClient time.
	if err := esdkConfig.Validate(); err != nil {
		return nil, generic("CreateClient failed to construct the ESDK client: %s", describeError(err))
	}
	esdk, err := esdkclient.NewClient(esdkConfig)
	if err != nil {
		return nil, generic("CreateClient failed to construct the ESDK client: %s", describeError(err))
	}
	clientID := uuid.NewString()
	s.mu.Lock()
	s.clients[clientID] = &clientEntry{esdk: esdk, cmm: cmm}
	s.mu.Unlock()
	return &CreateClientResponse{ClientID: clientID}, nil
}

func (s *appState) encrypt(ctx context.Context, req *EncryptRequest) (*EncryptResponse, *serverError) {
	entry, serr := s.resolve(req.ClientID)
	if serr != nil {
		return nil, serr
	}
	out, err := entry.esdk.Encrypt(ctx, esdktypes.EncryptInput{
		Plaintext:         req.Plaintext,
		MaterialsManager:  entry.cmm,
		EncryptionContext: req.EncryptionContext,
		AlgorithmSuiteId:  req.AlgorithmSuiteID,
		FrameLength:       req.FrameLength,
	})
	if err != nil {
		return nil, esdkClientError(describeError(err))
	}
	return &EncryptResponse{Ciphertext: out.Ciphertext}, nil
}

func (s *appState) decrypt(ctx context.Context, req *DecryptRequest) (*DecryptResponse, *serverError) {
	entry, serr := s.resolve(req.ClientID)
	if serr != nil {
		return nil, serr
	}
	out, err := entry.esdk.Decrypt(ctx, esdktypes.DecryptInput{
		Ciphertext:        req.Ciphertext,
		MaterialsManager:  entry.cmm,
		EncryptionContext: req.EncryptionContext,
	})
	if err != nil {
		return nil, esdkClientError(describeError(err))
	}
	return &DecryptResponse{
		Plaintext:         out.Plaintext,
		EncryptionContext: out.EncryptionContext,
		AlgorithmSuiteID:  out.AlgorithmSuiteId,
	}, nil
}

// resolve looks up the registered client; a missing, empty, or unknown
// clientId is a GenericServerError.
func (s *appState) resolve(clientID string) (*clientEntry, *serverError) {
	if clientID == "" {
		return nil, generic("clientId must be non-empty")
	}
	s.mu.Lock()
	entry := s.clients[clientID]
	s.mu.Unlock()
	if entry == nil {
		return nil, generic("unknown clientId: %s", clientID)
	}
	return entry, nil
}

// describeError extracts the library error's message, flattening nested
// collected errors. The generated exception types share three field shapes
// across packages ({Message}, {ErrObject}, {ListOfErrors, Message}), and their
// Error() methods format a nil ErrorCodeOverride into the string, so read the
// fields reflectively instead.
func describeError(err error) string {
	v := reflect.ValueOf(err)
	if v.Kind() == reflect.Ptr && !v.IsNil() {
		v = v.Elem()
	}
	if v.Kind() != reflect.Struct {
		return err.Error()
	}
	message := ""
	described := false
	if f := v.FieldByName("Message"); f.IsValid() && f.Kind() == reflect.String && f.String() != "" {
		message = f.String()
		described = true
	}
	if f := v.FieldByName("ErrObject"); f.IsValid() && f.Kind() == reflect.Interface && !f.IsNil() {
		if inner, ok := f.Interface().(error); ok {
			message = describeError(inner)
		} else {
			message = fmt.Sprintf("%v", f.Interface())
		}
		described = true
	}
	if f := v.FieldByName("ListOfErrors"); f.IsValid() && f.Kind() == reflect.Slice && f.Len() > 0 {
		causes := make([]string, 0, f.Len())
		for i := 0; i < f.Len(); i++ {
			if inner, ok := f.Index(i).Interface().(error); ok {
				causes = append(causes, describeError(inner))
			}
		}
		encountered := fmt.Sprintf("[encountered: %s]", strings.Join(causes, "; "))
		if message != "" {
			message += " " + encountered
		} else {
			message = encountered
		}
		described = true
	}
	if !described {
		return err.Error()
	}
	return message
}
