// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the AWS KMS Multi Keyring made up of multiple AWS KMS Keyrings.

A multi-keyring is a keyring that consists of one or more individual keyrings of the
same or a different type. The effect is like using several keyrings in a series.
When you use a multi-keyring to encrypt data, any of the wrapping keys in any of its
keyrings can decrypt that data.

When you create a multi-keyring to encrypt data, you designate one of the keyrings as
the generator keyring. All other keyrings are known as child keyrings. The generator keyring
generates and encrypts the plaintext data key. Then, all of the wrapping keys in all of the
child keyrings encrypt the same plaintext data key. The multi-keyring returns the plaintext
key and one encrypted data key for each wrapping key in the multi-keyring. If you create a
multi-keyring with no generator keyring, you can use it to decrypt data, but not to encrypt.
If the generator keyring is a KMS keyring, the generator key in the AWS KMS keyring generates
and encrypts the plaintext key. Then, all additional AWS KMS keys in the AWS KMS keyring,
and all wrapping keys in all child keyrings in the multi-keyring, encrypt the same plaintext key.

When decrypting, the AWS Encryption SDK uses the keyrings to try to decrypt one of the encrypted
data keys. The keyrings are called in the order that they are specified in the multi-keyring.
Processing stops as soon as any key in any keyring can decrypt an encrypted data key.

This example creates a Multi Keyring and then encrypts a custom input exampleText
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decryption of ciphertext is possible using the multi_keyring,
   and every one of the keyrings from the multi_keyring separately
3. All decrypted plaintext value match exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.

This example creates a multi_keyring using a KMS keyring as generator keyring and
another KMS keyring as a child keyring.

For more information on how to use Multi keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-multi-keyring.html

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

package awskmsmultikeyring

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

func AwsKmsMultiKeyringExample(exampleText, defaultKMSKeyId, alternateRegionKMSKeyId, alternateRegionKMSKeyRegion string) {
	// Step 1: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 2: Create an AwsKmsMultiKeyring that protects your data under two different KMS Keys.
	// Either KMS Key individually is capable of decrypting data encrypted under this Multi Keyring.
	generatorKeyId := defaultKMSKeyId
	awsKmsMultiKeyringInput := mpltypes.CreateAwsKmsMultiKeyringInput{
		Generator: &generatorKeyId,
		KmsKeyIds: []string{alternateRegionKMSKeyId},
	}
	awsKmsMultiKeyring, err := matProv.CreateAwsKmsMultiKeyring(context.Background(), awsKmsMultiKeyringInput)
	if err != nil {
		panic(err)
	}
	// Step 3: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	encryptionClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Step 4: Create your encryption context (Optional).
	// Remember that your encryption context is NOT SECRET.
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 5a: Encrypt
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMultiKeyring,
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
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMultiKeyring,
		Ciphertext:        res.Ciphertext,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// Demonstrate that a single AwsKmsKeyring configured with either KMS key
	// is also capable of decrypting the data.
	// Create the aws kms client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = alternateRegionKMSKeyRegion
	})
	// Create a single AwsKmsKeyring with the KMS key from our second region.
	awsKmsKeyringInput := mpltypes.CreateAwsKmsKeyringInput{
		KmsClient: kmsClient,
		KmsKeyId:  alternateRegionKMSKeyId,
	}
	awsKmsKeyring, err := matProv.CreateAwsKmsKeyring(context.Background(), awsKmsKeyringInput)
	if err != nil {
		panic(err)
	}
	decryptOutputKmsKeyring, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        res.Ciphertext,
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsKeyring,
	})
	if err != nil {
		panic(err)
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
	// The encryption context was specified on decrypt; we are validating the encryption context for demonstration only.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContext, decryptOutputKmsKeyring.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputKmsKeyring.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("KMS Multi Keyring Example Completed Successfully")
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
