// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
/*
This example sets up the Raw RSA Keyring
The Raw RSA keyring performs asymmetric encryption and decryption of data keys in local memory
with RSA public and private keys that you provide.
This keyring accepts PEM encodings of the key pair as UTF-8 interpreted bytes.
The encryption function encrypts the data key under the RSA public key. The decryption function
decrypts the data key using the private key.
This example generate private and public key pairs.
In practice, users of this library should not generate new key pairs
like this, and should instead retrieve an existing key from a secure
key management system (e.g. an HSM).
You may also provide your own key pair by placing PEM files in the
directory where the example is run or modifying the paths in the code
below. These files must be valid PEM encodings of the key pair as UTF-8
encoded bytes. If you do provide your own key pair, or if a key pair
already exists, this class' main method will not generate a new key pair.
This example creates a Raw RSA Keyring and then encrypts a custom input exampleText
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.
A Raw RSA keyring that encrypts and decrypts must include an asymmetric public key and private
key pair. However, you can encrypt data with a Raw RSA keyring that has only a public key,
and you can decrypt data with a Raw RSA keyring that has only a private key. This example requires
the user to either provide both private and public keys, or not provide any keys and the example
generates both to test encryption and decryption. If you configure a Raw RSA keyring with a
public and private key, be sure that they are part of the same key pair. Some language
implementations of the AWS Encryption SDK will not construct a Raw RSA keyring with keys
from different pairs. Others rely on you to verify that your keys are from the same key pair.
You can include any Raw RSA keyring in a multi-keyring.
For more information on how to use Raw RSA keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-rsa-keyring.html
*/

package rawrsakeyring

import (
	"context"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
)

func RawRsaExample(exampleText string) {
	// Step 1: Generate the key-pairs
	publicKeyBlock, privateKeyBlock, err := generateKeyPair()
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
	// and are used by the raw RSA keyring to determine
	// whether it should attempt to decrypt an encrypted data key.
	//
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#use-raw-rsa-keyring
	keyNamespace := "Some managed raw keys"
	keyName := "My 2048-bit RSA wrapping key"
	rsaKeyRingInput := mpltypes.CreateRawRsaKeyringInput{
		KeyName:       keyName,
		KeyNamespace:  keyNamespace,
		PaddingScheme: mpltypes.PaddingSchemeOaepSha512Mgf1,
		PublicKey:     pem.EncodeToMemory(publicKeyBlock),
		PrivateKey:    pem.EncodeToMemory(privateKeyBlock),
	}
	rsaKeyring, err := matProv.CreateRawRsaKeyring(context.Background(), rsaKeyRingInput)
	if err != nil {
		panic(err)
	}
	// Step 4: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	cryptoClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
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
	res, err := cryptoClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           rsaKeyring,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 6b: Decrypt
	// You do not need to specify the encryption context on decrypt
	// because the header of the encrypted message includes the encryption context.
	decryptOutput, err := cryptoClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        res.Ciphertext,
		Keyring:           rsaKeyring,
		EncryptionContext: encryptionContext,
	})
	if err != nil {
		panic(err)
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
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
	fmt.Println("Raw RSA Keyring Example Completed Successfully")
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

func generateKeyPair() (*pem.Block, *pem.Block, error) {
	privateKey, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return nil, nil, err
	}
	// Extract public key from the private key
	publicKey := &privateKey.PublicKey
	// Encode public key to PKCS1 DER format
	publicKeyDER, err := x509.MarshalPKIXPublicKey(publicKey)
	if err != nil {
		return nil, nil, err
	}
	privateKeyDer, err := x509.MarshalPKCS8PrivateKey(privateKey)
	if err != nil {
		return nil, nil, err
	}
	// Encode to PEM format
	publicKeyBlock := &pem.Block{
		Type:  "RSA PUBLIC KEY",
		Bytes: publicKeyDER,
	}
	privateKeyBlock := &pem.Block{
		Type:  "PRIVATE KEY",
		Bytes: privateKeyDer,
	}
	return publicKeyBlock, privateKeyBlock, nil
}
