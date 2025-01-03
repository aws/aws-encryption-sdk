// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package awskmshierarchicalkeyring

import (
	"context"

	keystore "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographykeystoresmithygenerated"
	keystoretypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographykeystoresmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

/*
 The Hierarchical Keyring Example relies on the existence
 of a DDB-backed key store with pre-existing
 branch key material.

 This example demonstrates configuring a KeyStore and creating a branch key.
*/

func createbranchkeyid(keyStoreTableName, logicalKeyStoreName, kmsKeyArn string, ddbClient *dynamodb.Client, kmsClient *kms.Client) (string, error) {
	// 1. Create the keystore
	// The KMS Configuration you use in the KeyStore MUST have the right access to the resources in the KeyStore.
	kmsConfig := keystoretypes.KMSConfigurationMemberkmsKeyArn{
		Value: kmsKeyArn,
	}
	keyStore, err := keystore.NewClient(keystoretypes.KeyStoreConfig{
		DdbTableName:        keyStoreTableName,
		KmsConfiguration:    &kmsConfig,
		LogicalKeyStoreName: logicalKeyStoreName,
		DdbClient:           ddbClient,
		KmsClient:           kmsClient,
	})
	if err != nil {
		return "", err
	}
	// 2. Create a branch key identifier with the AWS KMS Key configured in the KeyStore Configuration.
	branchKey, err := keyStore.CreateKey(context.Background(), keystoretypes.CreateKeyInput{})
	if err != nil {
		return "", err
	}
	return branchKey.BranchKeyIdentifier, nil
}
