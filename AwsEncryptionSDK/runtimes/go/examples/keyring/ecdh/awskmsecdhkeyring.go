// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the KMS ECDH Keyring.

This example takes in the sender's KMS ECC key ARN, the sender's public key,
the recipient's public key, and the algorithm definition where the ECC keys lie.

Both public keys MUST be UTF8 PEM-encoded X.509 public key,
also known as SubjectPublicKeyInfo (SPKI),

This keyring, depending on its KeyAgreement scheme,
takes in the sender's KMS ECC Key ARN, and the recipient's ECC Public Key
to derive a shared secret.
The keyring uses the shared secret to derive a data key to protect the
data keys that encrypt and decrypt exampletext.

This example also requires access to a KMS ECC key.
Our tests provide a KMS ECC Key ARN that you need permissions to, but you
can also provide your own KMS ECC key.
To use your own KMS ECC key, you must have either:
- Its public key downloaded in a UTF-8 encoded PEM file
- kms:GetPublicKey permissions on that key.
If you do not have the public key downloaded, running this example
through its main method will download the public key for you
by calling kms:GetPublicKey.
You must also have kms:DeriveSharedSecret permissions on the KMS ECC key.
This example also requires a recipient ECC Public Key that lies on the same
curve as the sender public key. This examples uses another distinct
KMS ECC Public Key, it does not have to be a KMS key; it can be a
valid SubjectPublicKeyInfo (SPKI) Public Key.

This example creates a KMS ECDH Keyring and then encrypts a custom input exampleText
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-create
*/

package ecdh

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	primitivestypes "github.com/aws/aws-cryptographic-material-providers-library/primitives/awscryptographyprimitivessmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-encryption-sdk/examples/utils"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func AwsKmsEcdhKeyringExample(
	exampleText string,
	ecdhCurveSpec primitivestypes.ECDHCurveSpec,
	kmsEcdhKeyIdP256RecipientKeyId string,
	kmsEcdhKeyIdP256SenderKeyId string,
	kmsEccPublicKeyFileNameSender string,
	kmsEccPublicKeyFileNameRecipient string) {
	// Step 1: Create the aws kms client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = "us-west-2"
	})
	// Step 2: Load public key from UTF-8 encoded PEM files into a DER encoded public key.
	// You may provide your own ECC keys.
	// If not, this class will call the KMS ECC key, retrieve its public key, and store it
	// in a PEM file for example use.
	if !utils.FileExists(kmsEccPublicKeyFileNameSender) {
		err = utils.WriteKmsEcdhEccPublicKey(kmsEcdhKeyIdP256SenderKeyId, kmsEccPublicKeyFileNameSender, kmsClient)
		if err != nil {
			panic(err)
		}
	}
	if !utils.FileExists(kmsEccPublicKeyFileNameRecipient) {
		err = utils.WriteKmsEcdhEccPublicKey(kmsEcdhKeyIdP256RecipientKeyId, kmsEccPublicKeyFileNameRecipient, kmsClient)
		if err != nil {
			panic(err)
		}
	}
	publicKeySender, err := utils.LoadPublicKeyFromPEM(kmsEccPublicKeyFileNameSender)
	if err != nil {
		panic(err)
	}
	publicKeyRecipient, err := utils.LoadPublicKeyFromPEM(kmsEccPublicKeyFileNameRecipient)
	if err != nil {
		panic(err)
	}
	// Step 3: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
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
	// For more information, see
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 6: Create the KMS ECDH keyring.
	// This keyring uses the KmsPrivateKeyToStaticPublicKey configuration. This configuration calls for both of
	// the keys to be on the same curve (P256, P384, P521).
	// On encrypt, the keyring calls AWS KMS to derive the shared secret from the sender's KMS ECC Key ARN and the recipient's public key.
	// For this example, on decrypt, the keyring calls AWS KMS to derive the shared secret from the sender's KMS ECC Key ARN and the recipient's public key;
	// however, on decrypt, the recipient can construct a keyring such that the shared secret is calculated with
	// the recipient's private key and the sender's public key. In both scenarios the shared secret will be the same.
	// For more information on this configuration see:
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-create
	// This keyring takes in:
	//  - kmsClient
	//  - kmsKeyId: Must be an ARN representing a KMS ECC key meant for KeyAgreement
	//  - curveSpec: The curve name where the public keys lie
	//  - senderPublicKey: A ByteBuffer of a UTF-8 encoded public
	//               key for the key passed into kmsKeyId in DER format
	//  - recipientPublicKey: A ByteBuffer of a UTF-8 encoded public
	//               key for the key passed into kmsKeyId in DER format
	kmsEcdhStaticConfigurationInput := mpltypes.KmsPrivateKeyToStaticPublicKeyInput{
		RecipientPublicKey:  publicKeyRecipient,
		SenderKmsIdentifier: kmsEcdhKeyIdP256SenderKeyId,
		SenderPublicKey:     publicKeySender,
	}
	kmsEcdhStaticConfiguration := &mpltypes.KmsEcdhStaticConfigurationsMemberKmsPrivateKeyToStaticPublicKey{
		Value: kmsEcdhStaticConfigurationInput,
	}
	awsKmsEcdhKeyringInput := mpltypes.CreateAwsKmsEcdhKeyringInput{
		CurveSpec:          ecdhCurveSpec,
		KeyAgreementScheme: kmsEcdhStaticConfiguration,
		KmsClient:          kmsClient,
	}
	awsKmsEcdhKeyring, err := matProv.CreateAwsKmsEcdhKeyring(context.Background(), awsKmsEcdhKeyringInput)
	if err != nil {
		panic(err)
	}
	// Step 7a: Encrypt the data
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsEcdhKeyring,
	})
	if err != nil {
		panic(err)
	}
	// Step 7b: Decrypt the data
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        res.Ciphertext,
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsEcdhKeyring,
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
		fmt.Println("AWS KMS ECDH Keyring Example Completed Successfully")
	} else {
		panic("FAILED!")
	}
}
