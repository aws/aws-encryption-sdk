// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package awskmshierarchicalkeyring

import (
	"context"
	"fmt"

	keystore "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygenerated"
	keystoretypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

/*
This example demonstrates configuring a KeyStore and then
uses a helper method to version a branch key.
*/
func versionBranchKeyId(keyStoreTableName, logicalKeyStoreName, kmsKeyArn, branchKeyId string) error {
	// Load the AWS SDK configuration
	cfg, err := config.LoadDefaultConfig(context.Background())
	if err != nil {
		return err
	}
	// Create DDB and KMS clients
	ddbClient := dynamodb.NewFromConfig(cfg)
	kmsClient := kms.NewFromConfig(cfg)
	// Create the keystore
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
		return err
	}
	// To version a branch key you MUST have access to kms:ReEncrypt* and kms:GenerateDataKeyWithoutPlaintext
	_, err = keyStore.VersionKey(context.Background(), keystoretypes.VersionKeyInput{
		BranchKeyIdentifier: branchKeyId,
	})
	if err != nil {
		return err
	}
	return nil
}

// Function to test versionBranchKeyId in main.go in examples directory
func CreateAndVersionBranchKeyId(keyStoreKMSKeyRegion, keyStoreRegion, keyStoreKMSKeyID, keyStoreName, logicalKeyStoreName string) error {
	// Create the aws sdk clients
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	// Create the aws kms client
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = keyStoreKMSKeyRegion
	})
	// Create the ddb client
	ddbClient := dynamodb.NewFromConfig(cfg, func(options *dynamodb.Options) {
		options.Region = keyStoreRegion
	})
	// create branch key ID
	branchKeyId, err := createbranchkeyid(
		keyStoreName,
		logicalKeyStoreName,
		keyStoreKMSKeyID,
		ddbClient,
		kmsClient,
	)
	if err != nil {
		panic(err)
	}
	// Version Branch Key
	err = versionBranchKeyId(
		keyStoreName,
		logicalKeyStoreName,
		keyStoreKMSKeyID,
		branchKeyId,
	)
	if err != nil {
		panic(err)
	}
	fmt.Println("Create and version branchKey Id Example Completed Successfully")
	return nil
}
