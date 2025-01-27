// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
using Amazon.DynamoDBv2;
using Amazon.KeyManagementService;
using AWS.Cryptography.EncryptionSDK;
using AWS.Cryptography.KeyStore;
using AWS.Cryptography.MaterialProviders;
using Xunit;

/// <summary>
/// This example sets up the Hierarchical Keyring, 
/// which establishes a key hierarchy
/// where "branch" keys are persisted in DynamoDb.
/// These branch keys are used to protect your data keys,
/// and these branch keys are themselves protected by a KMS Key.
/// 
/// Establishing a key hierarchy like this has two benefits:
/// 
/// First, by caching the branch key material, and only calling
/// KMS to re-establish authentication regularly according to your configured TTL,
/// you limit how often you need to call KMS to protect your data.
/// This is a performance/security tradeoff, where your authentication, audit, and
/// logging from KMS is no longer one-to-one with every encrypt or decrypt call.
/// Additionally KMS Cloudtrail cannot be used to distinguish Encrypt and Decrypt calls,
/// and you cannot restrict who has Encryption rights from who has Decryption rights
/// since they both ONLY need KMS:Decrypt.
/// However, the benefit is that you no longer have to make a
/// network call to KMS for every encrypt or decrypt.
/// 
/// Second, this key hierarchy facilitates cryptographic isolation
/// of a tenant's data in a multi-tenant data store.
/// Each tenant can have a unique Branch Key,
/// that is only used to protect the tenant's data.
/// You can either statically configure a single branch key
/// to ensure you are restricting access to a single tenant,
/// or you can implement an interface that selects 
/// the Branch Key based on the Encryption Context.
/// 
/// This example demonstrates configuring a Hierarchical Keyring
/// with a Branch Key ID Supplier to encrypt and decrypt data for
/// two separate tenants.
/// 
/// This example requires read access to the DDB Table where you are storing the Branch Keys.
/// This table must be configured with the following
/// primary key configuration:
/// - Partition key is named "partition_key" with type (S)
/// - Sort key is named "sort_key" with type (S)
///
/// This example assumes there are Branch Keys in the Key Store.
/// For a more detailed example on how to create a Key Store and add
/// branch keys to it, see the `AwsKmsHierarchicalKeyring` Example in this directory.
///  
/// In order to run this example you need:
///   - "dynamodb:Query"
///   - "dynamodb:GetItem"
///   - "kms:Decrypt"
///  
/// This example also requires using a KMS Key.
/// You need the following access on this key:
/// - Decrypt
/// </summary>
public class AwsKmsHierarchicalKeyringBranchKeySupplier
{
    // THESE ARE PUBLIC RESOURCES DO NOT USE IN A PRODUCTION ENVIRONMENT
    private static string branchKeyIdA = "43574aa0-de30-424e-bad4-0b06f6e89478";
    private static string branchKeyIdB = "a2f4be37-15ec-489a-bcb5-dcce1f6bfe84";
    private static void Run(MemoryStream plaintext)
    {
        // Initial KeyStore Setup: This example requires that you have already
        // created your KeyStore, and have populated it with two new branch keys.
        // See the "Create KeyStore Table Example" and "Create KeyStore Key Example"
        // for an example of how to do this.

        // 1. Configure your KeyStore resource.
        //    This SHOULD be the same configuration that you used
        //    to initially create and populate your KeyStore.
        var kmsConfig = new KMSConfiguration { KmsKeyArn = ExampleUtils.ExampleUtils.GetBranchKeyArn() };
        // Create an AWS KMS Configuration to use with your KeyStore.
        // The KMS Configuration MUST have the right access to the resources in the KeyStore.
        var keystoreConfig = new KeyStoreConfig
        {
            // Client MUST have permissions to decrypt kmsConfig.KmsKeyArn
            KmsClient = new AmazonKeyManagementServiceClient(),
            KmsConfiguration = kmsConfig,
            DdbTableName = ExampleUtils.ExampleUtils.GetKeyStoreName(),
            DdbClient = new AmazonDynamoDBClient(),
            LogicalKeyStoreName = ExampleUtils.ExampleUtils.GetLogicalKeyStoreName() 
        };
        
        var keystore = new KeyStore(keystoreConfig);
        var materialProviders = new MaterialProviders(new MaterialProvidersConfig());
        
        // 2. Create a Branch Key ID Supplier. See ExampleBranchKeyIdSupplier in this directory.
        var branchKeySupplier = new ExampleBranchKeySupplier(branchKeyIdA, branchKeyIdB);
        
        
        // 3. Create the Hierarchical Keyring, using the Branch Key ID Supplier above.
        //    With this configuration, the AWS SDK Client ultimately configured will be capable
        //    of encrypting or decrypting items for either tenant (assuming correct KMS access).
        //    If you want to restrict the client to only encrypt or decrypt for a single tenant,
        //    configure this Hierarchical Keyring using `BranchKeyId = tenant1BranchKeyId` instead
        //    of `BranchKeyIdSupplier = branchKeySupplier`.
        var createKeyringInput = new CreateAwsKmsHierarchicalKeyringInput
        {
            KeyStore = keystore,
            // This branchKeyId you have configured your keyring with MUST be decrypted by the 
            // KMS config in the keystore and therefore MUST have the right permissions.
            BranchKeyIdSupplier = branchKeySupplier,
            // The value provided to `EntryCapacity` dictates how many branch keys will be held locally
            Cache = new CacheType { Default = new DefaultCache{EntryCapacity = 100} },
            // This dictates how often we call back to KMS to authorize use of the branch keys
            TtlSeconds = 600
        };
        var keyring = materialProviders.CreateAwsKmsHierarchicalKeyring(createKeyringInput);
        
        // The Branch Key Id supplier uses the encryption context to determine which branch key id 
        // will be used to encrypt data.
        var encryptionContextA = new Dictionary<string, string>()
        {
            // We will encrypt with TenantKeyA
            {"tenant", "TenantA"},
            {"encryption", "context"},
            {"is not", "secret"},
            {"but adds", "useful metadata"},
            {"that can help you", "be confident that"},
            {"the data you are handling", "is what you think it is"}
        };
        var encryptionContextB = new Dictionary<string, string>()
        {
            // We will encrypt with TenantKeyB
            {"tenant", "TenantB"},
            {"encryption", "context"},
            {"is not", "secret"},
            {"but adds", "useful metadata"},
            {"that can help you", "be confident that"},
            {"the data you are handling", "is what you think it is"}
        };
        
        var encryptionSDK =  new ESDK(new AwsEncryptionSdkConfig());
        
        var encryptInputA = new EncryptInput
        {
            Plaintext = plaintext,
            Keyring = keyring,
            // We will encrypt with TenantKeyA
            EncryptionContext = encryptionContextA
        };

        var encryptInputB = new EncryptInput{
            Plaintext = plaintext,
            Keyring = keyring,
            // We will encrypt with TenantKeyB
            EncryptionContext = encryptionContextB
        };
        
        var encryptOutputA = encryptionSDK.Encrypt(encryptInputA);
        var encryptOutputB = encryptionSDK.Encrypt(encryptInputB);
        
        // To attest that TenantKeyB cannot decrypt a message written by TenantKeyA
        // let's construct more restrictive hierarchical keyrings.

        var createHierarchicalKeyringA = new CreateAwsKmsHierarchicalKeyringInput()
        {
            KeyStore = keystore,
            BranchKeyId = branchKeyIdA,
            Cache = new CacheType { Default = new DefaultCache{EntryCapacity = 100} },
            TtlSeconds = 600
        };
        var hierarchicalKeyringA = materialProviders.CreateAwsKmsHierarchicalKeyring(createHierarchicalKeyringA);
        
        var createHierarchicalKeyringB = new CreateAwsKmsHierarchicalKeyringInput()
        {
            KeyStore = keystore,
            BranchKeyId = branchKeyIdB,
            Cache = new CacheType { Default = new DefaultCache{EntryCapacity = 100} },
            TtlSeconds = 600
        };
        var hierarchicalKeyringB = materialProviders.CreateAwsKmsHierarchicalKeyring(createHierarchicalKeyringB);

        var decryptFailed = false;
        try
        {
            // Try to use keyring for Tenant B to decrypt a message encrypted with Tenant A's key
            // Expected to fail.
            var decryptInput = new DecryptInput
            {
                Ciphertext = encryptOutputA.Ciphertext,
                Keyring = hierarchicalKeyringB,
            };
            encryptionSDK.Decrypt(decryptInput);
        }
        catch (AwsCryptographicMaterialProvidersException)
        {
            decryptFailed = true;
        }
        Assert.True(decryptFailed);
        
        decryptFailed = false;
        try
        {
            // Try to use keyring for Tenant A to decrypt a message encrypted with Tenant B's key
            // Expected to fail.
            var decryptInput = new DecryptInput
            {
                Ciphertext = encryptOutputB.Ciphertext,
                Keyring = hierarchicalKeyringA,
            };
            encryptionSDK.Decrypt(decryptInput);
        }
        catch (AwsCryptographicMaterialProvidersException)
        {
            decryptFailed = true;
        }
        Assert.True(decryptFailed);
        
        // Decrypt your encrypted data using the same keyring you used on encrypt.
        encryptionSDK.Decrypt(new DecryptInput {
            Ciphertext = encryptOutputA.Ciphertext,
            Keyring = keyring }
        );
        encryptionSDK.Decrypt(new DecryptInput {
            Ciphertext = encryptOutputB.Ciphertext,
            Keyring = keyring }
        );

    }
    
    // We test examples to ensure they remain up-to-date.
    [Fact]
    public void TestAwsKmsHierarchicalKeyringExample()
    {
        Run(ExampleUtils.ExampleUtils.GetPlaintextStream());
    }

}
