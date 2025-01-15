// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Ephemeral Raw ECDH Keyring.

This example takes in the recipient's public key located at
eccPublicKeyFileNameRecipient as a
UTF8 PEM-encoded X.509 public key,
and the Curve Specification where the key lies.

This example loads ECC keys from PEM files with paths defined in
 - eccPublicKeyFileNameRecipient

If you do not provide these files, running this example through this
class' main method will generate three files required for all raw ECDH examples
eccPrivateKeyFileNameSender, eccPrivateKeyFileNameRecipient
and eccPublicKeyFileNameRecipient for you.
In practice, users of this library should not generate new key pairs
like this, and should instead retrieve an existing key from a secure
key management system (e.g. an HSM).
You may also provide your own key pair by placing PEM files in the
directory where the example is run or modifying the paths in the code
below. These files must be valid PEM encodings of the key pair as UTF-8
encoded bytes. If you do provide your own key pair, or if a key pair
already exists, this class' main method will not generate a new key pair.

This examples creates a RawECDH keyring with the EphemeralPrivateKeyToStaticPublicKey key agreement scheme.
This configuration will always create a new key pair as the sender key pair for the key agreement operation.
The ephemeral configuration can only encrypt data and CANNOT decrypt messages.

This example creates an Ephemeral Raw ECDH Keyring and then encrypts a custom input exampleText
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-ecdh-keyring.html#raw-ecdh-EphemeralPrivateKeyToStaticPublicKey
*/

package ecdh

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	primitivestypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/awscryptographyprimitivessmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/examples/utils"
)

func EphemeralRawECDHKeyringExample(
	exampleText string,
	ecdhCurveSpec primitivestypes.ECDHCurveSpec,
	eccPublicKeyFileNameRecipient string) {
	// Step 1: Generate Raw ECDH ECC keys and load public key.
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
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 5: Create the keyring.
	// This keyring uses an ephemeral configuration. This configuration will always create a new
	// key pair as the sender key pair for the key agreement operation. The ephemeral configuration can only
	// encrypt data and CANNOT decrypt messages.
	ephemeralRawEcdhStaticConfigurationInput := mpltypes.EphemeralPrivateKeyToStaticPublicKeyInput{
		RecipientPublicKey: publicKeyRecipient,
	}
	ephemeralRawECDHStaticConfiguration :=
		mpltypes.RawEcdhStaticConfigurationsMemberEphemeralPrivateKeyToStaticPublicKey{
			Value: ephemeralRawEcdhStaticConfigurationInput,
		}
	rawEcdhKeyRingInput := mpltypes.CreateRawEcdhKeyringInput{
		CurveSpec:          ecdhCurveSpec,
		KeyAgreementScheme: &ephemeralRawECDHStaticConfiguration,
	}
	ecdhKeyring, err := matProv.CreateRawEcdhKeyring(context.Background(), rawEcdhKeyRingInput)
	if err != nil {
		panic(err)
	}
	// Step 6: Encrypt
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
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	// (This is an example for demonstration; you do not need to do this in your own code.)
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	fmt.Println("Ephemeral Raw ECDH Keyring Example Completed Successfully")
}
