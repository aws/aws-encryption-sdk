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
/// This example demonstrates how to:
/// - Create a key store
/// - Create a branch key
/// - Version a branch key
/// - Configure a Hierarchical Keyring
///   with a static branch key configuration to ensure we are restricting
///   access to a single tenant. 
/// 
/// This example requires read, write, and DescribeTable access to the DDB Table where you are storing the Branch Keys.
/// This table must be configured with the following
/// primary key configuration:
/// - Partition key is named "partition_key" with type (S)
/// - Sort key is named "sort_key" with type (S)
///
/// For more information on the AWS KMS Hierarchical Keyring visit the Developer Guide:
/// https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-hierarchical-keyring.html
///
/// This example also requires using a KMS Key.
/// You need the following access on this key:
/// - GenerateDataKeyWithoutPlaintext
/// - ReEncrypt
/// - Decrypt
/// </summary>
public class AwsKmsHierarchicalKeyring
{
    private void Run(MemoryStream plaintext)
    {
        // 1. Configure your KeyStore resource.
        //    `ddbTableName` is the name you want for the DDB table that
        //    will back your keystore.
        //    `kmsKeyArn` is the KMS Key that will protect your branch keys and beacon keys
        //    when they are stored in your DDB table.
        var keyStoreConfig = new KeyStoreConfig
        {
            KmsClient = new AmazonKeyManagementServiceClient(),
            KmsConfiguration = new KMSConfiguration{ KmsKeyArn = ExampleUtils.ExampleUtils.GetBranchKeyArn() },
            DdbTableName = ExampleUtils.ExampleUtils.GetKeyStoreName(),
            DdbClient = new AmazonDynamoDBClient(),
            LogicalKeyStoreName = ExampleUtils.ExampleUtils.GetLogicalKeyStoreName() 
        };
        var keyStore = new KeyStore(keyStoreConfig);
        
        // 2. Create the DynamoDb table that will store the branch keys and beacon keys.
        //    This checks if the correct table already exists at `ddbTableName`
        //    by using the DescribeTable API. If no table exists,
        //    it will create one. If a table exists, it will verify
        //    the table's configuration and will error if the configuration is incorrect.
        //    It may take a couple minutes for the table to become ACTIVE,
        //    at which point it is ready to store branch and beacon keys.
        //    To create a KeyStore table, you need:
        //     - "dynamodb:DescribeTable",
        //     - "dynamodb:CreateTable",
       
        // This will NOT create a Key Store but check that the configured table name exists and it validates
        // its construction. In order to check the construction is correct the configured IAM role
        // MUST have `DescribeTable` permission on the Key Store resource.
        keyStore.CreateKeyStore(new CreateKeyStoreInput());
        
        // We have already created a Table with the specified configuration.
        // For testing purposes we will not create a table when we run this example.
        
        // 3. Create a new branch key and beacon key in our KeyStore.
        //    Both the branch key and the beacon key will share an Id.
        //    This creation is eventually consistent. See the CreateBranchKey
        //    Example for how to populate this table.
        //    To create a Branch Key and a Beacon Key you need:
        //    - "dynamodb:PutItem",
        //    - "dynamodb:ConditionCheckItem",
        //    - "dynamodb:UpdateItem" 
        
        // var branchKeyId = CreateBranchKey.createBranchKey();
        
        // For testing purposes we will not create another Branch Key when we run this example.
        // We will use an existing branch key created using the above code to run this
        // example.
        
        // THIS IS A PUBLIC RESOURCE DO NOT USE IN A PRODUCTION ENVIRONMENT
        var branchKeyId = "43574aa0-de30-424e-bad4-0b06f6e89478";
        
        // 4. Create the Hierarchical Keyring, with a static Branch Key ID.
        //    With this configuration, the AWS SDK Client ultimately configured will be capable
        //    of encrypting or decrypting items for either tenant (assuming correct KMS access).
        //    We are restricting the keyring to only branch key by statically configuring
        //    it with a branch key id. For an examples on how to decide on which branch key to 
        //    use see the `AwsKmsHierarchicalKeyringBranhcKeySupplier` example in this directory.
        var createKeyringInput = new CreateAwsKmsHierarchicalKeyringInput
        {
            KeyStore = keyStore,
            BranchKeyId = branchKeyId,
            // The value provided to `EntryCapacity` dictates how many branch keys will be held locally
            Cache = new CacheType { Default = new DefaultCache{EntryCapacity = 100} },
            // This dictates how often we call back to KMS to authorize use of the branch keys
            TtlSeconds = 600
        };
        var materialProviders = new MaterialProviders(new MaterialProvidersConfig());
        var keyring = materialProviders.CreateAwsKmsHierarchicalKeyring(createKeyringInput);
        
       
        // 5. Create an encryption context
        //    Most encrypted data should have an associated encryption context
        //    to protect integrity. This sample uses placeholder values.
        //    For more information see:
        //    blogs.aws.amazon.com/security/post/Tx2LZ6WBJJANTNW/How-to-Protect-the-Integrity-of-Your-Encrypted-Data-by-Using-AWS-Key-Management
        var encryptionContext = new Dictionary<string, string>()
        {
            {"encryption", "context"},
            {"is not", "secret"},
            {"but adds", "useful metadata"},
            {"that can help you", "be confident that"},
            {"the data you are handling", "is what you think it is"}
        };
        
        var encryptionSdk =  new ESDK(new AwsEncryptionSdkConfig());
        
        var encryptInput = new EncryptInput
        {
            Plaintext = plaintext,
            Keyring = keyring,
            EncryptionContext = encryptionContext
        };
        
        // 6. Encrypt the Data
        //    To encrypt data using the Hierarchical Keyring you need:
        //    - "dynamodb:Query",
        //    - "kms:Decrypt"
        var encryptOutput = encryptionSdk.Encrypt(encryptInput);
        
        // Demonstrate that the ciphertext and plaintext are different.
        Assert.NotEqual(encryptOutput.Ciphertext.ToArray(), plaintext.ToArray());
        
        var decryptInput = new DecryptInput
        {
            Ciphertext = encryptOutput.Ciphertext,
            EncryptionContext = encryptionContext,
            Keyring = keyring
        };

        // 7. Decrypt the Data
        //    To decrypt data using the Hierarchical Keyring you need:
        //    - "dynamodb:GetItem"
        //    - "kms:Decrypt"
        var decryptOutput = encryptionSdk.Decrypt(decryptInput);
        
        // Demonstrate that the decrypted ciphertext and plaintext are the same
        Assert.Equal(decryptOutput.Plaintext.ToArray(), plaintext.ToArray());
        
        // 8. Rotate the Branch Key in our KeyStore.
        //    Only the branch key will be rotated. 
        //    This rotation is eventually consistent. 
        //    See the VersionBranchKey for a narrower example.
        //    See the Developer Guide for details:
        //    https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-hierarchical-keyring.html#rotate-branch-key
        //    Example for how to rotate a branch key.
        //    To rotate a branch key you need:
        //    - "dynamodb:GetItem"
        //    - "kms:ReEncrypt*"
        //    - "kms:GenerateDataKeyWithoutPlaintext" 
        
        // For testing purposes we will not version this key when we run this example.
        // VersionBranchKey.versionBranchKey(branchKeyId);
    }

    [Fact]
    public void TestAwsKmsHierarchicalKeyring()
    {
        Run(ExampleUtils.ExampleUtils.GetPlaintextStream());
    }

}
