// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Public Key Discovery Raw ECDH Keyring.

A public key discovery Raw ECDH Keyring takes in the recipient's private key located
at eccPrivateKeyFileNameRecipient
as a UTF8 PEM-encoded (PKCS #8 PrivateKeyInfo structures) private key,
and the Curve Specification where the key lies.

If you provide the eccPrivateKeyFileNameRecipient, make sure to also
provide the recipient's public key located at eccPublicKeyFileNameRecipient
in the directory that you run this example. Even though the Public Key Discovery Raw ECDH keyring
uses the eccPrivateKeyFileNameRecipient to decrypt the data,
the eccPublicKeyFileNameRecipient is needed to generate the ciphertext to decrypt.

This example loads ECC keys from PEM files and the ciphertext with paths defined in
 - eccPrivateKeyFileNameRecipient
 - eccPublicKeyFileNameRecipient

If you do not provide these files, running this example through this
class' main method will generate three files required for all raw ECDH examples
eccPrivateKeyFilenameSender, eccPrivateKeyFileNameRecipient
and eccPublicKeyFileNameRecipient for you.
In practice, users of this library should not generate new key pairs
like this, and should instead retrieve an existing key from a secure
key management system (e.g. an HSM).
You may also provide your own key pair by placing PEM files in the
directory where the example is run or modifying the paths in the code
below. These files must be valid PEM encodings of the key pair as UTF-8
encoded bytes. If you do provide your own key pair, or if a key pair
already exists, this class' main method will not generate a new key pair.

This example creates a RawECDH keyring with the PublicKeyDiscovery key agreement scheme.
This scheme is only available on decrypt.

This example creates a Public Key Discovery Raw ECDH Keyring and takes in a ciphertext to decrypt it.
This example also includes some sanity checks for demonstration:
1. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-ecdh-keyring.html#raw-ecdh-PublicKeyDiscovery
*/

package ecdh

import (
	"context"
	"fmt"
	"os"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	primitivestypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/awscryptographyprimitivessmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/examples/utils"
)

func PublicKeyRawEcdhDiscoveryKeyringExample(
	exampleText string,
	ecdhCurveSpec primitivestypes.ECDHCurveSpec,
	eccPublicKeyFileNameRecipient string,
	eccPrivateKeyFileNameRecipient string) {
	// Step 1: Generate Raw ECDH ECC keys and load the recipient's private key.
	// You may provide your own ECC keys in the files returned by eccPublicKeyFileNameRecipient

	// If you do not provide these files, running this example through this
	// class' main method will generate three files required for all raw ECDH examples
	// eccPrivateKeyFileNameSender, eccPrivateKeyFileNameRecipient
	// and eccPublicKeyFileNameRecipient for you.
	if !utils.FileExists(eccPublicKeyFileNameRecipient) {
		err := utils.WriteRawEcdhEccKeys(ecdhCurveSpec)
		if err != nil {
			panic(err)
		}
	}
	privateKeyRecipient, err := os.ReadFile(eccPrivateKeyFileNameRecipient)
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
	// Step 5: Create the Public Key Discovery Raw ECDH keyring.
	// Create the keyring.
	// This keyring uses a discovery configuration. This configuration will check on decrypt
	// if it is meant to decrypt the message by checking if the configured public key is stored on the message.
	// The discovery configuration can only decrypt messages and CANNOT encrypt messages.
	discoveryRawEcdhStaticConfigurationInput := mpltypes.PublicKeyDiscoveryInput{
		RecipientStaticPrivateKey: privateKeyRecipient,
	}
	discoveryRawEcdhStaticConfiguration := &mpltypes.RawEcdhStaticConfigurationsMemberPublicKeyDiscovery{
		Value: discoveryRawEcdhStaticConfigurationInput,
	}
	discoveryRawEcdhKeyringInput := mpltypes.CreateRawEcdhKeyringInput{
		CurveSpec:          ecdhCurveSpec,
		KeyAgreementScheme: discoveryRawEcdhStaticConfiguration,
	}
	discoveryRawEcdhKeyring, err := matProv.CreateRawEcdhKeyring(context.Background(), discoveryRawEcdhKeyringInput)
	if err != nil {
		panic(err)
	}
	// Step 6a: Get the ciphertext
	// Although this example highlights Public Key Discovery Raw ECDH Keyring keyring, Discovery keyrings cannot
	// be used to encrypt, so for encryption we create a Ephemeral Raw ECDH keyring without discovery mode.
	cipherText := getCipherTextRawEcdh(matProv, encryptionClient, ecdhCurveSpec, exampleText, encryptionContext, eccPublicKeyFileNameRecipient)
	// Step 6b: Decrypt
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Keyring:           discoveryRawEcdhKeyring,
		EncryptionContext: encryptionContext,
		Ciphertext:        cipherText,
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
		fmt.Println("Public Key Discovery Raw ECDH Keyring Example Completed Successfully")
	} else {
		panic("FAILED!")
	}
}

// This function creates a Ephemeral Raw ECDH keyring and encrypt the exampleText
func getCipherTextRawEcdh(
	matProv *mpl.Client,
	encryptionClient *client.Client,
	ecdhCurveSpec primitivestypes.ECDHCurveSpec,
	exampleText string,
	encryptionContext map[string]string,
	eccPublicKeyFileNameRecipient string) []byte {
	// 1. Generate Raw ECDH ECC keys and load public key.
	// You may provide your own ECC keys in the files returned by eccPublicKeyFileNameRecipient

	// If you do not provide these files, running this example through this
	// class' main method will generate three files required for all raw ECDH examples
	// eccPrivateKeyFileNameSender, eccPrivateKeyFileNameRecipient
	// and eccPublicKeyFileNameRecipient for you.
	// Load public key from UTF-8 encoded PEM files into a DER encoded public key.
	if !utils.FileExists(eccPublicKeyFileNameRecipient) {
		err := utils.WriteRawEcdhEccKeys(ecdhCurveSpec)
		if err != nil {
			panic(err)
		}
	}
	publicKeyRecipient, err := utils.LoadPublicKeyFromPEM(eccPublicKeyFileNameRecipient)
	if err != nil {
		panic(err)
	}
	// Create the RawEcdhStaticConfigurations
	ephemeralRawEcdhStaticConfigurationInput := mpltypes.EphemeralPrivateKeyToStaticPublicKeyInput{
		RecipientPublicKey: publicKeyRecipient,
	}
	ephemeralRawECDHStaticConfiguration := mpltypes.RawEcdhStaticConfigurationsMemberEphemeralPrivateKeyToStaticPublicKey{
		Value: ephemeralRawEcdhStaticConfigurationInput,
	}
	// Create the Ephemeral Raw ECDH keyring.
	// This keyring uses an ephemeral configuration. This configuration will always create a new
	// key pair as the sender key pair for the key agreement operation. The ephemeral configuration can only
	// encrypt data and CANNOT decrypt messages.
	rawEcdhKeyRingInput := mpltypes.CreateRawEcdhKeyringInput{
		CurveSpec:          ecdhCurveSpec,
		KeyAgreementScheme: &ephemeralRawECDHStaticConfiguration,
	}
	ecdhKeyring, err := matProv.CreateRawEcdhKeyring(context.Background(), rawEcdhKeyRingInput)
	if err != nil {
		panic(err)
	}
	// Encrypt the data
	// A raw ecdh keyring with Ephemeral configuration cannot decrypt data since the key pair
	// used as the sender is ephemeral. This means that at decrypt time it does not have
	// the private key that corresponds to the public key that is stored on the message.
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           ecdhKeyring,
	})
	if err != nil {
		panic(err)
	}
	return res.Ciphertext
}
