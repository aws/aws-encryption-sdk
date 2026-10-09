// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Native .NET mirror of the ESDK TestServer Smithy model shapes and their
// rpcv2Cbor wire form: structure members are keyed by the Smithy member name,
// blobs travel as CBOR byte strings, and enums as the Smithy member NAME
// strings (the MPL enum VALUES are hex suite ids, so both directions map by
// name through the tables below).
//
// The polymorphic CryptographicMaterialsManager and Keyring shapes are tagged
// unions via optional members: every variant decodes as optional here and the
// exactly-one-set invariant is enforced at build time in KeyringFactory.

using System.Formats.Cbor;
using Amazon.KeyManagementService;
using AWS.Cryptography.MaterialProviders;

namespace EsdkTestServer;

// Operation request / response shapes.

internal sealed class CreateClientRequest
{
    internal EsdkClientConfig Config;
}

internal sealed class CreateClientResponse
{
    internal string ClientId;
}

internal sealed class EncryptRequest
{
    internal string ClientId;
    internal byte[] Plaintext;
    internal Dictionary<string, string> EncryptionContext;
    internal ESDKAlgorithmSuiteId AlgorithmSuiteId;
    internal long? FrameLength;
}

internal sealed class EncryptResponse
{
    internal byte[] Ciphertext;
}

internal sealed class DecryptRequest
{
    internal string ClientId;
    internal byte[] Ciphertext;
    internal Dictionary<string, string> EncryptionContext;
}

internal sealed class DecryptResponse
{
    internal byte[] Plaintext;
    internal Dictionary<string, string> EncryptionContext;
    internal ESDKAlgorithmSuiteId AlgorithmSuiteId;
}

// Client configuration shapes.

internal sealed class EsdkClientConfig
{
    internal ESDKCommitmentPolicy CommitmentPolicy;
    internal long? MaxEncryptedDataKeys;
    internal CmmConfig Cmm;
}

internal sealed class CmmConfig
{
    internal DefaultCmmConfig Default;
    internal RequiredEncryptionContextCmmConfig RequiredEncryptionContext;
    // Marks the Caching variant as present; its members are not decoded because
    // the server rejects the variant as unsupported.
    internal bool CachingPresent;
}

internal sealed class DefaultCmmConfig
{
    internal KeyringConfig Keyring;
}

internal sealed class RequiredEncryptionContextCmmConfig
{
    internal CmmConfig UnderlyingCmm;
    internal List<string> RequiredEncryptionContextKeys;
}

internal sealed class KeyringConfig
{
    internal AwsKmsKeyringConfig AwsKms;
    internal AwsKmsKeyringConfig AwsKmsMrk;
    internal AwsKmsMultiKeyringConfig AwsKmsMultiKeyring;
    internal AwsKmsMultiKeyringConfig AwsKmsMrkMultiKeyring;
    internal AwsKmsDiscoveryKeyringConfig AwsKmsDiscovery;
    internal AwsKmsMrkDiscoveryKeyringConfig AwsKmsMrkDiscovery;
    internal AwsKmsRsaKeyringConfig AwsKmsRsa;
    internal RawAesKeyringConfig RawAes;
    internal RawRsaKeyringConfig RawRsa;
    internal AwsKmsHierarchicalKeyringConfig AwsKmsHierarchical;
    internal MultiKeyringConfig Multi;
}

// Carries the AwsKms and AwsKmsMrk variants (same shape).
internal sealed class AwsKmsKeyringConfig
{
    internal string KmsKeyId;
    internal List<string> GrantTokens;
}

// Carries the AwsKmsMultiKeyring and AwsKmsMrkMultiKeyring variants (same shape).
internal sealed class AwsKmsMultiKeyringConfig
{
    internal string Generator;
    internal List<string> KmsKeyIds;
}

internal sealed class AwsKmsDiscoveryKeyringConfig
{
    internal DiscoveryFilterConfig DiscoveryFilter;
    internal List<string> GrantTokens;
}

internal sealed class AwsKmsMrkDiscoveryKeyringConfig
{
    internal string Region;
    internal DiscoveryFilterConfig DiscoveryFilter;
    internal List<string> GrantTokens;
}

internal sealed class AwsKmsRsaKeyringConfig
{
    internal string KmsKeyId;
    internal byte[] PublicKey;
    internal EncryptionAlgorithmSpec EncryptionAlgorithm;
    internal List<string> GrantTokens;
}

internal sealed class DiscoveryFilterConfig
{
    internal string Partition;
    internal List<string> AccountIds;
}

internal sealed class RawAesKeyringConfig
{
    internal string KeyNamespace;
    internal string KeyName;
    internal byte[] WrappingKey;
    internal AesWrappingAlg WrappingAlg;
}

internal sealed class RawRsaKeyringConfig
{
    internal string KeyNamespace;
    internal string KeyName;
    internal PaddingScheme PaddingScheme;
    internal byte[] PublicKey;
    internal byte[] PrivateKey;
}

internal sealed class AwsKmsHierarchicalKeyringConfig
{
    internal string BranchKeyId;
    internal string KeyStoreTableName;
    internal string LogicalKeyStoreName;
    internal string KmsKeyArn;
    internal long TtlSeconds;
}

internal sealed class MultiKeyringConfig
{
    internal KeyringConfig Generator;
    internal List<KeyringConfig> ChildKeyrings;
}

internal static class Model
{
    // Enum tables: wire member name -> library constant.

    private static readonly Dictionary<string, ESDKCommitmentPolicy> CommitmentPolicies = new()
    {
        ["FORBID_ENCRYPT_ALLOW_DECRYPT"] = ESDKCommitmentPolicy.FORBID_ENCRYPT_ALLOW_DECRYPT,
        ["REQUIRE_ENCRYPT_ALLOW_DECRYPT"] = ESDKCommitmentPolicy.REQUIRE_ENCRYPT_ALLOW_DECRYPT,
        ["REQUIRE_ENCRYPT_REQUIRE_DECRYPT"] = ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT,
    };

    private static readonly Dictionary<string, ESDKAlgorithmSuiteId> AlgorithmSuites = new()
    {
        ["ALG_AES_128_GCM_IV12_TAG16_NO_KDF"] = ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF,
        ["ALG_AES_192_GCM_IV12_TAG16_NO_KDF"] = ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_NO_KDF,
        ["ALG_AES_256_GCM_IV12_TAG16_NO_KDF"] = ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_NO_KDF,
        ["ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256"] = ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256,
        ["ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256"] = ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256,
        ["ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256"] = ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256,
        ["ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256"] = ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256,
        ["ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384"] = ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384,
        ["ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384"] = ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384,
        ["ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY"] = ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY,
        ["ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384"] = ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384,
    };

    // Inverts AlgorithmSuites, keyed by the ConstantClass hex value, so decrypt
    // can report the suite it determined from the message header.
    private static readonly Dictionary<string, string> AlgorithmSuiteWireNames =
        AlgorithmSuites.ToDictionary(entry => entry.Value.Value, entry => entry.Key);

    private static readonly Dictionary<string, AesWrappingAlg> AesWrappingAlgs = new()
    {
        ["ALG_AES128_GCM_IV12_TAG16"] = AesWrappingAlg.ALG_AES128_GCM_IV12_TAG16,
        ["ALG_AES192_GCM_IV12_TAG16"] = AesWrappingAlg.ALG_AES192_GCM_IV12_TAG16,
        ["ALG_AES256_GCM_IV12_TAG16"] = AesWrappingAlg.ALG_AES256_GCM_IV12_TAG16,
    };

    private static readonly Dictionary<string, PaddingScheme> PaddingSchemes = new()
    {
        ["PKCS1"] = PaddingScheme.PKCS1,
        ["OAEP_SHA1_MGF1"] = PaddingScheme.OAEP_SHA1_MGF1,
        ["OAEP_SHA256_MGF1"] = PaddingScheme.OAEP_SHA256_MGF1,
        ["OAEP_SHA384_MGF1"] = PaddingScheme.OAEP_SHA384_MGF1,
        ["OAEP_SHA512_MGF1"] = PaddingScheme.OAEP_SHA512_MGF1,
    };

    private static readonly Dictionary<string, EncryptionAlgorithmSpec> KmsRsaEncryptionAlgorithms = new()
    {
        ["RSAES_OAEP_SHA_1"] = EncryptionAlgorithmSpec.RSAES_OAEP_SHA_1,
        ["RSAES_OAEP_SHA_256"] = EncryptionAlgorithmSpec.RSAES_OAEP_SHA_256,
    };

    // Operation request decoding.

    internal static CreateClientRequest ReadCreateClientRequest(byte[] body)
    {
        var reader = NewReader(body);
        EsdkClientConfig config = null;
        ReadStructure(reader, "CreateClientRequest", (r, key) =>
        {
            if (key != "config") return false;
            config = ReadClientConfig(r);
            return true;
        });
        Require(config != null, "config");
        return new CreateClientRequest { Config = config };
    }

    internal static EncryptRequest ReadEncryptRequest(byte[] body)
    {
        var reader = NewReader(body);
        var request = new EncryptRequest();
        ReadStructure(reader, "EncryptRequest", (r, key) =>
        {
            switch (key)
            {
                case "clientId": request.ClientId = r.ReadTextString(); return true;
                case "plaintext": request.Plaintext = r.ReadByteString(); return true;
                case "encryptionContext": request.EncryptionContext = ReadStringMap(r, key); return true;
                case "algorithmSuiteId": request.AlgorithmSuiteId = ReadEnum(r, key, AlgorithmSuites); return true;
                case "frameLength": request.FrameLength = r.ReadInt64(); return true;
                default: return false;
            }
        });
        Require(request.ClientId != null, "clientId");
        Require(request.Plaintext != null, "plaintext");
        return request;
    }

    internal static DecryptRequest ReadDecryptRequest(byte[] body)
    {
        var reader = NewReader(body);
        var request = new DecryptRequest();
        ReadStructure(reader, "DecryptRequest", (r, key) =>
        {
            switch (key)
            {
                case "clientId": request.ClientId = r.ReadTextString(); return true;
                case "ciphertext": request.Ciphertext = r.ReadByteString(); return true;
                case "encryptionContext": request.EncryptionContext = ReadStringMap(r, key); return true;
                default: return false;
            }
        });
        Require(request.ClientId != null, "clientId");
        Require(request.Ciphertext != null, "ciphertext");
        return request;
    }

    // Configuration decoding.

    private static EsdkClientConfig ReadClientConfig(CborReader reader)
    {
        var config = new EsdkClientConfig();
        ReadStructure(reader, "config", (r, key) =>
        {
            switch (key)
            {
                case "commitmentPolicy": config.CommitmentPolicy = ReadEnum(r, key, CommitmentPolicies); return true;
                case "maxEncryptedDataKeys": config.MaxEncryptedDataKeys = r.ReadInt64(); return true;
                case "cmm": config.Cmm = ReadCmm(r); return true;
                default: return false;
            }
        });
        Require(config.CommitmentPolicy != null, "commitmentPolicy");
        Require(config.Cmm != null, "cmm");
        return config;
    }

    private static CmmConfig ReadCmm(CborReader reader)
    {
        var cmm = new CmmConfig();
        ReadStructure(reader, "cmm", (r, key) =>
        {
            switch (key)
            {
                case "Default":
                    cmm.Default = ReadDefaultCmm(r);
                    return true;
                case "RequiredEncryptionContext":
                    cmm.RequiredEncryptionContext = ReadRequiredEncryptionContextCmm(r);
                    return true;
                case "Caching":
                    // Members are not decoded: the server rejects the variant as unsupported.
                    if (r.PeekState() != CborReaderState.StartMap)
                    {
                        throw new FormatException("Caching: expected a CBOR map");
                    }
                    r.SkipValue();
                    cmm.CachingPresent = true;
                    return true;
                default:
                    return false;
            }
        });
        return cmm;
    }

    private static DefaultCmmConfig ReadDefaultCmm(CborReader reader)
    {
        var config = new DefaultCmmConfig();
        ReadStructure(reader, "Default", (r, key) =>
        {
            if (key != "keyring") return false;
            config.Keyring = ReadKeyring(r);
            return true;
        });
        Require(config.Keyring != null, "keyring");
        return config;
    }

    private static RequiredEncryptionContextCmmConfig ReadRequiredEncryptionContextCmm(CborReader reader)
    {
        var config = new RequiredEncryptionContextCmmConfig();
        ReadStructure(reader, "RequiredEncryptionContext", (r, key) =>
        {
            switch (key)
            {
                case "underlyingCMM": config.UnderlyingCmm = ReadCmm(r); return true;
                case "requiredEncryptionContextKeys": config.RequiredEncryptionContextKeys = ReadStringList(r, key); return true;
                default: return false;
            }
        });
        Require(config.UnderlyingCmm != null, "underlyingCMM");
        Require(config.RequiredEncryptionContextKeys != null, "requiredEncryptionContextKeys");
        return config;
    }

    private static KeyringConfig ReadKeyring(CborReader reader)
    {
        var keyring = new KeyringConfig();
        ReadStructure(reader, "keyring", (r, key) =>
        {
            switch (key)
            {
                case "AwsKms": keyring.AwsKms = ReadAwsKmsKeyring(r); return true;
                case "AwsKmsMrk": keyring.AwsKmsMrk = ReadAwsKmsKeyring(r); return true;
                case "AwsKmsMultiKeyring": keyring.AwsKmsMultiKeyring = ReadAwsKmsMultiKeyring(r); return true;
                case "AwsKmsMrkMultiKeyring": keyring.AwsKmsMrkMultiKeyring = ReadAwsKmsMultiKeyring(r); return true;
                case "AwsKmsDiscovery": keyring.AwsKmsDiscovery = ReadAwsKmsDiscoveryKeyring(r); return true;
                case "AwsKmsMrkDiscovery": keyring.AwsKmsMrkDiscovery = ReadAwsKmsMrkDiscoveryKeyring(r); return true;
                case "AwsKmsRsa": keyring.AwsKmsRsa = ReadAwsKmsRsaKeyring(r); return true;
                case "RawAes": keyring.RawAes = ReadRawAesKeyring(r); return true;
                case "RawRsa": keyring.RawRsa = ReadRawRsaKeyring(r); return true;
                case "AwsKmsHierarchical": keyring.AwsKmsHierarchical = ReadAwsKmsHierarchicalKeyring(r); return true;
                case "Multi": keyring.Multi = ReadMultiKeyring(r); return true;
                default: return false;
            }
        });
        return keyring;
    }

    private static AwsKmsKeyringConfig ReadAwsKmsKeyring(CborReader reader)
    {
        var config = new AwsKmsKeyringConfig();
        ReadStructure(reader, "AwsKms", (r, key) =>
        {
            switch (key)
            {
                case "kmsKeyId": config.KmsKeyId = r.ReadTextString(); return true;
                case "grantTokens": config.GrantTokens = ReadStringList(r, key); return true;
                default: return false;
            }
        });
        Require(config.KmsKeyId != null, "kmsKeyId");
        return config;
    }

    private static AwsKmsMultiKeyringConfig ReadAwsKmsMultiKeyring(CborReader reader)
    {
        var config = new AwsKmsMultiKeyringConfig();
        ReadStructure(reader, "AwsKmsMultiKeyring", (r, key) =>
        {
            switch (key)
            {
                case "generator": config.Generator = r.ReadTextString(); return true;
                case "kmsKeyIds": config.KmsKeyIds = ReadStringList(r, key); return true;
                default: return false;
            }
        });
        return config;
    }

    private static AwsKmsDiscoveryKeyringConfig ReadAwsKmsDiscoveryKeyring(CborReader reader)
    {
        var config = new AwsKmsDiscoveryKeyringConfig();
        ReadStructure(reader, "AwsKmsDiscovery", (r, key) =>
        {
            switch (key)
            {
                case "discoveryFilter": config.DiscoveryFilter = ReadDiscoveryFilter(r); return true;
                case "grantTokens": config.GrantTokens = ReadStringList(r, key); return true;
                default: return false;
            }
        });
        return config;
    }

    private static AwsKmsMrkDiscoveryKeyringConfig ReadAwsKmsMrkDiscoveryKeyring(CborReader reader)
    {
        var config = new AwsKmsMrkDiscoveryKeyringConfig();
        ReadStructure(reader, "AwsKmsMrkDiscovery", (r, key) =>
        {
            switch (key)
            {
                case "region": config.Region = r.ReadTextString(); return true;
                case "discoveryFilter": config.DiscoveryFilter = ReadDiscoveryFilter(r); return true;
                case "grantTokens": config.GrantTokens = ReadStringList(r, key); return true;
                default: return false;
            }
        });
        Require(config.Region != null, "region");
        return config;
    }

    private static DiscoveryFilterConfig ReadDiscoveryFilter(CborReader reader)
    {
        var filter = new DiscoveryFilterConfig();
        ReadStructure(reader, "discoveryFilter", (r, key) =>
        {
            switch (key)
            {
                case "partition": filter.Partition = r.ReadTextString(); return true;
                case "accountIds": filter.AccountIds = ReadStringList(r, key); return true;
                default: return false;
            }
        });
        Require(filter.Partition != null, "partition");
        Require(filter.AccountIds != null, "accountIds");
        return filter;
    }

    private static AwsKmsRsaKeyringConfig ReadAwsKmsRsaKeyring(CborReader reader)
    {
        var config = new AwsKmsRsaKeyringConfig();
        ReadStructure(reader, "AwsKmsRsa", (r, key) =>
        {
            switch (key)
            {
                case "kmsKeyId": config.KmsKeyId = r.ReadTextString(); return true;
                case "publicKey": config.PublicKey = r.ReadByteString(); return true;
                case "encryptionAlgorithm": config.EncryptionAlgorithm = ReadEnum(r, key, KmsRsaEncryptionAlgorithms); return true;
                case "grantTokens": config.GrantTokens = ReadStringList(r, key); return true;
                default: return false;
            }
        });
        Require(config.KmsKeyId != null, "kmsKeyId");
        return config;
    }

    private static RawAesKeyringConfig ReadRawAesKeyring(CborReader reader)
    {
        var config = new RawAesKeyringConfig();
        ReadStructure(reader, "RawAes", (r, key) =>
        {
            switch (key)
            {
                case "keyNamespace": config.KeyNamespace = r.ReadTextString(); return true;
                case "keyName": config.KeyName = r.ReadTextString(); return true;
                case "wrappingKey": config.WrappingKey = r.ReadByteString(); return true;
                case "wrappingAlg": config.WrappingAlg = ReadEnum(r, key, AesWrappingAlgs); return true;
                default: return false;
            }
        });
        Require(config.KeyNamespace != null, "keyNamespace");
        Require(config.KeyName != null, "keyName");
        Require(config.WrappingKey != null, "wrappingKey");
        Require(config.WrappingAlg != null, "wrappingAlg");
        return config;
    }

    private static RawRsaKeyringConfig ReadRawRsaKeyring(CborReader reader)
    {
        var config = new RawRsaKeyringConfig();
        ReadStructure(reader, "RawRsa", (r, key) =>
        {
            switch (key)
            {
                case "keyNamespace": config.KeyNamespace = r.ReadTextString(); return true;
                case "keyName": config.KeyName = r.ReadTextString(); return true;
                case "paddingScheme": config.PaddingScheme = ReadEnum(r, key, PaddingSchemes); return true;
                case "publicKey": config.PublicKey = r.ReadByteString(); return true;
                case "privateKey": config.PrivateKey = r.ReadByteString(); return true;
                default: return false;
            }
        });
        Require(config.KeyNamespace != null, "keyNamespace");
        Require(config.KeyName != null, "keyName");
        Require(config.PaddingScheme != null, "paddingScheme");
        return config;
    }

    private static AwsKmsHierarchicalKeyringConfig ReadAwsKmsHierarchicalKeyring(CborReader reader)
    {
        var config = new AwsKmsHierarchicalKeyringConfig();
        var sawTtlSeconds = false;
        ReadStructure(reader, "AwsKmsHierarchical", (r, key) =>
        {
            switch (key)
            {
                case "branchKeyId": config.BranchKeyId = r.ReadTextString(); return true;
                case "keyStoreTableName": config.KeyStoreTableName = r.ReadTextString(); return true;
                case "logicalKeyStoreName": config.LogicalKeyStoreName = r.ReadTextString(); return true;
                case "kmsKeyArn": config.KmsKeyArn = r.ReadTextString(); return true;
                case "ttlSeconds": config.TtlSeconds = r.ReadInt64(); sawTtlSeconds = true; return true;
                default: return false;
            }
        });
        Require(config.BranchKeyId != null, "branchKeyId");
        Require(config.KeyStoreTableName != null, "keyStoreTableName");
        Require(config.LogicalKeyStoreName != null, "logicalKeyStoreName");
        Require(config.KmsKeyArn != null, "kmsKeyArn");
        Require(sawTtlSeconds, "ttlSeconds");
        return config;
    }

    private static MultiKeyringConfig ReadMultiKeyring(CborReader reader)
    {
        var config = new MultiKeyringConfig();
        ReadStructure(reader, "Multi", (r, key) =>
        {
            switch (key)
            {
                case "generator":
                    config.Generator = ReadKeyring(r);
                    return true;
                case "childKeyrings":
                    var children = new List<KeyringConfig>();
                    r.ReadStartArray();
                    while (r.PeekState() != CborReaderState.EndArray)
                    {
                        children.Add(ReadKeyring(r));
                    }
                    r.ReadEndArray();
                    config.ChildKeyrings = children;
                    return true;
                default:
                    return false;
            }
        });
        Require(config.ChildKeyrings != null, "childKeyrings");
        return config;
    }

    // Response / error encoding.

    internal static byte[] WriteCreateClientResponse(CreateClientResponse response)
    {
        var writer = new CborWriter();
        writer.WriteStartMap(1);
        writer.WriteTextString("clientId");
        writer.WriteTextString(response.ClientId);
        writer.WriteEndMap();
        return writer.Encode();
    }

    internal static byte[] WriteEncryptResponse(EncryptResponse response)
    {
        var writer = new CborWriter();
        writer.WriteStartMap(1);
        writer.WriteTextString("ciphertext");
        writer.WriteByteString(response.Ciphertext);
        writer.WriteEndMap();
        return writer.Encode();
    }

    internal static byte[] WriteDecryptResponse(DecryptResponse response)
    {
        var includeContext = response.EncryptionContext is { Count: > 0 };
        string suiteName = null;
        if (response.AlgorithmSuiteId != null)
        {
            AlgorithmSuiteWireNames.TryGetValue(response.AlgorithmSuiteId.Value, out suiteName);
        }
        var writer = new CborWriter();
        writer.WriteStartMap(1 + (includeContext ? 1 : 0) + (suiteName != null ? 1 : 0));
        writer.WriteTextString("plaintext");
        writer.WriteByteString(response.Plaintext);
        if (includeContext)
        {
            writer.WriteTextString("encryptionContext");
            writer.WriteStartMap(response.EncryptionContext.Count);
            foreach (var entry in response.EncryptionContext)
            {
                writer.WriteTextString(entry.Key);
                writer.WriteTextString(entry.Value);
            }
            writer.WriteEndMap();
        }
        if (suiteName != null)
        {
            writer.WriteTextString("algorithmSuiteId");
            writer.WriteTextString(suiteName);
        }
        writer.WriteEndMap();
        return writer.Encode();
    }

    internal static byte[] WriteError(ServerErrorException error)
    {
        var writer = new CborWriter();
        writer.WriteStartMap(2);
        writer.WriteTextString("__type");
        writer.WriteTextString(error.TypeId);
        writer.WriteTextString("message");
        writer.WriteTextString(error.Message);
        writer.WriteEndMap();
        return writer.Encode();
    }

    // CBOR member helpers.

    private static CborReader NewReader(byte[] body) => new(body, CborConformanceMode.Lax);

    // Walks a structure's map entries: a CBOR null value counts as absent, and
    // members the handler does not recognize are skipped.
    private static void ReadStructure(CborReader reader, string what, Func<CborReader, string, bool> onMember)
    {
        if (reader.PeekState() != CborReaderState.StartMap)
        {
            throw new FormatException($"{what}: expected a CBOR map, got {reader.PeekState()}");
        }
        reader.ReadStartMap();
        while (reader.PeekState() != CborReaderState.EndMap)
        {
            var key = reader.ReadTextString();
            if (reader.PeekState() == CborReaderState.Null)
            {
                reader.ReadNull();
                continue;
            }
            if (!onMember(reader, key))
            {
                reader.SkipValue();
            }
        }
        reader.ReadEndMap();
    }

    private static void Require(bool present, string member)
    {
        if (!present)
        {
            throw new FormatException($"missing required member {member}");
        }
    }

    private static T ReadEnum<T>(CborReader reader, string key, Dictionary<string, T> values)
    {
        var name = reader.ReadTextString();
        if (!values.TryGetValue(name, out var value))
        {
            throw new FormatException($"member {key}: unknown enum value \"{name}\"");
        }
        return value;
    }

    private static Dictionary<string, string> ReadStringMap(CborReader reader, string key)
    {
        if (reader.PeekState() != CborReaderState.StartMap)
        {
            throw new FormatException($"member {key}: expected a CBOR map, got {reader.PeekState()}");
        }
        var map = new Dictionary<string, string>();
        reader.ReadStartMap();
        while (reader.PeekState() != CborReaderState.EndMap)
        {
            map[reader.ReadTextString()] = reader.ReadTextString();
        }
        reader.ReadEndMap();
        return map;
    }

    private static List<string> ReadStringList(CborReader reader, string key)
    {
        if (reader.PeekState() != CborReaderState.StartArray)
        {
            throw new FormatException($"member {key}: expected a CBOR array, got {reader.PeekState()}");
        }
        var list = new List<string>();
        reader.ReadStartArray();
        while (reader.PeekState() != CborReaderState.EndArray)
        {
            list.Add(reader.ReadTextString());
        }
        reader.ReadEndArray();
        return list;
    }
}
