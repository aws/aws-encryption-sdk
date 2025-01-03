// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example demonstrates how to set an algorithm suite while using the Raw AES Keyring
in the AWS Encryption SDK.

The algorithm suite used in the encrypt() method is the algorithm used to protect your
data using the data key. By setting this algorithm, you can configure the algorithm used
to encrypt and decrypt your data.

Algorithm suites can be set in a similar manner in other keyrings as well. However,
please make sure that you're using a logical algorithm suite that is compatible with your
keyring. For more information on algorithm suites supported by the AWS Encryption SDK, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html

The AES wrapping algorithm (AesWrappingAlg::AlgAes256GcmIv12Tag16) protects your data key using
the user-provided wrapping key. In contrast, the algorithm suite used in the encrypt() method
is the algorithm used to protect your data using the data key. This example demonstrates setting the
latter, which is the algorithm suite for protecting your data. When the commitment policy is
RequireEncryptRequireDecrypt, the default algorithm used in the encrypt method is
AlgAes256GcmHkdfSha512CommitKeyEcdsaP384, which is a committing and signing algorithm.
Signature verification ensures the integrity of a digital message as it goes across trust
boundaries. However, signature verification adds a significant performance cost to encryption
and decryption. If encryptors and decryptors are equally trusted, we can consider using an algorithm
suite that does not include signing. This example sets the algorithm suite as
AlgAes256GcmHkdfSha512CommitKey, which is a committing but non-signing algorithm.
For more information on digital signatures, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#digital-sigs

This example creates a Raw AES Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context and the algorithm suite AlgAes256GcmHkdfSha512CommitKey.
This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on how to use Raw AES keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-aes-keyring.html
*/

package examples

import (
	"context"
	"crypto/rand"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
)

func SetEncryptionAlgorithmSuiteExample(exampleText string) {
	// Step 1: Generate a 256-bit AES key to use with your keyring.
	// In practice, you should get this key from a secure key management system such as an HSM.
	key, err := generateAes256KeyBytes()
	if err != nil {
		panic(err)
	}
	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 3: Create the keyring
	// The key namespace and key name are defined by you
	// and are used by the raw AES keyring to determine
	// whether it should attempt to decrypt an encrypted data key.
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#use-raw-aes-keyring
	var keyNamespace = "A managed aes keys"
	var keyName = "My 256-bit AES wrapping key"
	// Note: The wrapping algorithm here is NOT the algorithm suite we set in this example.
	aesKeyRingInput := mpltypes.CreateRawAesKeyringInput{
		KeyName:      keyName,
		KeyNamespace: keyNamespace,
		WrappingKey:  key,
		WrappingAlg:  mpltypes.AesWrappingAlgAlgAes256GcmIv12Tag16,
	}
	aesKeyring, err := matProv.CreateRawAesKeyring(context.Background(), aesKeyRingInput)
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
	// Here, we customize the Algorithm Suite that is used to Encrypt the plaintext.
	// In particular, we use an Algorithm Suite without Signing.
	// Signature verification adds a significant performance cost on decryption.
	// If the users encrypting data and the users decrypting data are equally trusted,
	// consider using an algorithm suite that does not include signing.
	// See more about Digital Signatures:
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#digital-sigs
	algorithmSuiteId := mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmHkdfSha512CommitKey
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           aesKeyring,
		AlgorithmSuiteId:  &algorithmSuiteId,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 6b: Decrypt
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        res.Ciphertext,
		EncryptionContext: encryptionContext,
		Keyring:           aesKeyring,
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
	fmt.Println("Set Encryption Algorithm Suite Example Completed Successfully")
}

func generateAes256KeyBytes() ([]byte, error) {
	numOfBytes := 32 // 256 bits = 32 bytes
	key := make([]byte, numOfBytes)
	// Use crypto/rand for cryptographically secure random numbers
	_, err := rand.Read(key)
	if err != nil {
		return nil, err
	}
	return key, nil
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
