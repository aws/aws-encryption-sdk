// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 This example sets up the Hierarchical Keyring, which establishes a key hierarchy where "branch"
 keys are persisted in DynamoDb. These branch keys are used to protect your data keys, and these
 branch keys are themselves protected by a KMS Key.

 Establishing a key hierarchy like this has two benefits:
 First, by caching the branch key material, and only calling KMS to re-establish authentication
 regularly according to your configured TTL, you limit how often you need to call KMS to protect
 your data. This is a performance security tradeoff, where your authentication, audit, and logging
 from KMS is no longer one-to-one with every encrypt or decrypt call. Additionally, KMS Cloudtrail
 cannot be used to distinguish Encrypt and Decrypt calls, and you cannot restrict who has
 Encryption rights from who has Decryption rights since they both ONLY need KMS:Decrypt. However,
 the benefit is that you no longer have to make a network call to KMS for every encrypt or
 decrypt.

 Second, this key hierarchy facilitates cryptographic isolation of a tenant's data in a
 multi-tenant data store. Each tenant can have a unique Branch Key, that is only used to protect
 the tenant's data. You can either statically configure a single branch key to ensure you are
 restricting access to a single tenant, or you can implement an interface that selects the Branch
 Key based on the Encryption Context.

 This example demonstrates configuring a Hierarchical Keyring with a Branch Key ID Supplier to
 encrypt and decrypt data for two separate tenants.

 This example requires access to the DDB Table where you are storing the Branch Keys. This
 table must be configured with the following primary key configuration: - Partition key is named
 "partition_key" with type (S) - Sort key is named "sort_key" with type (S).

 This example also requires using a KMS Key. You need the following access on this key:
 - GenerateDataKeyWithoutPlaintext
 - Decrypt

 For more information on how to use Hierarchical Keyrings, see
 https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-hierarchical-keyring.html
*/

package awskmshierarchicalkeyring

import (
	"context"
	"fmt"

	keystore "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygenerated"
	keystoretypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygeneratedtypes"
	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func AwsKmsHKeyExample(exampletext, keyStoreKMSKeyRegion, keyStoreRegion, keyStoreKMSKeyID, keyStoreName, logicalKeyStoreName string) {
	// Step 1: Create the aws sdk clients
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	// Step 1a: Create the aws kms client
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = keyStoreKMSKeyRegion
	})
	// Step 1b: Create the ddb client
	ddbClient := dynamodb.NewFromConfig(cfg, func(options *dynamodb.Options) {
		options.Region = keyStoreRegion
	})
	// Step 2: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	client, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Step 2: Create the keystore to manage the tenant keys
	// This SHOULD be the same configuration that you used
	// to initially create and populate your KeyStore.
	kmsConfig := keystoretypes.KMSConfigurationMemberkmsKeyArn{
		Value: keyStoreKMSKeyID,
	}
	keyStore, err := keystore.NewClient(keystoretypes.KeyStoreConfig{
		DdbTableName:        keyStoreName,
		KmsConfiguration:    &kmsConfig,
		LogicalKeyStoreName: logicalKeyStoreName,
		DdbClient:           ddbClient,
		KmsClient:           kmsClient,
	})
	if err != nil {
		panic(err)
	}
	// Step 3: Create two new branch keys
	branchKeyA, err := createbranchkeyid(keyStoreName, logicalKeyStoreName, keyStoreKMSKeyID, ddbClient, kmsClient)
	if err != nil {
		panic(err)
	}
	branchKeyB, err := createbranchkeyid(keyStoreName, logicalKeyStoreName, keyStoreKMSKeyID, ddbClient, kmsClient)
	if err != nil {
		panic(err)
	}
	// Step 4: Create a branch key supplier that maps the branch key id to a more readable format
	// See branchkeysupplier.go in this package for the branchKeySupplier structure
	keySupplier := branchKeySupplier{branchKeyA: branchKeyA, branchKeyB: branchKeyB}
	// Step 5: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 6: Create the Hierarchical Keyring.
	hkeyringInput := mpltypes.CreateAwsKmsHierarchicalKeyringInput{
		KeyStore:            keyStore,
		BranchKeyIdSupplier: &keySupplier,
		TtlSeconds:          600,
	}
	hKeyRing, err := matProv.CreateAwsKmsHierarchicalKeyring(context.Background(), hkeyringInput)
	if err != nil {
		panic(err)
	}
	// Step 7: Create encryption context for both tenants.
	// The Branch Key Id supplier uses the encryption context to determine which branch key id will
	// be used to encrypt data.
	// Remember that your encryption context is NOT SECRET.
	// For more information, see
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context

	// Create encryption context for TenantA
	encryptionContextA := map[string]string{
		"tenant":                    "TenantA",
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Create encryption context for TenantB
	encryptionContextB := map[string]string{
		"tenant":                    "TenantB",
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 8a: Encrypt the data
	// Encrypt data for Tenant A
	encryptOutputA, err := client.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampletext),
		EncryptionContext: encryptionContextA,
		Keyring:           hKeyRing,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(encryptOutputA.Ciphertext) == exampletext {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Encrypt data for Tenant B
	encryptOutputB, err := client.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampletext),
		EncryptionContext: encryptionContextB,
		Keyring:           hKeyRing,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(encryptOutputB.Ciphertext) == exampletext {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 8b: Decrypt the data with various scenerios for demonstration

	// For demonstration, let's attest that TenantKeyB cannot decrypt a message written by TenantKeyA,
	// and vice versa and construct more restrictive hierarchical keyrings.
	hkeyringInputA := mpltypes.CreateAwsKmsHierarchicalKeyringInput{
		KeyStore:    keyStore,
		BranchKeyId: &branchKeyA,
		TtlSeconds:  600,
	}
	hKeyRingA, err := matProv.CreateAwsKmsHierarchicalKeyring(context.Background(), hkeyringInputA)
	if err != nil {
		panic(err)
	}
	hkeyringInputB := mpltypes.CreateAwsKmsHierarchicalKeyringInput{
		KeyStore:    keyStore,
		BranchKeyId: &branchKeyB,
		TtlSeconds:  600,
	}
	hKeyRingB, err := matProv.CreateAwsKmsHierarchicalKeyring(context.Background(), hkeyringInputB)
	if err != nil {
		panic(err)
	}
	// Demonstrate that data encrypted by one tenant's key
	// cannot be decrypted with by a keyring specific to another tenant.

	// Keyring with tenant B's branch key cannot decrypt data encrypted with tenant A's branch key
	// This will fail and raise a AwsCryptographicMaterialProvidersException,
	// which we swallow ONLY for demonstration purposes.
	_, err = client.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        encryptOutputA.Ciphertext,
		EncryptionContext: encryptionContextA,
		Keyring:           hKeyRingB,
	})
	if err == nil {
		panic("Expected error did not occur")
	}
	switch err.(type) {
	case mpltypes.AwsCryptographicMaterialProvidersException:
		// You may choose how to handle the exception in this switch case.
	default:
		panic("error is expected to be a type of AwsCryptographicMaterialProvidersException")
	}
	// Keyring with tenant A's branch key cannot decrypt data encrypted with tenant B's branch key.
	// This will fail and raise a AwsCryptographicMaterialProvidersException,
	// which we swallow ONLY for demonstration purposes.
	_, err = client.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        encryptOutputB.Ciphertext,
		EncryptionContext: encryptionContextA,
		Keyring:           hKeyRingA,
	})
	if err == nil {
		panic("Expected error did not occur")
	}
	switch err.(type) {
	case mpltypes.AwsCryptographicMaterialProvidersException:
		// You may choose how to handle the exception in this switch case.
	default:
		panic("error is expected to be a type of AwsCryptographicMaterialProvidersException")
	}
	// Demonstrate that data encrypted by one tenant's branch key can be decrypted by that tenant,
	// and that the decrypted data matches the input data.

	// For tenant A
	decryptOutputA, err := client.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        encryptOutputA.Ciphertext,
		EncryptionContext: encryptionContextA,
		Keyring:           hKeyRingA,
	})
	if err != nil {
		panic(err)
	}
	// If you are not specifying the encryption context on decrypt. Its recommended to check if the encryption context matches.
	// Although, we are specifying the encryption context on decrypt, only for demonstration we are validating the encryption context.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContextA, decryptOutputA.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputA.Plaintext) != exampletext {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// For tenant B
	decryptOutputB, err := client.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        encryptOutputB.Ciphertext,
		EncryptionContext: encryptionContextB,
		Keyring:           hKeyRingB,
	})
	if err != nil {
		panic(err)
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
	// The encryption context was specified on decrypt; we are validating the encryption context for demonstration only.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContextB, decryptOutputB.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputB.Plaintext) != exampletext {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputA.Plaintext) != exampletext {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("Aws Kms Hierarchical Keyring Example Completed Successfully")
}

// This function only does subset matching because AWS Encryption SDK can add pairs, so don't require an exact match.
func validateEncryptionContext(expected, actual map[string]string) error {
	for expectedKey, expectedValue := range expected {
		actualValue, exists := actual[expectedKey]
		if !exists || actualValue != expectedValue {
			return fmt.Errorf("encryption context mismatch: expected key '%s' with value '%s'",
				expectedKey, expectedValue)
		}
	}
	return nil
}
