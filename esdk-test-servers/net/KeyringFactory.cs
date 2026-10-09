// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Translation from the modeled CMM/Keyring configuration to real MPL
// cryptographic materials managers and keyrings. Enforces the "exactly one
// variant member set" invariant the tagged-union-via-optional-members shapes
// cannot express in the type system; everything else is delegated to the real
// MPL constructors so their validation failures surface as-is.

using System.Security.Cryptography;
using System.Text;
using Amazon;
using Amazon.DynamoDBv2;
using Amazon.KeyManagementService;
using Amazon.KeyManagementService.Model;
using AWS.Cryptography.KeyStore;
using AWS.Cryptography.MaterialProviders;

namespace EsdkTestServer;

internal static class KeyringFactory
{
    private static readonly byte[] PemPrefix = Encoding.ASCII.GetBytes("-----BEGIN ");

    // Resolves a modeled CryptographicMaterialsManager to a real MPL materials
    // manager, recursing through RequiredEncryptionContext variants.
    internal static ICryptographicMaterialsManager BuildCmm(MaterialProviders materialProviders, CmmConfig cmm)
    {
        var set = new[]
        {
            cmm.Default != null,
            cmm.RequiredEncryptionContext != null,
            cmm.CachingPresent,
        }.Count(present => present);
        if (set != 1)
        {
            throw new ArgumentException($"CryptographicMaterialsManager must set exactly one variant, found {set}");
        }

        if (cmm.Default != null)
        {
            return materialProviders.CreateDefaultCryptographicMaterialsManager(
                new CreateDefaultCryptographicMaterialsManagerInput
                {
                    Keyring = BuildKeyring(materialProviders, cmm.Default.Keyring),
                });
        }
        if (cmm.RequiredEncryptionContext != null)
        {
            return materialProviders.CreateRequiredEncryptionContextCMM(new CreateRequiredEncryptionContextCMMInput
            {
                UnderlyingCMM = BuildCmm(materialProviders, cmm.RequiredEncryptionContext.UnderlyingCmm),
                RequiredEncryptionContextKeys = cmm.RequiredEncryptionContext.RequiredEncryptionContextKeys,
            });
        }
        throw new ArgumentException("the Caching CMM is not supported: no caching CMM exists in the MPL");
    }

    // Resolves a modeled Keyring to a real MPL keyring, recursing through Multi
    // variants.
    internal static IKeyring BuildKeyring(MaterialProviders materialProviders, KeyringConfig keyring)
    {
        var set = new[]
        {
            keyring.AwsKms != null,
            keyring.AwsKmsMrk != null,
            keyring.AwsKmsMultiKeyring != null,
            keyring.AwsKmsMrkMultiKeyring != null,
            keyring.AwsKmsDiscovery != null,
            keyring.AwsKmsMrkDiscovery != null,
            keyring.AwsKmsRsa != null,
            keyring.RawAes != null,
            keyring.RawRsa != null,
            keyring.AwsKmsHierarchical != null,
            keyring.Multi != null,
        }.Count(present => present);
        if (set != 1)
        {
            throw new ArgumentException($"Keyring must set exactly one variant, found {set}");
        }

        if (keyring.RawAes is { } rawAes)
        {
            return materialProviders.CreateRawAesKeyring(new CreateRawAesKeyringInput
            {
                KeyNamespace = rawAes.KeyNamespace,
                KeyName = rawAes.KeyName,
                WrappingKey = new MemoryStream(rawAes.WrappingKey),
                WrappingAlg = rawAes.WrappingAlg,
            });
        }
        if (keyring.RawRsa is { } rawRsa)
        {
            return materialProviders.CreateRawRsaKeyring(new CreateRawRsaKeyringInput
            {
                KeyNamespace = rawRsa.KeyNamespace,
                KeyName = rawRsa.KeyName,
                PaddingScheme = rawRsa.PaddingScheme,
                PublicKey = rawRsa.PublicKey == null ? null : new MemoryStream(rawRsa.PublicKey),
                PrivateKey = rawRsa.PrivateKey == null ? null : new MemoryStream(rawRsa.PrivateKey),
            });
        }
        if (keyring.AwsKms is { } awsKms)
        {
            return materialProviders.CreateAwsKmsKeyring(new CreateAwsKmsKeyringInput
            {
                KmsClient = KmsClientForKey(awsKms.KmsKeyId),
                KmsKeyId = awsKms.KmsKeyId,
                GrantTokens = awsKms.GrantTokens,
            });
        }
        if (keyring.AwsKmsMrk is { } awsKmsMrk)
        {
            return materialProviders.CreateAwsKmsMrkKeyring(new CreateAwsKmsMrkKeyringInput
            {
                KmsClient = KmsClientForKey(awsKmsMrk.KmsKeyId),
                KmsKeyId = awsKmsMrk.KmsKeyId,
                GrantTokens = awsKmsMrk.GrantTokens,
            });
        }
        if (keyring.AwsKmsMultiKeyring is { } awsKmsMulti)
        {
            return materialProviders.CreateAwsKmsMultiKeyring(new CreateAwsKmsMultiKeyringInput
            {
                Generator = awsKmsMulti.Generator,
                KmsKeyIds = awsKmsMulti.KmsKeyIds,
            });
        }
        if (keyring.AwsKmsMrkMultiKeyring is { } awsKmsMrkMulti)
        {
            return materialProviders.CreateAwsKmsMrkMultiKeyring(new CreateAwsKmsMrkMultiKeyringInput
            {
                Generator = awsKmsMrkMulti.Generator,
                KmsKeyIds = awsKmsMrkMulti.KmsKeyIds,
            });
        }
        if (keyring.AwsKmsDiscovery is { } awsKmsDiscovery)
        {
            return materialProviders.CreateAwsKmsDiscoveryKeyring(new CreateAwsKmsDiscoveryKeyringInput
            {
                KmsClient = KmsClient(DefaultRegion()),
                DiscoveryFilter = BuildDiscoveryFilter(awsKmsDiscovery.DiscoveryFilter),
                GrantTokens = awsKmsDiscovery.GrantTokens,
            });
        }
        if (keyring.AwsKmsMrkDiscovery is { } awsKmsMrkDiscovery)
        {
            return materialProviders.CreateAwsKmsMrkDiscoveryKeyring(new CreateAwsKmsMrkDiscoveryKeyringInput
            {
                KmsClient = KmsClient(awsKmsMrkDiscovery.Region),
                Region = awsKmsMrkDiscovery.Region,
                DiscoveryFilter = BuildDiscoveryFilter(awsKmsMrkDiscovery.DiscoveryFilter),
                GrantTokens = awsKmsMrkDiscovery.GrantTokens,
            });
        }
        if (keyring.AwsKmsRsa is { } awsKmsRsa)
        {
            return BuildAwsKmsRsaKeyring(materialProviders, awsKmsRsa);
        }
        if (keyring.AwsKmsHierarchical is { } hierarchical)
        {
            return BuildAwsKmsHierarchicalKeyring(materialProviders, hierarchical);
        }
        return BuildMultiKeyring(materialProviders, keyring.Multi);
    }

    private static IKeyring BuildAwsKmsRsaKeyring(MaterialProviders materialProviders, AwsKmsRsaKeyringConfig config)
    {
        var kmsClient = KmsClientForKey(config.KmsKeyId);
        MemoryStream publicKey;
        if (config.PublicKey != null)
        {
            publicKey = new MemoryStream(config.PublicKey);
        }
        else
        {
            // Fetch the RSA public key from KMS so the keyring can OnEncrypt. KMS
            // GetPublicKey returns DER (X.509 SubjectPublicKeyInfo); the MPL wants PEM.
            var response = kmsClient.GetPublicKeyAsync(new GetPublicKeyRequest { KeyId = config.KmsKeyId })
                .GetAwaiter().GetResult();
            publicKey = DerToPublicKeyPem(response.PublicKey.ToArray());
        }
        var input = new CreateAwsKmsRsaKeyringInput
        {
            KmsClient = kmsClient,
            KmsKeyId = config.KmsKeyId,
            PublicKey = publicKey,
            GrantTokens = config.GrantTokens,
        };
        if (config.EncryptionAlgorithm != null)
        {
            input.EncryptionAlgorithm = config.EncryptionAlgorithm;
        }
        return materialProviders.CreateAwsKmsRsaKeyring(input);
    }

    private static IKeyring BuildAwsKmsHierarchicalKeyring(
        MaterialProviders materialProviders, AwsKmsHierarchicalKeyringConfig config)
    {
        var region = DefaultRegion();
        var keyStore = new KeyStore(new KeyStoreConfig
        {
            DdbTableName = config.KeyStoreTableName,
            LogicalKeyStoreName = config.LogicalKeyStoreName,
            KmsConfiguration = new KMSConfiguration { KmsKeyArn = config.KmsKeyArn },
            DdbClient = new AmazonDynamoDBClient(RegionEndpoint.GetBySystemName(region)),
            KmsClient = KmsClient(region),
        });
        return materialProviders.CreateAwsKmsHierarchicalKeyring(new CreateAwsKmsHierarchicalKeyringInput
        {
            KeyStore = keyStore,
            BranchKeyId = config.BranchKeyId,
            TtlSeconds = config.TtlSeconds,
            Cache = new CacheType { Default = new DefaultCache { EntryCapacity = 100 } },
        });
    }

    private static IKeyring BuildMultiKeyring(MaterialProviders materialProviders, MultiKeyringConfig config)
    {
        var children = config.ChildKeyrings
            .Select(child => BuildKeyring(materialProviders, child))
            .ToList();
        var input = new CreateMultiKeyringInput { ChildKeyrings = children };
        if (config.Generator != null)
        {
            input.Generator = BuildKeyring(materialProviders, config.Generator);
        }
        return materialProviders.CreateMultiKeyring(input);
    }

    private static DiscoveryFilter BuildDiscoveryFilter(DiscoveryFilterConfig config)
    {
        if (config == null)
        {
            return null;
        }
        return new DiscoveryFilter { Partition = config.Partition, AccountIds = config.AccountIds };
    }

    // Resolves the AWS region for KMS and DynamoDB clients.
    private static string DefaultRegion()
    {
        foreach (var key in new[] { "AWS_REGION", "AWS_DEFAULT_REGION", "ESDK_TESTSERVER_KMS_REGION" })
        {
            var value = Environment.GetEnvironmentVariable(key);
            if (!string.IsNullOrEmpty(value))
            {
                return value;
            }
        }
        return "us-west-2";
    }

    private static AmazonKeyManagementServiceClient KmsClient(string region) =>
        new(RegionEndpoint.GetBySystemName(region));

    // Builds a KMS client in the key's own region: KMS rejects an ARN whose
    // region differs from the client's region. Falls back to the default region
    // for a bare key id or alias that carries no region.
    private static AmazonKeyManagementServiceClient KmsClientForKey(string kmsKeyId)
    {
        var region = DefaultRegion();
        if (kmsKeyId.StartsWith("arn:", StringComparison.Ordinal))
        {
            var parts = kmsKeyId.Split(':');
            if (parts.Length > 3 && parts[3].Length > 0)
            {
                region = parts[3];
            }
        }
        return KmsClient(region);
    }

    // Wraps DER (X.509 SubjectPublicKeyInfo) bytes as a PEM PUBLIC KEY block,
    // passing bytes that already look like PEM through unchanged.
    private static MemoryStream DerToPublicKeyPem(byte[] der)
    {
        if (der.AsSpan().StartsWith(PemPrefix))
        {
            return new MemoryStream(der);
        }
        var pem = PemEncoding.Write("PUBLIC KEY", der);
        return new MemoryStream(Encoding.ASCII.GetBytes(pem));
    }
}
