// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
/*
This example sets up the AWS KMS MRK (multi-region key) Discovery Keyring
The AWS KMS discovery keyring is an AWS KMS keyring that doesn't specify any wrapping keys.
When decrypting, an MRK discovery keyring allows the AWS Encryption SDK to ask AWS KMS to decrypt
any encrypted data key by using the AWS KMS MRK that encrypted it, regardless of who owns or
has access to that AWS KMS key. The call succeeds only when the caller has kms:Decrypt
permission on the AWS KMS MRK.
The AWS Encryption SDK provides a standard AWS KMS discovery keyring and a discovery keyring
for AWS KMS multi-Region keys. Because it doesn't specify any wrapping keys, a discovery keyring
can't encrypt data. If you use a discovery keyring to encrypt data, alone or in a multi-keyring,
the encrypt operation fails.
The AWS Key Management Service (AWS KMS) MRK keyring interacts with AWS KMS to
create, encrypt, and decrypt data keys with multi-region AWS KMS keys (MRKs).
This example creates a KMS MRK Keyring and then encrypts a custom input exampleText
with an encryption context. This encrypted ciphertext is then decrypted using an
MRK Discovery keyring. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.
For information about using multi-Region keys with the AWS Encryption SDK, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/configure.html#config-mrks
For more info on KMS MRKs (multi-region keys), see the KMS documentation:
https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html
For more information on how to use KMS Discovery keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html#kms-keyring-discovery
For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/
package awskmsmrkdiscoverykeyring

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

func AwsKmsMrkDiscoveryKeyringExample(exampleText, defaultRegionMrkKeyArn, defaultMRKKeyRegion, alternateRegionMrkKeyRegion, defaultKMSKeyAccountID string) {
	// Step 1: Create the aws kms client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClientEncrypt := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = defaultMRKKeyRegion
	})
	kmsClientDecrypt := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = alternateRegionMrkKeyRegion
	})
	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 3: Create the keyring
	// Though this example highlights Discovery keyrings, Discovery keyrings
	// cannot be used to encrypt, so for encryption we create a KMS MRK keyring.
	// So, we create two keyrings. One for encryption, second one for decryption
	// First Keyring: Create KMS MRK Keyring used for encryption
	awsKmsMrkKeyringInputEncrypt := mpltypes.CreateAwsKmsMrkKeyringInput{
		KmsClient: kmsClientEncrypt,
		KmsKeyId:  defaultRegionMrkKeyArn,
	}
	awsKmsMrkKeyringEncrypt, err := matProv.CreateAwsKmsMrkKeyring(context.Background(), awsKmsMrkKeyringInputEncrypt)
	if err != nil {
		panic(err)
	}
	// Second Keyring: create a Discovery keyring to use for decryption.
	discoveryFilter := mpltypes.DiscoveryFilter{
		AccountIds: []string{defaultKMSKeyAccountID},
		Partition:  "aws",
	}
	// In order to illustrate the MRK behavior of this keyring, we configure
	// the keyring to use the second KMS region where the MRK is replicated to.
	// This example assumes you have already replicated your key, but since we
	// are using a discovery keyring, we don't need to provide the mrk replica key id
	awsKmsMrkDiscoveryInput := mpltypes.CreateAwsKmsMrkDiscoveryKeyringInput{
		KmsClient:       kmsClientDecrypt,
		Region:          alternateRegionMrkKeyRegion,
		DiscoveryFilter: &discoveryFilter,
	}
	awsKmsMrkDiscoveryKeyring, err := matProv.CreateAwsKmsMrkDiscoveryKeyring(context.Background(), awsKmsMrkDiscoveryInput)
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
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 6a: Encrypt
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMrkKeyringEncrypt,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption ARE the same")
	}
	// Step 6b: Decrypt
	// Create a Discovery keyring to use for decryption.
	// On Decrypt, the header of the encrypted message (ciphertext) will be parsed.
	// The header contains the Encrypted Data Keys (EDKs), which, if the EDK
	// was encrypted by a KMS Keyring, includes the KMS Key ARN.
	// The Discovery Keyring filters these EDKs for
	// EDKs encrypted by Single Region OR Multi Region KMS Keys.
	// If a Discovery Filter is present, these KMS Keys must belong
	// to an AWS Account ID in the discovery filter's AccountIds and
	// must be from the discovery filter's partition.
	// Finally, KMS is called to decrypt each filtered EDK until an EDK is
	// successfully decrypted. The resulting data key is used to decrypt the
	// ciphertext's message.
	// If all calls to KMS fail, the decryption fails.
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMrkDiscoveryKeyring,
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
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("AWS KMS MRK Discovery Keyring Example Completed Successfully")
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
