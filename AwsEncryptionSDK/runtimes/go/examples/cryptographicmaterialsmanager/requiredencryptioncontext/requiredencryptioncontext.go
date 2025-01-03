// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
Demonstrate an encrypt/decrypt cycle using a Required Encryption Context CMM.
A required encryption context CMM asks for required keys in the encryption context field
on encrypt such that they will not be stored on the message, but WILL be included in the header signature.
On decrypt, the client MUST supply the key/value pair(s) that were not stored to successfully decrypt the message.
*/

package requiredencryptioncontext

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func RequiredEncryptionContextExample(exampleText, defaultKMSKeyId, defaultKmsKeyRegion string) {
	// Step 1: Create the aws kms client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = defaultKmsKeyRegion
	})
	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 3: Create the keyring
	awsKmsKeyringInput := mpltypes.CreateAwsKmsKeyringInput{
		KmsClient: kmsClient,
		KmsKeyId:  defaultKMSKeyId,
	}
	awsKmsKeyring, err := matProv.CreateAwsKmsKeyring(context.Background(), awsKmsKeyringInput)
	if err != nil {
		panic(err)
	}
	// Step 4: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	encryptionClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Step 5: Create your encryption context.
	// Remember that your encryption context is NOT SECRET.
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
		"requiredKey1":              "requiredValue1",
		"requiredKey2":              "requiredValue2",
	}
	// Step 6: Create your required encryption context keys.
	// These keys MUST be in your encryption context.
	// These keys and their corresponding values WILL NOT be stored on the message but will be used
	// for authentication.
	underlyingCMM, err := matProv.CreateDefaultCryptographicMaterialsManager(context.Background(), mpltypes.CreateDefaultCryptographicMaterialsManagerInput{Keyring: awsKmsKeyring})
	if err != nil {
		panic(err)
	}
	requiredEncryptionContextKeys := []string{}
	requiredEncryptionContextKeys = append(requiredEncryptionContextKeys, "requiredKey1", "requiredKey2")
	requiredEncryptionContextInput := mpltypes.CreateRequiredEncryptionContextCMMInput{
		UnderlyingCMM: underlyingCMM,
		// If you pass in a keyring but no underlying cmm, it will result in a failure because only cmm is supported.
		RequiredEncryptionContextKeys: requiredEncryptionContextKeys,
	}
	requiredEC, err := matProv.CreateRequiredEncryptionContextCMM(context.Background(), requiredEncryptionContextInput)
	if err != nil {
		panic(err)
	}
	// Step 7a: Encrypt
	// NOTE: the keys "requiredKey1", and "requiredKey2"
	// WILL NOT be stored in the message header, but "encryption", "is not",
	// "but adds", "that can help you", and "the data you are handling" WILL be stored.
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		MaterialsManager:  requiredEC,
		EncryptionContext: encryptionContext,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 7b: Decrypt
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Ciphertext:        res.Ciphertext,
		MaterialsManager:  requiredEC,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// For demonstration attempt to decrypt your encrypted data using the same cryptographic material manager
	// you used on encrypt, but we won't pass the encryption context we DID NOT store on the message.
	// This will fail
	_, err = encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:       res.Ciphertext,
		MaterialsManager: requiredEC,
	})
	// We expect failure.
	if err == nil {
		panic("Decryption passed without any error when encryption context was not provided.")
	}
	// Decrypt your encrypted data using the same cryptographic material manager
	// you used to encrypt, but supply encryption context that contains ONLY the encryption context that
	// was NOT stored. This will pass.
	reproducedEncryptionContext := map[string]string{
		"requiredKey1": "requiredValue1",
		"requiredKey2": "requiredValue2",
	}
	decryptOutputreproducedEC, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: reproducedEncryptionContext,
		Ciphertext:        res.Ciphertext,
		MaterialsManager:  requiredEC,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputreproducedEC.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// You can also decrypt with the underlyingCMM, but must still provide the reproducedEncryptionContext.
	decryptOutputWithCMM, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: reproducedEncryptionContext,
		Ciphertext:        res.Ciphertext,
		MaterialsManager:  underlyingCMM,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutputWithCMM.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("Required Encryption Context CMM Example Completed Successfully")
}
