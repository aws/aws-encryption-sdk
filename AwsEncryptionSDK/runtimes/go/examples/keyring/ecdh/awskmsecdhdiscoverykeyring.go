// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the KMS ECDH Discovery Keyring.

This example takes in the recipient's KMS ECC key ARN.
This example attempts to decrypt a ciphertext using the kmsEcdhKeyIdP256RecipientKeyId,
it does so by checking if the message header contains the recipient's public key.

This example also requires access to a KMS ECC key.
Our tests provide a KMS ECC Key ARN that anyone can use, but you
can also provide your own KMS ECC key.
To use your own KMS ECC key, you must have:
    - kms:GetPublicKey permissions on that key.
This example will call kms:GetPublicKey on keyring creation.
You must also have kms:DeriveSharedSecret permissions on the KMS ECC key.

This example creates a KMS ECDH Discovery Keyring and then decrypts a ciphertext.
For getting the ciphertext, we create a KMS ECDH keyring without discovery
because kms_ecdh_discovery_keyring cannot encrypt data.
This example also includes some sanity checks for demonstration:
1. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-discovery
*/

package ecdh

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	primitivestypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/awscryptographyprimitivessmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/examples/utils"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func AwsKmsEcdhDiscoveryKeyringExample(
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
	// Step 5: Create the KMS ECDH keyring.
	// This keyring uses the KmsPublicKeyDiscovery configuration.
	// On encrypt, the keyring will fail as it is not allowed to encrypt data under this configuration.
	// On decrypt, the keyring will check if its corresponding public key is stored in the message header. It
	// will call AWS KMS to derive the shared from the recipient's KMS ECC Key ARN and the sender's public key;
	// For more information on this configuration see:
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-discovery
	// This keyring takes in:
	//  - kmsClient
	//  - recipientKmsIdentifier: Must be an ARN representing a KMS ECC key meant for KeyAgreement
	//  - curveSpec: The curve name where the public keys lie
	kmsEcdhDiscoveryStaticConfigurationInput := mpltypes.KmsPublicKeyDiscoveryInput{
		RecipientKmsIdentifier: kmsEcdhKeyIdP256RecipientKeyId,
	}
	kmsEcdhDiscoveryStaticConfiguration := &mpltypes.KmsEcdhStaticConfigurationsMemberKmsPublicKeyDiscovery{
		Value: kmsEcdhDiscoveryStaticConfigurationInput,
	}
	awsKmsEcdhDiscoveryKeyringInput := mpltypes.CreateAwsKmsEcdhKeyringInput{
		CurveSpec:          ecdhCurveSpec,
		KeyAgreementScheme: kmsEcdhDiscoveryStaticConfiguration,
		KmsClient:          kmsClient,
	}
	awsKmsEcdhDiscoveryKeyring, err := matProv.CreateAwsKmsEcdhKeyring(context.Background(), awsKmsEcdhDiscoveryKeyringInput)
	if err != nil {
		panic(err)
	}
	// Step 6: Get ciphertext by creating a KMS ECDH keyring WITHOUT discovery
	// because the KMS ECDH keyring WITH discovery CANNOT encrypt data.
	// We are generating a message intended for the kmsEcdhKeyIdP256RecipientKeyId recipient.
	// Since a KMS ECDH keyring WITHOUT discovery cannot encrypt data, this example will ONLY decrypt
	// messages where the configured key on the Discovery keyring is present on the message ciphertext.
	// In this example we call `kms:GetPublicKey` to get the public key associated with the
	// kmsEcdhKeyIdP256RecipientKeyId KMS key ID.
	// If the message contains this public key, message decryption will be attempted.
	cipherText := getCipherTextKmsEcdh(matProv, encryptionClient, ecdhCurveSpec, exampleText, encryptionContext, kmsClient, kmsEcdhKeyIdP256RecipientKeyId, kmsEcdhKeyIdP256SenderKeyId, kmsEccPublicKeyFileNameSender, kmsEccPublicKeyFileNameRecipient)

	// Step 7: Decrypt your encrypted data using the keyring with discovery behavior we created in step 5.
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Keyring:           awsKmsEcdhDiscoveryKeyring,
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
		fmt.Println("AWS KMS ECDH Discovery Keyring Example Completed Successfully")
	} else {
		panic("FAILED!")
	}
}

// This function creates a AWS KMS ECDH keyring and encrypt the exampleText
func getCipherTextKmsEcdh(
	matProv *mpl.Client,
	encryptionClient *client.Client,
	ecdhCurveSpec primitivestypes.ECDHCurveSpec,
	exampleText string,
	encryptionContext map[string]string,
	kmsClient *kms.Client,
	kmsEcdhKeyIdP256RecipientKeyId string,
	kmsEcdhKeyIdP256SenderKeyId string,
	kmsEccPublicKeyFileNameSender string,
	kmsEccPublicKeyFileNameRecipient string) []byte {
	// 1. Create the public key files for sender and recipient
	// You may provide your own ECC keys.
	// If not, this class will call the KMS ECC key, retrieve its public key, and store it
	// in a PEM file for example use.
	// Sender ECC key used in this example is retrieved with kmsEcdhKeyIdP256SenderKeyId
	// Recipent ECC key used in this example is retrieved with kmsEcdhKeyIdP256RecipientKeyId
	if !utils.FileExists(kmsEccPublicKeyFileNameSender) {
		err := utils.WriteKmsEcdhEccPublicKey(kmsEcdhKeyIdP256SenderKeyId, kmsEccPublicKeyFileNameSender, kmsClient)
		if err != nil {
			panic(err)
		}
	}
	if !utils.FileExists(kmsEccPublicKeyFileNameRecipient) {
		err := utils.WriteKmsEcdhEccPublicKey(kmsEcdhKeyIdP256RecipientKeyId, kmsEccPublicKeyFileNameRecipient, kmsClient)
		if err != nil {
			panic(err)
		}
	}
	// 2. Load public key from UTF-8 encoded PEM files into a DER encoded public key.
	publicKeySender, err := utils.LoadPublicKeyFromPEM(kmsEccPublicKeyFileNameSender)
	if err != nil {
		panic(err)
	}
	publicKeyRecipient, err := utils.LoadPublicKeyFromPEM(kmsEccPublicKeyFileNameRecipient)
	if err != nil {
		panic(err)
	}
	// 3. Create the KmsPrivateKeyToStaticPublicKeyInput and kmsEcdhStaticConfiguration
	kmsEcdhStaticConfigurationInput := mpltypes.KmsPrivateKeyToStaticPublicKeyInput{
		RecipientPublicKey:  publicKeyRecipient,
		SenderKmsIdentifier: kmsEcdhKeyIdP256SenderKeyId,
		SenderPublicKey:     publicKeySender,
	}
	kmsEcdhStaticConfiguration := &mpltypes.KmsEcdhStaticConfigurationsMemberKmsPrivateKeyToStaticPublicKey{
		Value: kmsEcdhStaticConfigurationInput,
	}
	// 4. Create the KMS ECDH keyring.
	awsKmsEcdhKeyringInput := mpltypes.CreateAwsKmsEcdhKeyringInput{
		CurveSpec:          ecdhCurveSpec,
		KeyAgreementScheme: kmsEcdhStaticConfiguration,
		KmsClient:          kmsClient,
	}
	awsKmsEcdhKeyring, err := matProv.CreateAwsKmsEcdhKeyring(context.Background(), awsKmsEcdhKeyringInput)
	if err != nil {
		panic(err)
	}
	// 5. Encrypt the data with the encryption_context
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsEcdhKeyring,
	})
	if err != nil {
		panic(err)
	}
	// 6. Return the ciphertext
	return res.Ciphertext
}
