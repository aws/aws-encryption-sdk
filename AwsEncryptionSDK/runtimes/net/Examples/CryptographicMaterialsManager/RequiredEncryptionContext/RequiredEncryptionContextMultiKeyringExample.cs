// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using System;
using System.Collections.Generic;
using System.IO;
using Amazon.DynamoDBv2;
using Amazon.KeyManagementService;
using AWS.Cryptography.EncryptionSDK;
using AWS.Cryptography.KeyStore;
using AWS.Cryptography.MaterialProviders;
using Xunit;
using static ExampleUtils.ExampleUtils;

/// Demonstrate an encrypt/decrypt cycle using a Required Encryption Context CMM with a Multi keyring.
/// This example shows how to use Required Encryption Context with multiple keyrings (KMS, Raw AES, Raw RSA, and Hierarchical),
/// where encryption context keys are not stored on the message but are required for decryption.
/// The example demonstrates that any of the individual keyrings can be used to decrypt the message
/// as long as the required encryption context is provided.
public class RequiredEncryptionContextMultiKeyringExample
{
    private static void Run(MemoryStream plaintext, string keyArn)
    {
        // Create your encryption context.
        // Remember that your encryption context is NOT SECRET.
        // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
        var encryptionContext = new Dictionary<string, string>()
        {
            {"encryption", "context"},
            {"is not", "secret"},
            {"but adds", "useful metadata"},
            {"that can help you", "be confident that"},
            {"the data you are handling", "is what you think it is"}
        };
        
        // Create your required encryption context keys.
        // These keys MUST be in your encryption context.
        // These keys and their corresponding values WILL NOT be stored on the message but will be used
        // for authentication during decryption.
        var requiredEncryptionContextKeys = new List<string>()
        {
            "encryption",
            "but adds",
            "the data you are handling"
        };
        
        // Instantiate the Material Providers and the AWS Encryption SDK
        var materialProviders = new MaterialProviders(new MaterialProvidersConfig());
        var encryptionSdk = new ESDK(new AwsEncryptionSdkConfig());

        // Create a KMS keyring to use as the generator.
        var createKmsKeyringInput = new CreateAwsKmsKeyringInput
        {
            KmsClient = new AmazonKeyManagementServiceClient(),
            KmsKeyId = keyArn
        };
        var kmsKeyring = materialProviders.CreateAwsKmsKeyring(createKmsKeyringInput);

        // Create a raw AES keyring to additionally encrypt under
        var rawAESKeyring = GetRawAESKeyring(materialProviders);

        // Create a raw RSA keyring
        var rawRSAKeyring = GetRawRSAKeyring(materialProviders);

        // Create a hierarchical keyring
        var hierarchicalKeyring = GetHierarchicalKeyring(materialProviders);

        // Create a MultiKeyring that consists of all the previously created Keyrings.
        // When using this MultiKeyring to encrypt data, any of the individual keyrings
        // may be used to decrypt the data.
        var createMultiKeyringInput = new CreateMultiKeyringInput
        {
            Generator = hierarchicalKeyring,
            ChildKeyrings = new List<IKeyring>() {kmsKeyring, rawRSAKeyring, rawAESKeyring}
        };
        var multiKeyring = materialProviders.CreateMultiKeyring(createMultiKeyringInput);
        
        // Create a required encryption context CMM using the multi keyring.
        var cmm = GetRequiredEncryptionContextCMM(materialProviders, requiredEncryptionContextKeys, multiKeyring);
        
        // Encrypt your plaintext data. NOTE: the keys "encryption", "but adds", and "the data you are handling"
        // WILL NOT be stored in the message header, but "is not" and "that can help you" WILL be stored.
        var encryptInput = new EncryptInput
        {
            Plaintext = plaintext,
            MaterialsManager = cmm,
            EncryptionContext = encryptionContext
        };
        var encryptOutput = encryptionSdk.Encrypt(encryptInput);
        var ciphertext = encryptOutput.Ciphertext;
        
        // Demonstrate that the ciphertext and plaintext are different.
        Assert.NotEqual(ciphertext.ToArray(), plaintext.ToArray());
        
        // Create the reproduced encryption context that contains ONLY the encryption context that
        // was NOT stored on the message but is required for authentication.
        var reproducedEncryptionContext = new Dictionary<string, string>()
        {
            {"encryption", "context"},
            {"but adds", "useful metadata"},
            {"the data you are handling", "is what you think it is"}
        };
        
        // Demonstrate that you can decrypt using the KMS keyring directly
        // as long as you provide the required encryption context.
        var kmsDecryptInput = new DecryptInput
        {
            Ciphertext = ciphertext,
            Keyring = kmsKeyring,
            EncryptionContext = reproducedEncryptionContext
        };
        var kmsDecryptOutput = encryptionSdk.Decrypt(kmsDecryptInput);
        
        // Verify the decrypted plaintext is identical to the original plaintext.
        VerifyDecryptedIsPlaintext(kmsDecryptOutput, plaintext);
        
        // Demonstrate that you can also decrypt using the raw AES keyring directly
        // as long as you provide the required encryption context.
        var aesDecryptInput = new DecryptInput
        {
            Ciphertext = ciphertext,
            Keyring = rawAESKeyring,
            EncryptionContext = reproducedEncryptionContext
        };
        var aesDecryptOutput = encryptionSdk.Decrypt(aesDecryptInput);
        
        // Verify the decrypted plaintext is identical to the original plaintext.
        VerifyDecryptedIsPlaintext(aesDecryptOutput, plaintext);
        
        // Demonstrate that you can also decrypt using the raw RSA keyring directly
        // as long as you provide the required encryption context.
        var rsaDecryptInput = new DecryptInput
        {
            Ciphertext = ciphertext,
            Keyring = rawRSAKeyring,
            EncryptionContext = reproducedEncryptionContext
        };
        var rsaDecryptOutput = encryptionSdk.Decrypt(rsaDecryptInput);
        
        // Verify the decrypted plaintext is identical to the original plaintext.
        VerifyDecryptedIsPlaintext(rsaDecryptOutput, plaintext);
        
        // Demonstrate that you can also decrypt using the hierarchical keyring directly
        // as long as you provide the required encryption context.
        var hierarchicalDecryptInput = new DecryptInput
        {
            Ciphertext = ciphertext,
            Keyring = hierarchicalKeyring,
            EncryptionContext = reproducedEncryptionContext
        };
        var hierarchicalDecryptOutput = encryptionSdk.Decrypt(hierarchicalDecryptInput);
        
        // Verify the decrypted plaintext is identical to the original plaintext.
        VerifyDecryptedIsPlaintext(hierarchicalDecryptOutput, plaintext);
        
        // Demonstrate that you can also decrypt using the multi keyring directly
        // as long as you provide the required encryption context.
        var multiKeyringDecryptInput = new DecryptInput
        {
            Ciphertext = ciphertext,
            Keyring = multiKeyring,
            EncryptionContext = reproducedEncryptionContext
        };
        var multiKeyringDecryptOutput = encryptionSdk.Decrypt(multiKeyringDecryptInput);
        
        // Verify the decrypted plaintext is identical to the original plaintext.
        VerifyDecryptedIsPlaintext(multiKeyringDecryptOutput, plaintext);
        
        // Demonstrate that decryption fails without the required encryption context
        var decryptFailed = false;
        var failDecryptInput = new DecryptInput
        {
            Ciphertext = ciphertext,
            Keyring = kmsKeyring,
            // Not providing the required encryption context
        };
        try
        {
            encryptionSdk.Decrypt(failDecryptInput);
        }
        catch (Exception)
        {
            decryptFailed = true;
        }
        
        Assert.True(decryptFailed, "Decryption should fail without required encryption context");
        
        // Demonstrate that decryption fails with incorrect encryption context values
        decryptFailed = false;
        var incorrectEncryptionContext = new Dictionary<string, string>()
        {
            {"encryption", "wrong_value"},
            {"but adds", "useful metadata"},
            {"the data you are handling", "is what you think it is"}
        };
        
        var failDecryptInput2 = new DecryptInput
        {
            Ciphertext = ciphertext,
            Keyring = rawAESKeyring,
            EncryptionContext = incorrectEncryptionContext
        };
        try
        {
            encryptionSdk.Decrypt(failDecryptInput2);
        }
        catch (Exception)
        {
            decryptFailed = true;
        }
        
        Assert.True(decryptFailed, "Decryption should fail with incorrect encryption context values");
    }
    
    private static void VerifyDecryptedIsPlaintext(DecryptOutput decryptOutput, MemoryStream plaintext)
    {
        // Demonstrate that the decrypted plaintext is identical to the original plaintext.
        var decrypted = decryptOutput.Plaintext;
        Assert.Equal(decrypted.ToArray(), plaintext.ToArray());
    }
    
    // Helper method to create a Raw RSA keyring for the example.
    private static IKeyring GetRawRSAKeyring(MaterialProviders materialProviders)
    {
        // Generate RSA key pair for the example
        RSAEncryption.RSA.GenerateKeyPairBytes(2048, out var publicKeyBytes, out var privateKeyBytes);
        var publicKey = new MemoryStream(publicKeyBytes);
        var privateKey = new MemoryStream(privateKeyBytes);

        // The key namespace and key name are defined by you
        // and are used by the raw RSA keyring to determine
        // whether it should attempt to decrypt an encrypted data key.
        var keyNamespace = "Some managed raw keys";
        var keyName = "My 2048-bit RSA wrapping key";

        // Create the RSA keyring
        var createRawRsaKeyringInput = new CreateRawRsaKeyringInput
        {
            KeyNamespace = keyNamespace,
            KeyName = keyName,
            PaddingScheme = PaddingScheme.OAEP_SHA512_MGF1,
            PublicKey = publicKey,
            PrivateKey = privateKey
        };
        return materialProviders.CreateRawRsaKeyring(createRawRsaKeyringInput);
    }
    
    // Helper method to create a Hierarchical keyring for the example.
    private static IKeyring GetHierarchicalKeyring(MaterialProviders materialProviders)
    {
        // THESE ARE PUBLIC RESOURCES DO NOT USE IN A PRODUCTION ENVIRONMENT
        var branchKeyId = "43574aa0-de30-424e-bad4-0b06f6e89478";
        
        var kmsConfig = new KMSConfiguration { KmsKeyArn = GetBranchKeyArn() };
        
        // Create an AWS KMS Configuration to use with your KeyStore.
        // The KMS Configuration MUST have the right access to the resources in the KeyStore.
        var keystoreConfig = new KeyStoreConfig
        {
            // Client MUST have permissions to decrypt kmsConfig.KmsKeyArn
            KmsClient = new AmazonKeyManagementServiceClient(),
            KmsConfiguration = kmsConfig,
            DdbTableName = GetKeyStoreName(),
            DdbClient = new AmazonDynamoDBClient(),
            LogicalKeyStoreName = GetLogicalKeyStoreName() 
        };
        var keystore = new KeyStore(keystoreConfig);
        
        // Create the Hierarchical keyring
        var createKeyringInput = new CreateAwsKmsHierarchicalKeyringInput
        {
            KeyStore = keystore,
            BranchKeyId = branchKeyId,
            // The value provided to `EntryCapacity` dictates how many branch keys will be held locally
            Cache = new CacheType { Default = new DefaultCache{EntryCapacity = 100} },
            // This dictates how often we call back to KMS to authorize use of the branch keys
            TtlSeconds = 600
        };
        return materialProviders.CreateAwsKmsHierarchicalKeyring(createKeyringInput);
    }
    
    // We test examples to ensure they remain up-to-date.
    [Fact]
    public void TestRequiredEncryptionContextMultiKeyringExample()
    {
        Run(GetPlaintextStream(), GetDefaultRegionKmsKeyArn());
    }
}
