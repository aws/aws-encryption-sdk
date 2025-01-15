// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 This example sets up an MRK multi-keyring and an MRK discovery
 multi-keyring using a custom client supplier.
 A custom client supplier grants users access to more granular
 configuration aspects of their authentication details and KMS
 client. In this example, we create a simple custom client supplier
 that authenticates with a different IAM role based on the
 region of the KMS key.

 This example creates a MRK multi-keyring configured with a custom
 client supplier using a single MRK and encrypts the example_data with it.
 Then, it creates a MRK discovery multi-keyring to decrypt the ciphertext.
*/

package clientsupplier

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
)

func ClientSupplierExample(exampleText, mrkKeyIdEncrypt, awsAccountId string, awsRegions []string) {
	// Step 1: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	encryptionClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Step 2: Create your encryption context.
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
	// Step 3: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 4: Create keyrings
	// First Keyring: Create the multi-keyring using our custom client supplier
	// defined in the RegionalRoleClientSupplier class in this directory.
	// Note: RegionalRoleClientSupplier will internally use the key_arn's region
	// to retrieve the correct IAM role.
	awsKmsMrkKeyringMultiInput := mpltypes.CreateAwsKmsMrkMultiKeyringInput{
		ClientSupplier: &RegionalRoleClientSupplier{},
		Generator:      &mrkKeyIdEncrypt,
	}
	awsKmsMrkMultiKeyring, err := matProv.CreateAwsKmsMrkMultiKeyring(context.Background(), awsKmsMrkKeyringMultiInput)
	if err != nil {
		panic(err)
	}
	// Second Keyring: Create a MRK discovery multi-keyring with a custom client supplier.
	// A discovery MRK multi-keyring will be composed of
	// multiple discovery MRK keyrings, one for each region.
	// Each component keyring has its own KMS client in a particular region.
	// When we provide a client supplier to the multi-keyring, all component
	// keyrings will use that client supplier configuration.
	// In our tests, we make `mrk_key_id_encrypt` an MRK with a replica, and
	// provide only the replica region in our discovery filter.
	discoveryFilter := mpltypes.DiscoveryFilter{
		AccountIds: []string{awsAccountId},
		Partition:  "aws",
	}
	awsKmsMrkDiscoveryMultiKeyringInput := mpltypes.CreateAwsKmsMrkDiscoveryMultiKeyringInput{
		ClientSupplier:  &RegionalRoleClientSupplier{},
		Regions:         awsRegions,
		DiscoveryFilter: &discoveryFilter,
	}
	awsKmsMrkDiscoveryMultiKeyring, err := matProv.CreateAwsKmsMrkDiscoveryMultiKeyring(context.Background(), awsKmsMrkDiscoveryMultiKeyringInput)
	// Step 5a: Encrypt
	res, err := encryptionClient.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampleText),
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMrkMultiKeyring,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(res.Ciphertext) == exampleText {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 5b: Decrypt
	// Decrypt your encrypted data using the discovery multi keyring.
	// On Decrypt, the header of the encrypted message (ciphertext) will be parsed.
	// The header contains the Encrypted Data Keys (EDKs), which, if the EDK
	// was encrypted by a KMS Keyring, includes the KMS Key ARN.
	// For each member of the Multi Keyring, every EDK will try to be decrypted until a decryption
	// is successful.
	// Since every member of the Multi Keyring is a Discovery Keyring:
	//   Each Keyring will filter the EDKs by the Discovery Filter and the Keyring's region.
	//   For each filtered EDK, the keyring will attempt decryption with the keyring's client.
	// All of this is done serially, until a success occurs or all keyrings have failed
	// all (filtered) EDKs. KMS MRK Discovery Keyrings will attempt to decrypt
	// Multi Region Keys (MRKs) and regular KMS Keys.
	decryptOutput, err := encryptionClient.Decrypt(context.Background(), esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           awsKmsMrkDiscoveryMultiKeyring,
		Ciphertext:        res.Ciphertext,
	})
	if err != nil {
		panic(err)
	}
	// Validate Plaintext after decryption and Plaintext before encryption ARE the same
	if string(decryptOutput.Plaintext) != exampleText {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
	// The encryption context was specified on decrypt; we are validating the encryption context for demonstration only.
	// Before your application uses plaintext data, verify that the encryption context that
	// you used to encrypt the message is included in the encryption context that was used to
	// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
	if err = validateEncryptionContext(encryptionContext, decryptOutput.EncryptionContext); err != nil {
		panic(err)
	}
	// Test the Missing Region Exception
	// (This is for demonstration; you do not need to do this in your code.)

	// Create a MRK discovery multi-keyring with a custom client supplier and a fake region.
	awsKmsMrkDiscoveryMultiKeyringInputMissingRegion := mpltypes.CreateAwsKmsMrkDiscoveryMultiKeyringInput{
		ClientSupplier:  &RegionalRoleClientSupplier{},
		Regions:         []string{"fake-region"},
		DiscoveryFilter: &discoveryFilter,
	}
	_, err = matProv.CreateAwsKmsMrkDiscoveryMultiKeyring(context.Background(), awsKmsMrkDiscoveryMultiKeyringInputMissingRegion)
	// Swallow the AwsCryptographicMaterialProvidersException but you may choose how to handle the exception
	switch err.(type) {
	case mpltypes.AwsCryptographicMaterialProvidersException:
		// You may choose how to handle the exception in this switch case.
	default:
		panic("Decryption using discovery keyring with missing region MUST raise AwsCryptographicMaterialProvidersException")
	}
	fmt.Println("Client Supplier Example completed successfully")
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
