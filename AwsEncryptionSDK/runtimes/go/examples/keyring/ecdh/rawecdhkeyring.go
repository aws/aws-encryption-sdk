// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Raw ECDH Keyring.

This example takes in the sender's private key located at
eccPrivateKeyFileNameSender as a UTF8 PEM-encoded
(PKCS #8 PrivateKeyInfo structures) private key,
and the recipient's public key located at
eccPublicKeyFileNameRecipient as a
UTF8 PEM-encoded X.509 public key,
also known as SubjectPublicKeyInfo (SPKI),
and the Curve Specification where the keys lie.

This example loads ECC keys from PEM files with paths defined in
 - eccPrivateKeyFileNameSender
 - eccPublicKeyFileNameRecipient

If you do not provide these files, running this example through this
class' main method will generate three files required for all raw ECDH examples
eccPrivateKeyFileNameSender, eccPrivateKeyFileNameRecipient
and eccPublicKeyFileNameRecipient for you.
These files will be generated in the directory where the example is run.
In practice, users of this library should not generate new key pairs
like this, and should instead retrieve an existing key from a secure
key management system (e.g. an HSM).
You may also provide your own key pair by placing PEM files in the
directory where the example is run or modifying the paths in the code
below. These files must be valid PEM encodings of the key pair as UTF-8
encoded bytes. If you do provide your own key pair, or if a key pair
already exists, this class' main method will not generate a new key pair.

This example creates a RawECDH keyring with the RawPrivateKeyToStaticPublicKey key agreement scheme.
On encrypt, the shared secret is derived from the sender's private key and the recipient's public key.
On decrypt, the shared secret is derived from the sender's private key and the recipient's public key;
however, on decrypt the recipient can construct a keyring such that the shared secret is calculated with
the recipient's private key and the sender's public key. In both scenarios the shared secret will be the same.

This example creates a Raw ECDH Keyring and then encrypts a custom input exampleText
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-ecdh-keyring.html#raw-ecdh-RawPrivateKeyToStaticPublicKey
*/

package ecdh

import (
	"context"
	"fmt"
	"os"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	primitivestypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/awscryptographyprimitivessmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/examples/utils"
)

func RawECDHKeyringExample(
	exampleText string,
	ecdhCurveSpec primitivestypes.ECDHCurveSpec,
	eccPublicKeyFileNameRecipient string,
	eccPrivateKeyFileNameSender string) {
	// Step 1: Generate Raw ECDH ECC keys and load public key.
	// You may provide your own ECC keys in the files returned by eccPublicKeyFileNameRecipient

	// If you do not provide these files, running this example through this
	// class' main method will generate three files required for all raw ECDH examples
	// eccPrivateKeyFileNameSender, eccPrivateKeyFileNameRecipient
	// and eccPublicKeyFileNameRecipient for you.
	if !utils.FileExists(eccPublicKeyFileNameRecipient) || !utils.FileExists(eccPrivateKeyFileNameSender) {
		err := utils.WriteRawEcdhEccKeys(ecdhCurveSpec)
		if err != nil {
			panic(err)
		}
	}
	privateKeySender, err := os.ReadFile(eccPrivateKeyFileNameSender)
	if err != nil {
		panic(err)
	}
	publicKeyRecipient, err := utils.LoadPublicKeyFromPEM(eccPublicKeyFileNameRecipient)
	if err != nil {
		panic(err)
	}

	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
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
	// For more information, see
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 5:  Create the Raw ECDH keyring.
	// This keyring uses static sender and recipient keys. This configuration calls for both of
	// the keys to be on the same curve (P256 / P384 / P521).
	// On encrypt, the shared secret is derived from the sender's private key and the recipient's public key.
	// For this example, on decrypt, the shared secret is derived from the sender's private key and the recipient's public key;
	// However, on decrypt, the recipient can construct a keyring such that the shared secret is calculated with
	// the recipient's private key and the sender's public key. In both scenarios the shared secret will be the same.
	RawEcdhStaticConfigurationInput := mpltypes.RawPrivateKeyToStaticPublicKeyInput{
		SenderStaticPrivateKey: privateKeySender,
		RecipientPublicKey:     publicKeyRecipient,
	}
	RawECDHStaticConfiguration := &mpltypes.RawEcdhStaticConfigurationsMemberRawPrivateKeyToStaticPublicKey{
		Value: RawEcdhStaticConfigurationInput,
	}
	rawEcdhKeyRingInput := mpltypes.CreateRawEcdhKeyringInput{
		CurveSpec:          ecdhCurveSpec,
		KeyAgreementScheme: RawECDHStaticConfiguration,
	}
	rawEcdhKeyring, err := matProv.CreateRawEcdhKeyring(context.Background(), rawEcdhKeyRingInput)
	if err != nil {
		panic(err)
	}
	// Step 6a: Encrypt
	// A raw ecdh keyring with Ephemeral configuration cannot decrypt data since the key pair
	// used as the sender is ephemeral. This means that at decrypt time it does not have
	// the private key that corresponds to the public key that is stored on the message.
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           rawEcdhKeyring,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	// (This is an example for demonstration; you do not need to do this in your own code.)
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 6b: Decrypt
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        res.Ciphertext,
		EncryptionContext: encryptionContext,
		Keyring:           rawEcdhKeyring,
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
	if string(decryptOutput.Plaintext) == exampleText {
		fmt.Println("Raw ECDH Keyring Example Completed Successfully")
	} else {
		panic("FAILED!")
	}
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
