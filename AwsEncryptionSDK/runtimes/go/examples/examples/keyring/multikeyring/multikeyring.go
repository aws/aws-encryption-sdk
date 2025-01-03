// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Multi Keyring

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

This example creates a multi_keyring using a KMS keyring as generator keyring and a raw AES keyring
as a child keyring. You can use different combinations of keyrings in the multi_keyring.

For more information on how to use Multi keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-multi-keyring.html

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

package multikeyring

import (
	"context"
	"crypto/rand"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func MultiKeyringExample(exampleText, defaultKMSKeyId, defaultKmsKeyRegion string) {
	// Step 1: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 2: Create the MultiKeyring that consists of the KMS Keyring as generator and Raw AES Keyring as child keyring
	// When using this MultiKeyring to encrypt data, either KMS Keyring or
	// Raw AES Keyring (or a MultiKeyring containing either) may be used to decrypt the data
	awsKmsKeyring := getKMSKeyring(defaultKMSKeyId, defaultKmsKeyRegion, matProv)
	rawAESKeyring := getRawAESKeyring(matProv)
	createMultiKeyringInput := mpltypes.CreateMultiKeyringInput{
		Generator:     awsKmsKeyring,
		ChildKeyrings: []mpltypes.IKeyring{rawAESKeyring},
	}
	multiKeyring, err := matProv.CreateMultiKeyring(context.Background(), createMultiKeyringInput)
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
	// Demonstrate that you can also successfully decrypt data using the `rawAESKeyring` directly.
	// Because you used a MultiKeyring on Encrypt, you can use either the `kmsKeyring` or
	// `rawAESKeyring` individually to decrypt the data.
	decryptOutputRawAES, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        res.Ciphertext,
		EncryptionContext: encryptionContext,
		Keyring:           rawAESKeyring,
	})
	if err != nil {
		panic(err)
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
	// The encryption context was specified on decrypt; we are validating the encryption context for demonstration only.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContext, decryptOutputRawAES.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputRawAES.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// Demonstrate that you can also successfully decrypt data using the `awsKmsKeyring` directly.
	decryptOutputAwsKms, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
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
	if err = validateEncryptionContext(encryptionContext, decryptOutputAwsKms.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputAwsKms.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("Multi Keyring Example Completed Successfully")
}
func getKMSKeyring(kmsKeyId string, kmsRegion string, matProv *mpl.Client) mpltypes.IKeyring {
	// 1. Create the aws kms client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = kmsRegion
	})
	// 2. Create Aws Kms keyring
	awsKmsKeyringInput := mpltypes.CreateAwsKmsKeyringInput{
		KmsClient: kmsClient,
		KmsKeyId:  kmsKeyId,
	}
	awsKmsKeyring, err := matProv.CreateAwsKmsKeyring(context.Background(), awsKmsKeyringInput)
	if err != nil {
		panic(err)
	}
	return awsKmsKeyring
}
func getRawAESKeyring(matProv *mpl.Client) mpltypes.IKeyring {
	// 1. Generate a 256-bit AES key to use with your keyring.
	// In practice, you should get this key from a secure key management system such as an HSM.
	key, err := generateAes256KeyBytes()
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
func generateAes256KeyBytes() ([]byte, error) {
	const keySize = 32 // 256 bits = 32 bytes
	key := make([]byte, keySize)
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
