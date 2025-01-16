// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 This example demonstrates how to use a shared cache across multiple Hierarchical Keyrings.
 With this functionality, users only need to maintain one common shared cache across multiple
 Hierarchical Keyrings with different Key Stores instances/KMS Clients/KMS Keys.

 If you want to use a Shared Cache, you need to initialize it only once, and
 pass the same cache `shared_cache` to different hierarchical keyrings.

 There are three important parameters that users need to carefully set while providing the shared cache:

 1. Partition ID - Partition ID is an optional parameter provided to the Hierarchical Keyring input,
 which distinguishes Cryptographic Material Providers (i.e: Keyrings) writing to a cache.
 - If the Partition ID is set and is the same for two Hierarchical Keyrings (or another Material Provider),
   they CAN share the same cache entries in the cache.
 - If the Partition ID is set and is different for two Hierarchical Keyrings (or another Material Provider),
   they CANNOT share the same cache entries in the cache.
 - If the Partition ID is not set by the user, it is initialized as a random 16-byte UUID which makes
   it unique for every Hierarchical Keyring, and two Hierarchical Keyrings (or another Material Provider)
   CANNOT share the same cache entries in the cache.

 2. Logical Key Store Name - This parameter is set by the user when configuring the Key Store for
 the Hierarchical Keyring. This is a logical name for the branch key store.
 Suppose you have a physical Key Store (K). You create two instances of K (K1 and K2). Now, you create
 two Hierarchical Keyrings (HK1 and HK2) with these Key Store instances (K1 and K2 respectively).
 - If you want to share cache entries across these two keyrings, you should set the Logical Key Store Names
   for both the Key Store instances (K1 and K2) to be the same.
 - If you set the Logical Key Store Names for K1 and K2 to be different, HK1 (which uses Key Store instance K1)
   and HK2 (which uses Key Store instance K2) will NOT be able to share cache entries.

 3. Branch Key ID - Choose an effective Branch Key ID Schema

 This is demonstrated in the example below.
 Notice that both K1 and K2 are instances of the same physical Key Store (K).
 You MUST NEVER have two different physical Key Stores with the same Logical Key Store Name.

 Important Note: If you have two or more Hierarchy Keyrings with:
 - Same Partition ID
 - Same Logical Key Store Name of the Key Store for the Hierarchical Keyring
 - Same Branch Key ID
 then they WILL share the cache entries in the Shared Cache.
 Please make sure that you set all of Partition ID, Logical Key Store Name and Branch Key ID
 to be the same for two Hierarchical Keyrings if and only if you want them to share cache entries.

 This example first creates a shared cache that you can use across multiple Hierarchical Keyrings.
 The example then configures a Hierarchical Keyring (HK1 and HK2) with the shared cache,
 a Branch Key ID and two instances (K1 and K2) of the same physical Key Store (K) respectively,
 i.e. HK1 with K1 and HK2 with K2. The example demonstrates that if you set the same Partition ID
 for HK1 and HK2, the two keyrings can share cache entries.
 If you set different Partition ID of the Hierarchical Keyrings, or different
 Logical Key Store Names of the Key Store instances, then the keyrings will NOT
 be able to share cache entries.

 This example requires access to the DDB Table (K) where you are storing the Branch Keys. This
 table must be configured with the following primary key configuration: - Partition key is named
 "partition_key" with type (S) - Sort key is named "sort_key" with type (S)

 This example also requires using a KMS Key. You need the following access on this key:
 - GenerateDataKeyWithoutPlaintext
 - Decrypt
*/

package awskmshierarchicalkeyring

import (
	"context"
	"fmt"

	keystore "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygenerated"
	keystoretypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygeneratedtypes"
	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	client "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

func SharedCacheExample(exampletext, keyStoreKMSKeyRegion, keyStoreRegion, keyStoreKMSKeyID, keyStoreName, logicalKeyStoreName string) {
	// Step 1: Create the aws sdk clients
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		fmt.Println(err)
		panic(err)
	}
	// Step 1a: Create the aws kms client
	kmsClient := kms.NewFromConfig(cfg, func(o *kms.Options) {
		o.Region = keyStoreKMSKeyRegion
	})
	// Step 1b: Create the ddb client
	ddbClient := dynamodb.NewFromConfig(cfg, func(options *dynamodb.Options) {
		options.Region = keyStoreRegion
	})
	// Step 2: Initialize the mpl client
	matProv, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Step 3: Create the CryptographicMaterialsCache (CMC) to share across multiple Hierarchical Keyrings
	// using the Material Providers Library
	// This CMC takes in:
	// - CacheType
	cache := mpltypes.CacheTypeMemberDefault{
		Value: mpltypes.DefaultCache{
			EntryCapacity: 100,
		},
	}
	cmcCacheInput := mpltypes.CreateCryptographicMaterialsCacheInput{
		Cache: &cache,
	}
	sharedCryptographicMaterialsCache, err := matProv.CreateCryptographicMaterialsCache(context.Background(), cmcCacheInput)
	if err != nil {
		panic(err)
	}
	// Step 4:  Create a CacheType object for the sharedCryptographicMaterialsCache
	// Note that the `cache` parameter in the Hierarchical Keyring Input takes a `CacheType` as input
	// Here, we pass a `Shared` CacheType that passes an already initialized shared cache.

	// If you want to use a Shared Cache, you need to initialize it only once, and
	// pass the same cache `shared_cache` to different hierarchical keyrings.

	// CryptographicMaterialsCacheRef is an Rc (Reference Counted), so if you clone it to
	// pass it to different Hierarchical Keyrings, it will still point to the same
	// underlying cache, and increment the reference count accordingly.
	shared_cache := mpltypes.CacheTypeMemberShared{sharedCryptographicMaterialsCache}
	// Step 2: Instantiate the encryption SDK client.
	// This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
	// which enforces that this client only encrypts using committing algorithm suites and enforces
	// that this client will only decrypt encrypted messages that were created with a committing
	// algorithm suite.
	client, err := client.NewClient(esdktypes.AwsEncryptionSdkConfig{})
	if err != nil {
		panic(err)
	}
	// Step 5: Configure your Key Store resource keyStore1.
	// This SHOULD be the same configuration that you used
	// to initially create and populate your physical Key Store.
	// Note that key_store_table_name is the physical Key Store,
	// and key_store1 is instances of this physical Key Store.
	kmsConfig := keystoretypes.KMSConfigurationMemberkmsKeyArn{
		Value: keyStoreKMSKeyID,
	}
	keyStore1, err := keystore.NewClient(keystoretypes.KeyStoreConfig{
		DdbTableName:        keyStoreName,
		KmsConfiguration:    &kmsConfig,
		LogicalKeyStoreName: logicalKeyStoreName,
		DdbClient:           ddbClient,
		KmsClient:           kmsClient,
	})
	if err != nil {
		panic(err)
	}
	// Step 6: Call create_branch_key_id to create one new branch key
	branchKeyId, err := createbranchkeyid(keyStoreName, logicalKeyStoreName, keyStoreKMSKeyID, ddbClient, kmsClient)
	if err != nil {
		panic(err)
	}
	// Step 7: Create the Hierarchical Keyring HK1 with Key Store instance K1, partition_id,
	// the shared_cache and the branch_key_id.
	// Note that we are now providing an already initialized shared cache instead of just mentioning
	// the cache type and the Hierarchical Keyring initializing a cache at initialization.
	// partition_id for this example is a random UUID
	partitionId := "91c1b6a2-6fc3-4539-ad5e-938d597ed730"
	// Please make sure that you read the guidance on how to set Partition ID, Logical Key Store Name and
	// Branch Key ID at the top of this example before creating Hierarchical Keyrings with a Shared Cache
	hkeyringInput := mpltypes.CreateAwsKmsHierarchicalKeyringInput{
		KeyStore:    keyStore1,
		BranchKeyId: &branchKeyId,
		TtlSeconds:  600,
		Cache:       &shared_cache,
		PartitionId: &partitionId,
	}
	keyring1, err := matProv.CreateAwsKmsHierarchicalKeyring(context.Background(), hkeyringInput)
	// Step 8: Create encryption context for both tenants.
	// The Branch Key Id supplier uses the encryption context to determine which branch key id will
	// be used to encrypt data.
	// Remember that your encryption context is NOT SECRET.
	// For more information, see
	// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context

	// Create encryption context for TenantA
	encryptionContext := map[string]string{
		"tenant":                    "TenantA",
		"encryption":                "context",
		"is not":                    "secret",
		"but adds":                  "useful metadata",
		"that can help you":         "be confident that",
		"the data you are handling": "is what you think it is",
	}
	// Step 9: Encrypt the data for encryptionContext using keyring1
	encryptOutput, err := client.Encrypt(context.Background(), esdktypes.EncryptInput{
		Plaintext:         []byte(exampletext),
		EncryptionContext: encryptionContext,
		Keyring:           keyring1,
	})
	if err != nil {
		panic(err)
	}
	// Validate Ciphertext and Plaintext before encryption are NOT the same
	if string(encryptOutput.Ciphertext) == exampletext {
		panic("Ciphertext and Plaintext before encryption are the same")
	}
	// Step 10: Decrypt your encrypted data using the same keyring HK1 you used on encrypt.
	decryptOutput, err := client.Decrypt(context.Background(), esdktypes.DecryptInput{
		Ciphertext:        encryptOutput.Ciphertext,
		EncryptionContext: encryptionContext,
		Keyring:           keyring1,
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
	if string(decryptOutput.Plaintext) != exampletext {
		panic("Plaintext after decryption and Plaintext before encryption are NOT the same")
	}
	fmt.Println("Shared Cache Example Completed Successfully")
}
