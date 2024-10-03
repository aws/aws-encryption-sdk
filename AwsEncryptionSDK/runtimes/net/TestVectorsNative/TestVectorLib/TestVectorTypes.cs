// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using Newtonsoft.Json;

namespace TestVectors
{
    // TODO Need to use some enums for various fields, possibly subtypes to represent RSA vs AES having different params?
    public class Key {
        [JsonProperty("decrypt")]
        public bool Decrypt { get; set; }
        [JsonProperty("encrypt")]
        public bool Encrypt { get; set; }
        [JsonRequired]
        [JsonProperty("type")]
        public string Type { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("key-id")]
        public string Id { get; set; } = null!;
        [JsonProperty("algorithm")]
        public string? Algorithm { get; set; }
        [JsonProperty("bits")]
        public ushort Bits { get; set; }
        [JsonProperty("encoding")]
        public string? Encoding { get; set; }
        [JsonProperty("material")]
        public string? Material { get; set; }
        [JsonProperty("branchKeyVersion")]
        public string? BranchKeyVersion { get; set; }
        [JsonProperty("branchKey")]
        public string? BranchKey { get; set; }
        [JsonProperty("beaconKey")]
        public string? BeaconKey { get; set; }
    }

    public class KeyManifest
    {
        [JsonRequired]
        [JsonProperty("manifest")]
        public ManifestMeta Meta { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("keys")]
        public Dictionary<string, Key> Keys { get; set; } = null!;
    }

    // TODO Rename? Need to use some enums for various fields, possibly subtypes to represent RSA vs AES having different params?
    public class MasterKey {
        [JsonProperty("type")]
        public string? Type { get; set; }
        [JsonProperty("key")]
        public string? Key { get; set; }
        [JsonProperty("provider-id")]
        public string? ProviderId { get; set; }
        [JsonProperty("encryption-algorithm")]
        public string? EncryptionAlgorithm { get; set; }
        [JsonProperty("padding-algorithm")]
        public string? PaddingAlgorithm { get; set; }
        [JsonProperty("padding-hash")]
        public string? PaddingHash { get; set; }
        [JsonProperty("default-mrk-region")]
        public string? DefaultMrkRegion { get; set; }
        [JsonProperty("aws-kms-discovery-filter")]
        public DiscoveryFilter? AwsKmsDiscoveryFilter { get; set; }
    }

    public class DiscoveryFilter
    {
        [JsonProperty("partition")]
        public string? Partition { get; set; }
        [JsonProperty("account-ids")]
        public IList<string>? AccountIds { get; set; }
    }

    public class DecryptVector {
        [JsonProperty("description")]
        public string? Description { get; set; }
        [JsonRequired]
        [JsonProperty("ciphertext")]
        public string Ciphertext { get; set; } = null!;
        [JsonProperty("master-keys")]
        public IList<MasterKey>? MasterKeys { get; set; }
        [JsonProperty("cmm")]
        public string? CMM { get; set; }
        [JsonProperty("encryption-context")]
        public Dictionary<string, string>? EncryptionContext { get; set; }
        [JsonProperty("result")]
        public DecryptResult? Result { get; set; }
        [JsonProperty("decryption-method")]
        public string? DecryptionMethod { get; set; }
    }

    public class DecryptResult
    {
        [JsonProperty("output")]
        public DecryptOutput? Output { get; set; }
        [JsonProperty("error")]
        public DecryptError? Error { get; set; }
    }

    public class DecryptOutput
    {
        [JsonProperty("plaintext")]
        public string? Plaintext { get; set; }
    }
    public class DecryptError
    {
        [JsonProperty("error-description")]
        public string? ErrorMessage { get; set; }
    }

    public class DecryptManifest {
        [JsonRequired]
        [JsonProperty("manifest")]
        public ManifestMeta Meta { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("client")]
        public Client Client { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("keys")]
        public string KeysUri { get; set; }= null!;
        [JsonRequired]
        [JsonProperty("tests")]
        public Dictionary<string, DecryptVector> VectorMap { get; set; } = null!;
    }

    public class EncryptScenario
    {
        [JsonRequired]
        [JsonProperty("plaintext")]
        public string PlaintextName { get; set; } = null!;
        /// <summary>
        /// Hex string of algorithm suite ID
        /// </summary>
        [JsonRequired]
        [JsonProperty("algorithm")]
        public string Algorithm { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("frame-size")]
        public uint FrameSize { get; set; }
        [JsonRequired]
        [JsonProperty("encryption-context")]
        public Dictionary<string, string> EncryptionContext { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("master-keys")]
        public IList<MasterKey> MasterKeys { get; set; } = null!;
        [JsonProperty("cmm")]
        public string? CMM { get; set; }
    }

    public class EncryptVector
    {
        [JsonRequired]
        [JsonProperty("encryption-scenario")]
        public EncryptScenario Scenario { get; set; } = null!;

        // TODO: each of these three are currently unused, but we need to model them
        // so that we can successfully parse the manifest
        [JsonProperty("decryption-method")]
        public string? DecryptionMethod { get; set; }
        [JsonProperty("result")]
        public DecryptResult? Result { get; set; }
        [JsonProperty("decryption-master-keys")]
        public IList<MasterKey>? DecryptionMasterKeys { get; set; }

        // TODO create tampered messages
        // 'dynamic' type because we sometimes set this as a string
        // and sometimes set it as an object. Will need to figure this
        // out when we support tampered messages
        [JsonProperty("tampering")]
        public dynamic? Tampering { get; set; }
    }

    public class EncryptManifest
    {
        [JsonRequired]
        [JsonProperty("manifest")]
        public ManifestMeta Meta { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("keys")]
        public string KeysUri { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("plaintexts")]
        public Dictionary<string, uint> PlaintextSizes { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("tests")]
        public Dictionary<string, EncryptVector> VectorMap { get; set; } = null!;
    }

    public class ManifestMeta
    {
        [JsonRequired]
        [JsonProperty("type")]
        public string Type { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("version")]
        public int Version { get; set; }
    }

    public class Client
    {
        [JsonRequired]
        [JsonProperty("name")]
        public string Name { get; set; } = null!;
        [JsonRequired]
        [JsonProperty("version")]
        public string Version { get; set; } = null!;
    }
}
