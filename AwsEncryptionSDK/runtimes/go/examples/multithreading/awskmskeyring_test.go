// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package multithreading

import (
	"testing"

	"github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/examples/utils"
)

// TestAWSKMSMultiThreadRace exercises concurrent encrypt/decrypt to detect data races.
// Run with: go test -race -count=1 -timeout 5m ./multithreading/
func TestAWSKMSMultiThreadRace(t *testing.T) {
	AWSKMSMultiThreadTest(
		utils.GenerateUUIDTestData(100),
		utils.DefaultKMSKeyId(),
		utils.DefaultKmsKeyRegion(),
	)
}
