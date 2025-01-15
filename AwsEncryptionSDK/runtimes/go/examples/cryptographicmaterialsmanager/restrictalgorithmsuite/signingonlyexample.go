// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 Demonstrate an encrypt/decrypt cycle using a Custom Cryptographic Materials Manager (CMM).
 `signingsuiteonlycmm.go` demonstrates creating a custom CMM to reject Non-Signing Algorithms.
*/

package restrictalgorithmsuite

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func SigningOnlyExample(exampleText, defaultKMSKeyId, defaultKmsKeyRegion string) {
	// Step 1: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	encryptionClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Step 2: Create the AWS KMS client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = defaultKmsKeyRegion
	})
	// Step 3: Create your encryption context.
	// Remember that your encryption context is NOT SECRET.
	// For more information, see
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 3: Create the Aws KMS Keyring
	awsKmsKeyringInput := mpltypes.CreateAwsKmsKeyringInput{
		KmsClient: kmsClient,
		KmsKeyId:  defaultKMSKeyId,
	}
	awsKmsKeyring, err := matProv.CreateAwsKmsKeyring(context.Background(), awsKmsKeyringInput)
	if err != nil {
		panic(err)
	}
	// Step 4: Create an instance of the custom CMM
	cmm, err := NewSigningSuiteOnlyCMM(awsKmsKeyring)
	if err != nil {
		panic(err)
	}
	// Step 5a: Encrypt
	algorithmSuiteId := mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmHkdfSha512CommitKeyEcdsaP384
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		MaterialsManager:  cmm,
		AlgorithmSuiteId:  &algorithmSuiteId,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 5b: Decrypt
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        res.Ciphertext,
		EncryptionContext: encryptionContext,
		MaterialsManager:  cmm,
	})
	if err != nil {
		panic(err)
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
	// The encryption context was specified on decrypt; we are validating the encryption context for demonstration only.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContext, decryptOutput.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// Demonstrate that a Non Signing Algorithm Suite will be rejected by the CMM.
	nonSigningAlgorithmSuiteId := mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmHkdfSha512CommitKey
	_, err = encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		MaterialsManager:  cmm,
		AlgorithmSuiteId:  &nonSigningAlgorithmSuiteId,
	})
	if err == nil {
		panic("Expected error but error is nil")
	}
	switch err.(type) {
	case mpltypes.AwsCryptographicMaterialProvidersException:
		// You may choose how to handle the exception in this switch case.
	default:
		panic("error is expected to be a type of AwsCryptographicMaterialProvidersException")
	}
	fmt.Println("SigningSuiteOnlyCMM Example Completed Successfully")
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
