using Amazon.DynamoDBv2;
using Amazon.KeyManagementService;
using AWS.Cryptography.EncryptionSDK;
using AWS.Cryptography.KeyStore;
using AWS.Cryptography.MaterialProviders;
using Xunit;

/// <summary>
/// This example demonstrates how to use a shared cache across multiple Hierarchical Keyrings.
/// With this functionality, users only need to maintain one common shared cache across multiple
/// Hierarchical Keyrings with different Key Stores instances/KMS Clients/KMS Keys.
/// 
/// There are three important parameters that users need to carefully set while providing the shared cache:
/// 
/// 1. Partition ID - Partition ID is an optional parameter provided to the Hierarchical Keyring input,
/// which distinguishes Cryptographic Material Providers (i.e: Keyrings) writing to a cache.
/// - If the Partition ID is set and is the same for two Hierarchical Keyrings (or another Material Provider),
///   they CAN share the same cache entries in the cache.
/// - If the Partition ID is set and is different for two Hierarchical Keyrings (or another Material Provider),
///   they CANNOT share the same cache entries in the cache.
/// - If the Partition ID is not set by the user, it is initialized as a random 16-byte UUID which makes
///   it unique for every Hierarchical Keyring, and two Hierarchical Keyrings (or another Material Provider)
///   CANNOT share the same cache entries in the cache.
/// 
/// 2. Logical Key Store Name - This parameter is set by the user when configuring the Key Store for
/// the Hierarchical Keyring. This is a logical name for the branch key store.
/// Suppose you have a physical Key Store (K). You create two instances of K (K1 and K2). Now, you create
/// two Hierarchical Keyrings (HK1 and HK2) with these Key Store instances (K1 and K2 respectively).
/// - If you want to share cache entries across these two keyrings, you should set the Logical Key Store Names
///   for both the Key Store instances (K1 and K2) to be the same.
/// - If you set the Logical Key Store Names for K1 and K2 to be different, HK1 (which uses Key Store instance K1)
///   and HK2 (which uses Key Store instance K2) will NOT be able to share cache entries.
/// 
/// 3. Branch Key ID - Choose an effective Branch Key ID Schema
/// 
/// This is demonstrated in the example below.
/// Notice that both K1 and K2 are instances of the same physical Key Store (K).
/// You MUST NEVER have two different physical Key Stores with the same Logical Key Store Name.
/// 
/// Important Note: If you have two or more Hierarchy Keyrings with:
/// - Same Partition ID
/// - Same Logical Key Store Name of the Key Store for the Hierarchical Keyring 
/// - Same Branch Key ID
/// then they WILL share the cache entries in the Shared Cache.
/// Please make sure that you set all of Partition ID, Logical Key Store Name and Branch Key ID
/// to be the same for two Hierarchical Keyrings if and only if you want them to share cache entries.
/// 
/// This example first creates a shared cache that you can use across multiple Hierarchical Keyrings.
/// The example then configures a Hierarchical Keyring (HK1 and HK2) with the shared cache,
/// a Branch Key ID and two instances (K1 and K2) of the same physical Key Store (K) respectively,
/// i.e. HK1 with K1 and HK2 with K2. The example demonstrates that if you set the same Partition ID
/// for HK1 and HK2, the two keyrings can share cache entries.
/// If you set different Partition ID of the Hierarchical Keyrings, or different
/// Logical Key Store Names of the Key Store instances, then the keyrings will NOT
/// be able to share cache entries.
///
/// This example requires access to the DDB Table (K) where you are storing the Branch Keys. This
/// table must be configured with the following primary key configuration: - Partition key is named
/// "partition_key" with type (S) - Sort key is named "sort_key" with type (S)
///
/// This example also requires using a KMS Key. You need the following access on this key:
/// - GenerateDataKeyWithoutPlaintext
/// - Decrypt
/// </summary>
public class SharedCacheAcrossHierarchicalKeyrings
{
    // THESE ARE PUBLIC RESOURCES DO NOT USE IN A PRODUCTION ENVIRONMENT
    private static string branchKeyId = "43574aa0-de30-424e-bad4-0b06f6e89478";
    private static void Run(MemoryStream plaintext)
    {
        // Create the CryptographicMaterialsCache (CMC) to share across multiple Hierarchical Keyrings
        // using the Material Providers Library
        //    This CMC takes in:
        //     - CacheType
        var materialProviders = new MaterialProviders(new MaterialProvidersConfig());

        var cache = new CacheType { Default = new DefaultCache{EntryCapacity = 100} };

        var cryptographicMaterialsCacheInput = new CreateCryptographicMaterialsCacheInput {Cache = cache};
        
        var sharedCryptographicMaterialsCache = materialProviders.CreateCryptographicMaterialsCache(cryptographicMaterialsCacheInput);

        // Create a CacheType object for the sharedCryptographicMaterialsCache
        // Note that the `cache` parameter in the Hierarchical Keyring Input takes a `CacheType` as input
        // Here, we pass a `Shared` CacheType that passes an already initialized shared cache
        var sharedCache = new CacheType { Shared = sharedCryptographicMaterialsCache };

        // Instantiate the SDK
        // This builds the AwsCrypto client with the RequireEncryptRequireDecrypt commitment policy,
        // which enforces that this client only encrypts using committing algorithm suites and enforces
        // that this client will only decrypt encrypted messages that were created with a committing
        // algorithm suite.
        // This is the default commitment policy if you build the client with
        // `AwsCrypto.builder().build()`
        // or `AwsCrypto.standard()`.
        var encryptionSDK =  new ESDK(new AwsEncryptionSdkConfig());

        // Configure your KeyStore resource keystore1.
        //    This SHOULD be the same configuration that you used
        //    to initially create and populate your physical KeyStore.
        // Note that ddbTableName keyStoreTableName is the physical Key Store,
        // and keystore1 is instances of this physical Key Store.

        // Create an AWS KMS Configuration to use with your KeyStore.
        // The KMS Configuration MUST have the right access to the resources in the KeyStore.
        var kmsConfig = new KMSConfiguration { KmsKeyArn = ExampleUtils.ExampleUtils.GetBranchKeyArn() };
        
        var keystoreConfig = new KeyStoreConfig
        {
            // Client MUST have permissions to decrypt kmsConfig.KmsKeyArn
            KmsClient = new AmazonKeyManagementServiceClient(),
            KmsConfiguration = kmsConfig,
            DdbTableName = ExampleUtils.ExampleUtils.GetKeyStoreName(),
            DdbClient = new AmazonDynamoDBClient(),
            LogicalKeyStoreName = ExampleUtils.ExampleUtils.GetLogicalKeyStoreName() 
        };

        
        var keystore1 = new KeyStore(keystoreConfig);

        // Create the Hierarchical Keyring HK1 with Key Store instance K1, partitionId,
        // the shared Cache and the BranchKeyId.
        // Note that we are now providing an already initialized shared cache instead of just mentioning
        // the cache type and the Hierarchical Keyring initializing a cache at initialization.
        
        // partitionId for this example is a random UUID
        var partitionId = "91c1b6a2-6fc3-4539-ad5e-938d597ed730";

        // Please make sure that you read the guidance on how to set Partition ID, Logical Key Store Name and
        // Branch Key ID at the top of this example before creating Hierarchical Keyrings with a Shared Cache

        var createKeyringInput1 = new CreateAwsKmsHierarchicalKeyringInput
        {
            KeyStore = keystore1,
            // This branchKeyId you have configured your keyring with MUST be decrypted by the 
            // KMS config in the keystore and therefore MUST have the right permissions.
            BranchKeyId = branchKeyId,
            // The value provided to `EntryCapacity` dictates how many branch keys will be held locally
            Cache = sharedCache,
            // This dictates how often we call back to KMS to authorize use of the branch keys
            TtlSeconds = 600,
            PartitionId = partitionId
        };
        var keyring1 = materialProviders.CreateAwsKmsHierarchicalKeyring(createKeyringInput1);
        
        // Create example encryption context
        var encryptionContext = new Dictionary<string, string>()
        {
            {"encryption", "context"},
            {"is not", "secret"},
            {"but adds", "useful metadata"},
            {"that can help you", "be confident that"},
            {"the data you are handling", "is what you think it is"}
        };
        
        // Encrypt the data for encryptionContext using keyring1
        var encryptInput1 = new EncryptInput
        {
            Plaintext = plaintext,
            Keyring = keyring1,
            EncryptionContext = encryptionContext
        };
        
        var encryptOutput1 = encryptionSDK.Encrypt(encryptInput1);
        
        
        // Decrypt your encrypted data using the same keyring HK1 you used on encrypt.
        var decryptOutput1 = encryptionSDK.Decrypt(new DecryptInput {
            Ciphertext = encryptOutput1.Ciphertext,
            Keyring = keyring1 }
        );

        // Demonstrate that the decrypted plaintext is identical to the original plaintext.
        var decrypted1 = decryptOutput1.Plaintext;
        Assert.Equal(decrypted1.ToArray(), plaintext.ToArray());

        // Through the above encrypt and decrypt roundtrip, the cache will be populated and
        // the cache entries can be used by another Hierarchical Keyring with the
        // - Same Partition ID
        // - Same Logical Key Store Name of the Key Store for the Hierarchical Keyring 
        // - Same Branch Key ID

        // Configure your KeyStore resource keystore2.
        //    This SHOULD be the same configuration that you used
        //    to initially create and populate your physical KeyStore.
        // Note that ddbTableName keyStoreTableName is the physical Key Store,
        // and keystore2 is instances of this physical Key Store.

        // Note that for this example, keystore2 is identical to keystore1.
        // You can optionally change configurations like KMS Client or KMS Key ID based
        // on your use-case.
        // Make sure you have the required permissions to use different configurations.

        // - If you want to share cache entries across two keyrings HK1 and HK2,
        //   you should set the Logical Key Store Names for both
        //   Key Store instances (K1 and K2) to be the same.
        // - If you set the Logical Key Store Names for K1 and K2 to be different,
        //   HK1 (which uses Key Store instance K1) and HK2 (which uses Key Store
        //   instance K2) will NOT be able to share cache entries.
        var keystore2 = new KeyStore(keystoreConfig);

        // Create the Hierarchical Keyring HK2 with Key Store instance K2, the shared Cache
        // and the same partitionId and BranchKeyId used in HK1 because we want to share cache entries
        // (and experience cache HITS).

        // Please make sure that you read the guidance on how to set Partition ID, Logical Key Store Name and
        // Branch Key ID at the top of this example before creating Hierarchical Keyrings with a Shared Cache

        var createKeyringInput2 = new CreateAwsKmsHierarchicalKeyringInput
        {
            KeyStore = keystore2,
            // This branchKeyId you have configured your keyring with MUST be decrypted by the 
            // KMS config in the keystore and therefore MUST have the right permissions.
            BranchKeyId = branchKeyId,
            // The value provided to `EntryCapacity` dictates how many branch keys will be held locally
            Cache = sharedCache,
            // This dictates how often we call back to KMS to authorize use of the branch keys
            TtlSeconds = 600,
            PartitionId = partitionId
        };
        var keyring2 = materialProviders.CreateAwsKmsHierarchicalKeyring(createKeyringInput2);

        // This encrypt-decrypt roundtrip with HK2 will experience Cache HITS from previous HK1 roundtrip
        // Encrypt the data for encryptionContext using hierarchicalKeyring2
        var encryptInput2 = new EncryptInput
        {
            Plaintext = plaintext,
            Keyring = keyring2,
            EncryptionContext = encryptionContext
        };
        
        var encryptOutput2 = encryptionSDK.Encrypt(encryptInput2);

        // Decrypt your encrypted data using the same keyring HK2 you used on encrypt.
        var decryptOutput2 = encryptionSDK.Decrypt(new DecryptInput {
            Ciphertext = encryptOutput2.Ciphertext,
            Keyring = keyring2 }
        );

        // Demonstrate that the decrypted plaintext is identical to the original plaintext.
        var decrypted2 = decryptOutput2.Plaintext;
        Assert.Equal(decrypted2.ToArray(), plaintext.ToArray());
    }
    
    // We test examples to ensure they remain up-to-date.
    [Fact]
    public void TestAwsKmsHierarchicalKeyringExample()
    {
        Run(ExampleUtils.ExampleUtils.GetPlaintextStream());
    }

}
