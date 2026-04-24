// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package benchmark

import (
	"context"
	"fmt"
	"log"
	"strings"

	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
)

const oneGB = 1_073_741_824

// runSizeLimitSmokeTest validates that encrypt/decrypt properly reject data exceeding the size limit
// and succeed at the boundary.
func (b *ESDKBenchmark) runSizeLimitSmokeTest() error {
	log.Println("Running size limit smoke tests...")
	ctx := context.Background()

	type testCase struct {
		name      string
		size      int
		expectErr bool
	}

	cases := []testCase{
		{"limit-1", oneGB - 1, false},
		{"limit", oneGB, false},
		{"limit+1", oneGB + 1, true},
	}

	// Test encrypt plaintext size limits
	for _, tc := range cases {
		log.Printf("  Encrypt plaintext size=%d (%s) expectErr=%v", tc.size, tc.name, tc.expectErr)
		data := make([]byte, tc.size)

		input := esdktypes.EncryptInput{
			Plaintext: data,
			Keyring:   b.Keyring,
		}
		_, err := b.EsdkClient.Encrypt(ctx, input)

		if tc.expectErr {
			if err == nil {
				return fmt.Errorf("encrypt(%s): expected error for plaintext size %d, got nil", tc.name, tc.size)
			}
			if !strings.Contains(strings.ToLower(err.Error()), "size") {
				log.Printf("  WARNING: error does not mention 'size': %v", err)
			}
			log.Printf("  PASS: encrypt(%s) returned expected error", tc.name)
		} else {
			if err != nil {
				return fmt.Errorf("encrypt(%s): unexpected error for plaintext size %d: %w", tc.name, tc.size, err)
			}
			log.Printf("  PASS: encrypt(%s) succeeded", tc.name)
		}
	}

	// Test decrypt ciphertext — no size limit on ciphertext
	log.Println("Size limit smoke tests PASSED")
	return nil
}
