// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
/*
This example sets up the AWS KMS MRK (multi-region key) Keyring
The AWS Key Management Service (AWS KMS) MRK keyring interacts with AWS KMS to
create, encrypt, and decrypt data keys with multi-region AWS KMS keys (MRKs).
This example creates a KMS MRK Keyring and then encrypts a custom input exampleText
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.
AWS KMS MRK keyrings can be used independently or in a multi-keyring with other keyrings
of the same or a different type.
For more information on how to use KMS keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html
For more info on KMS MRK (multi-region keys), see the KMS documentation:
https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html
For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/
package awskmsmrkkeyring

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

func AwsKmsMrkKeyringExample(exampleText, defaultRegionMrkKeyArn, alternateRegionMrkKeyArn, defaultMRKKeyRegion, alternateRegionMrkKeyRegion string) {
	// Step 1: Create the aws kms client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClientEncrypt := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = defaultMRKKeyRegion
	})
	kmsClientDecrypt := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = alternateRegionMrkKeyRegion
	})
	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 3: Create the keyrings
	// Create one keyring for encrypt with KMS client on defaultMRKKeyRegion region
	// Create second keyring for decrypt with KMS client on alternateRegionMrkKeyRegion region.
	// In order to illustrate the MRK behavior, we are creating two keyrings with two different regions
	awsKmsMrkKeyringInputEncrypt := mpltypes.CreateAwsKmsMrkKeyringInput{
		KmsClient: kmsClientEncrypt,
		KmsKeyId:  defaultRegionMrkKeyArn,
	}
	awsKmsMrkKeyringInputDecrypt := mpltypes.CreateAwsKmsMrkKeyringInput{
		KmsClient: kmsClientDecrypt,
		KmsKeyId:  alternateRegionMrkKeyArn,
	}
	awsKmsMrkKeyringEncrypt, err := matProv.CreateAwsKmsMrkKeyring(context.Background(), awsKmsMrkKeyringInputEncrypt)
	if err != nil {
		panic(err)
	}
	awsKmsMrkKeyringDecrypt, err := matProv.CreateAwsKmsMrkKeyring(context.Background(), awsKmsMrkKeyringInputDecrypt)
	if err != nil {
		panic(err)
	}
	// Step 4: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	encryptionClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Step 5: Create your encryption context (Optional).
	// Remember that your encryption context is NOT SECRET.
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 6a: Encrypt
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMrkKeyringEncrypt,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 6b: Decrypt
	// 1. Decrypt with the same keyring (same region) as encrypt
	decryptOutputSameRegion, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMrkKeyringEncrypt,
		Ciphertext:        res.Ciphertext,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputSameRegion.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// 2. Decrypt with different keyring on different region.
	decryptOutputDifferentRegion, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMrkKeyringDecrypt,
		Ciphertext:        res.Ciphertext,
	})
	if err != nil {
		panic(err)
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
	// The encryption context was specified on decrypt; we are validating the encryption context for demonstration only.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContext, decryptOutputDifferentRegion.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputDifferentRegion.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("AWS KMS MRK Keyring Example Completed Successfully")
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
