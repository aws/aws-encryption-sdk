// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
Demonstrate limiting the number of Encrypted Data Keys [EDKs] allowed
when encrypting or decrypting a message.
Limiting encrypted data keys is most valuable when you are decrypting
messages from an untrusted source.
By default, the ESDK will allow up to 65,535 encrypted data keys.
A malicious actor might construct an encrypted message with thousands of
encrypted data keys, none of which can be decrypted.
As a result, the AWS Encryption SDK would attempt to decrypt each
encrypted data key until it exhausted the encrypted data keys in the message.

For more information on limiting EDKs, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/configure.html#config-limit-keys
*/

package misc

import (
	"context"
	"crypto/rand"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
)

func LimitEncryptedDataKeyExample(exampleText, defaultKMSKeyId, defaultKmsKeyRegion string, maxEncryptedDataKeys int64) {
	// Step 1: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 2: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	// Also, set the EncryptionSDK's MaxEncryptedDataKeys parameter here
	encryptionClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{
		MaxEncryptedDataKeys: &maxEncryptedDataKeys,
	})
	if err != nil {
		panic(err)
	}
	// Step 3: Generate `maxEncryptedDataKeys` AES keyrings to use with your keyring.
	// In practice, you should get this key from a secure key management system such as an HSM.
	rawAESKeyrings := make([]mpltypes.IKeyring, 0, maxEncryptedDataKeys)
	var i int64 = 0
	for i < maxEncryptedDataKeys {
		rawAESKeyrings = append(rawAESKeyrings, getRawAESKeyring(matProv))
		i++
	}
	// Step 4: Create a Multi Keyring with `maxEncryptedDataKeys` AES Keyrings
	createMultiKeyringInput := mpltypes.CreateMultiKeyringInput{
		Generator:     rawAESKeyrings[0],
		ChildKeyrings: rawAESKeyrings[1:],
	}
	multiKeyring, err := matProv.CreateMultiKeyring(context.Background(), createMultiKeyringInput)
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
		Keyring:           multiKeyring,
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
		Keyring:           multiKeyring,
		Ciphertext:        res.Ciphertext,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
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
	// Demonstrate that an EncryptionSDK with a lower MaxEncryptedDataKeys
	// will fail to decrypt the encrypted message.
	// (This is an example for demonstration; you do not need to do this in your own code.)
	lowerMaxEncryptedDataKeys := maxEncryptedDataKeys - 1
	encryptionClientIncorrectMaxEncryptedKeys, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{
		MaxEncryptedDataKeys: &lowerMaxEncryptedDataKeys,
	})
	if err != nil {
		panic(err)
	}
	_, err = encryptionClientIncorrectMaxEncryptedKeys.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           multiKeyring,
		Ciphertext:        res.Ciphertext,
	})
	if err == nil {
		panic("Expected error not found.")
	}
	// Swallow the AwsCryptographicMaterialProvidersException but you may choose how to handle the exception
	switch err.(type) {
	case esdktypes.AwsEncryptionSdkException:
		// You may choose how to handle the exception in this switch case.
	default:
		panic("Decryption using lower then max encrypted data keys MUST raise AwsEncryptionSdkException")
	}
	fmt.Println("Limit Encrypted Data Key Example completed successfully")
}

func getRawAESKeyring(matProv *mpl.Client) mpltypes.IKeyring {
	// 1. Generate a 256-bit AES key to use with your keyring.
	// In practice, you should get this key from a secure key management system such as an HSM.
	key, err := generate256KeyBytesAES()
	if err != nil {
		panic(err)
	}
	// The key namespace and key name are defined by you
	// and are used by the raw AES keyring to determine
	// whether it should attempt to decrypt an encrypted data key.
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#use-raw-aes-keyring
	var keyNamespace = "A managed aes keys"
	var keyName = "My 256-bit AES wrapping key"
	// 2. Create the keyring
	aesKeyRingInput := mpltypes.CreateRawAesKeyringInput{
		KeyName:      keyName,
		KeyNamespace: keyNamespace,
		WrappingKey:  key,
		WrappingAlg:  mpltypes.AesWrappingAlgAlgAes256GcmIv12Tag16,
	}
	aesKeyring, err := matProv.CreateRawAesKeyring(context.Background(), aesKeyRingInput)
	return aesKeyring
}

func generate256KeyBytesAES() ([]byte, error) {
	const keySize = 32 // 256 bits = 32 bytes
	key := make([]byte, keySize)
	// Use crypto/rand for cryptographically secure random numbers
	_, err := rand.Read(key)
	if err != nil {
		return nil, err
	}
	return key, nil
}
