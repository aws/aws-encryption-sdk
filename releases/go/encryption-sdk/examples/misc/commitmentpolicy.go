// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example configures a client with a specific commitment policy for the
AWS Encryption SDK client, then encrypts and decrypts data using an AWS KMS Keyring.

The commitment policy in this example (ForbidEncryptAllowDecrypt) should only be
used as part of a migration from version 1.x to 2.x, or for advanced users with
specialized requirements. Most AWS Encryption SDK users should use the default
commitment policy (RequireEncryptRequireDecrypt).

This example creates a KMS Keyring and then encrypts a custom input exampleText
with an encryption context for the commitment policy ForbidEncryptAllowDecrypt.
This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on setting your commitment policy, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#commitment-policy

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

package misc

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func CommitmentPolicyExample(exampleText, defaultKMSKeyId, defaultKmsKeyRegion string) {
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
	// Build the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.

	// Create one with the commitment policy RequireEncryptAllowDecrypt and another with ForbidEncryptAllowDecrypt.
	// Read more about commitment policies here: https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#commitment-policy
	commitPolicyRequireEncryptRequireDecrypt := mpltypes.ESDKCommitmentPolicyRequireEncryptRequireDecrypt
	commitPolicyForbidEncryptAllowDecrypt := mpltypes.ESDKCommitmentPolicyForbidEncryptAllowDecrypt
	forbidEncryptClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{CommitmentPolicy: &commitPolicyForbidEncryptAllowDecrypt})
	if err != nil {
		panic(err)
	}
	requireEncryptClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{CommitmentPolicy: &commitPolicyRequireEncryptRequireDecrypt})
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
	// Make sure you use a non-committing algorithm with the commitment policy ForbidEncryptAllowDecrypt.
	// Otherwise encrypt() will throw
	// Error: AwsCryptographicMaterialProvidersError
	//   {
	//     error: InvalidAlgorithmSuiteInfoOnEncrypt
	//     {
	//       message: "Configuration conflict. Commitment policy requires only non-committing algorithm suites"
	//     }
	//   }
	// By default for ForbidEncryptAllowDecrypt, the algorithm used is
	// AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 which is a non-committing algorithm.
	res, err := forbidEncryptClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsKeyring,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 6b: Decrypt
	decryptOutput, err := forbidEncryptClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsKeyring,
		Ciphertext:        res.Ciphertext,
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
	// Demonstrate that an EncryptionSDK that enforces Key Commitment on Decryption
	// will fail to decrypt the encrypted message (as it was encrypted without Key Commitment).
	_, err = requireEncryptClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsKeyring,
		Ciphertext:        res.Ciphertext,
	})
	// We expect this to fail
	if err == nil {
		panic("Expected error but error is nil")
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("Set Commitment Policy Example Completed Successfully")
}
