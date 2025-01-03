// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
/*
This example sets up the AWS KMS Discovery Keyring
AWS KMS discovery keyring is an AWS KMS keyring that doesn't specify any wrapping keys.
The AWS Encryption SDK provides a standard AWS KMS discovery keyring and a discovery keyring
for AWS KMS multi-Region keys. For information about using multi-Region keys with the
AWS Encryption SDK, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/configure.html#config-mrks
Because it doesn't specify any wrapping keys, a discovery keyring can't encrypt data.
If you use a discovery keyring to encrypt data, alone or in a multi-keyring, the encrypt
operation fails.
When decrypting, a discovery keyring allows the AWS Encryption SDK to ask AWS KMS to decrypt
any encrypted data key by using the AWS KMS key that encrypted it, regardless of who owns or
has access to that AWS KMS key. The call succeeds only when the caller has kms:Decrypt
permission on the AWS KMS key.
This example creates a KMS Keyring and then encrypts a custom input exampleText
with an encryption context. This encrypted ciphertext is then decrypted using the Discovery keyring.
This example also includes some sanity checks for demonstration:
 1. Ciphertext and plaintext data are not the same
 2. Decrypted plaintext value matches exampleText
 3. Decryption is only possible if the Discovery Keyring contains the correct AWS Account ID's to
	which the KMS key used for encryption belongs
These sanity checks are for demonstration in the example only. You do not need these in your code.
For more information on how to use KMS Discovery keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html#kms-keyring-discovery
For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

package awskmsdiscoverykeyring

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

func AwsKmsDiscoveryKeyringExample(exampleText string, defaultKmsKeyId string, defaultKMSKeyAccountID string) {
	// Step 1: Create the aws kms client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = "us-west-2"
	})
	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(
		mpltypes.MaterialProvidersConfig{},
	)
	if err != nil {
		panic(err)
	}
	// Step 3: Create the keyring
	// Although this example highlights Discovery keyrings, Discovery keyrings cannot
	// be used to encrypt, so for encryption we create a KMS keyring without discovery mode.
	// So, we create two keyrings, one for encrypt and another for decrypt
	// First Keyring: Create a AwsKmsKeyring to use for encryption
	awsKmsKeyringInput := mpltypes.CreateAwsKmsKeyringInput{
		KmsClient: kmsClient,
		KmsKeyId:  defaultKmsKeyId,
	}
	awsKmsKeyring, err := matProv.CreateAwsKmsKeyring(context.Background(), awsKmsKeyringInput)
	if err != nil {
		panic(err)
	}
	// Second Keyring: Create a Discovery keyring to use for decryption.
	// We'll add a discovery filter so that we limit
	// the set of ciphertexts we are willing to decrypt to only ones created by KMS keys in our account and
	// partition.
	discoveryFilter := mpltypes.DiscoveryFilter{
		AccountIds: []string{defaultKMSKeyAccountID},
		Partition:  "aws",
	}
	awsKmsDiscoveryKeyringInput := mpltypes.CreateAwsKmsDiscoveryKeyringInput{
		KmsClient:       kmsClient,
		DiscoveryFilter: &discoveryFilter,
	}
	awsKmsDiscoveryKeyring, err := matProv.CreateAwsKmsDiscoveryKeyring(context.Background(), awsKmsDiscoveryKeyringInput)
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
	algorithmSuiteID := mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmHkdfSha512CommitKey
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		AlgorithmSuiteId:  &algorithmSuiteID,
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
		Keyring:    awsKmsDiscoveryKeyring,
		Ciphertext: res.Ciphertext,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContext, decryptOutput.EncryptionContext); err != nil {
		panic(err)
	}
	// Validate that if a different discovery keyring doesn't have the correct
	// AWS Account ID's, the decrypt will fail with an error message
	// Note that this assumes Account ID used here ('888888888888') is different than the one used
	// during encryption
	discoveryFilterFailureCase := mpltypes.DiscoveryFilter{
		AccountIds: []string{"888888888888"},
		Partition:  "aws",
	}
	awsKmsDiscoveryKeyringInputFailureCase := mpltypes.CreateAwsKmsDiscoveryKeyringInput{
		KmsClient:       kmsClient,
		DiscoveryFilter: &discoveryFilterFailureCase,
	}
	awsKmsDiscoveryKeyringFailureCase, err := matProv.CreateAwsKmsDiscoveryKeyring(context.Background(), awsKmsDiscoveryKeyringInputFailureCase)
	_, err = encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		Keyring:    awsKmsDiscoveryKeyringFailureCase,
		Ciphertext: res.Ciphertext,
	})
	// We expected error in failure case
	if err == nil {
		panic("Expected failure case to fail")
	}
	fmt.Println("AWS KMS Discovery Keyring Example Completed Successfully")
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
