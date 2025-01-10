// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the AWS KMS Keyring in an multithreaded environment.
The AWS KMS keyring uses symmetric encryption KMS keys to generate, encrypt and
decrypt data keys. This example creates a KMS Keyring and then encrypts a custom input exampleText
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches exampleText
These sanity checks are for demonstration in the example only. You do not need these in your code.
AWS KMS keyrings can be used independently or in a multi-keyring with other keyrings
of the same or a different type.
For more information on how to use KMS keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html
For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

package multithreading

import (
	"context"
	"fmt"
	"runtime"
	"sync"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

// Structure to hold operation results
type OperationResult struct {
	EncryptOutput *esdktypes.EncryptOutput
	DecryptOutput *esdktypes.DecryptOutput
	Error         error
}

// Function to handle encryption
func encryptData(
	ctx context.Context,
	encryptionClient *client.Client,
	plaintext string,
	encryptionContext map[string]string,
	keyring mpltypes.IKeyring) OperationResult {
	res, err := encryptionClient.Encrypt(ctx, esdktypes.EncryptInput{
		Plaintext:         []byte(plaintext),
		EncryptionContext: encryptionContext,
		Keyring:           keyring,
	})
	return OperationResult{
		EncryptOutput: res,
		Error:         err,
	}
}

// Function to handle decryption
func decryptData(
	ctx context.Context,
	encryptionClient *client.Client,
	ciphertext []byte,
	encryptionContext map[string]string,
	keyring mpltypes.IKeyring) OperationResult {
	res, err := encryptionClient.Decrypt(ctx, esdktypes.DecryptInput{
		EncryptionContext: encryptionContext,
		Keyring:           keyring,
		Ciphertext:        ciphertext,
	})
	return OperationResult{
		DecryptOutput: res,
		Error:         err,
	}
}

func processEncryptionWorker(
	wg *sync.WaitGroup,
	jobs <-chan string,
	encryptionClient *client.Client,
	awsKmsKeyring mpltypes.IKeyring,
	encryptionContext map[string]string,
) {
	defer wg.Done()
	for plaintext := range jobs {
		ctx := context.Background()
		// Perform encryption
		encryptResult := encryptData(ctx, encryptionClient, plaintext, encryptionContext, awsKmsKeyring)
		if encryptResult.Error != nil {
			panic(encryptResult.Error)
		}
		// Verify ciphertext is different from plaintext
		if string(encryptResult.EncryptOutput.Ciphertext) == plaintext {
			panic("Ciphertext and Plaintext before encryption are the same")
		}
		// Perform decryption
		decryptResult := decryptData(
			ctx,
			encryptionClient,
			encryptResult.EncryptOutput.Ciphertext,
			encryptionContext,
			awsKmsKeyring,
		)
		if decryptResult.Error != nil {
			panic(decryptResult.Error)
		}
		// If you do not specify the encryption context on Decrypt, it's recommended to check if the resulting encryption context matches.
		// The encryption context was specified on decrypt; we are validating the encryption context for demonstration only.
		// Before your application uses plaintext data, verify that the encryption context that
		// you used to encrypt the message is included in the encryption context that was used to
		// decrypt the message. The AWS Encryption SDK can add pairs, so don't require an exact match.
		if err := validateEncryptionContext(encryptionContext, decryptResult.DecryptOutput.EncryptionContext); err != nil {
			panic(err)
		}
		if string(decryptResult.DecryptOutput.Plaintext) != plaintext {
			panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
		}
	}
}

func AWSKMSMultiThreadTest(texts []string, defaultKmsKeyID, defaultKmsKeyRegion string) {
	// Create the AWS KMS client
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic(err)
	}
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = defaultKmsKeyRegion
	})
	// Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Create the keyring
	awsKmsKeyringInput := mpltypes.CreateAwsKmsKeyringInput{
		KmsClient: kmsClient,
		KmsKeyId:  defaultKmsKeyID,
	}
	awsKmsKeyring, err := matProv.CreateAwsKmsKeyring(context.Background(), awsKmsKeyringInput)
	if err != nil {
		panic(err)
	}
	// Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	encryptionClient, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Create your encryption context (Optional).
	// Remember that your encryption context is NOT SECRET.
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
	encryptionContext := map[string]string{
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Create buffered channels to handle multiple operations
	numWorkers := runtime.NumCPU() - 1 // Leave one CPU free for system tasks

	// Create a wait group to track all goroutines
	var wg sync.WaitGroup

	// Create a channel to send a plaintext
	jobs := make(chan string, len(texts))

	// Start worker pool
	for range numWorkers {
		wg.Add(1)
		go processEncryptionWorker(&wg, jobs, encryptionClient, awsKmsKeyring, encryptionContext)
	}

	// Send jobs to workers
	for _, text := range texts {
		jobs <- text
	}
	close(jobs)
	// Wait for all workers to complete
	wg.Wait()
	fmt.Println("AWS KMS Keyring example in multithreaded environment completed successfully.")
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
