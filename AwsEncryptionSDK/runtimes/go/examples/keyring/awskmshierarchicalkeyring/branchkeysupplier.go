// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package awskmshierarchicalkeyring

import (
	"fmt"

	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
)

/*
Demonstrates how to create a BranchKeyIdSupplier.

The BranchKeyIdSupplier determines which Branch Key is used
to protect or access data.
It is an important component in a Multi-tenant solution,
where each tenant is cryptographically isolated.
The Branch Key ID Supplier uses the Encryption Context
provided at Encrypt or Decrypt
to determine what "shared secret" (Branch Key)
is used.
*/

type branchKeySupplier struct {
	branchKeyA string
	branchKeyB string
}

func (b *branchKeySupplier) GetBranchKeyId(input mpltypes.GetBranchKeyIdInput) (*mpltypes.GetBranchKeyIdOutput, error) {
	// We MUST use the encryption context to determine
	// the Branch Key ID.
	ec := input.EncryptionContext
	if value, exists := ec["tenant"]; !exists || value == "" {
		return nil, fmt.Errorf("EncryptionContext invalid, does not contain expected tenant key value pair.")
	}
	branchKeyIdentifier := ec["tenant"]
	if branchKeyIdentifier == "TenantA" {
		return &mpltypes.GetBranchKeyIdOutput{BranchKeyId: b.branchKeyA}, nil
	} else if branchKeyIdentifier == "TenantB" {
		return &mpltypes.GetBranchKeyIdOutput{BranchKeyId: b.branchKeyB}, nil
	} else {
		return &mpltypes.GetBranchKeyIdOutput{}, fmt.Errorf("unknown branch key identifier")
	}
}
