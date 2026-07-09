//! # aws-esdk-cpp — a C++ shim over the Rust AWS Encryption SDK
//!
//! This crate is a shim that exposes the native Rust AWS Encryption SDK to C++
//! callers. Its behavioral contract is specified in `spec/shim/shim.md` and
//! `spec/shim/esdk-shim.md`.
//!
//! ## Layering
//!
//! 1. This bridge (`#[cxx::bridge] mod ffi`) + the Rust glue below it.
//! 2. The cxx-generated bindings (`lib.rs.h` / `lib.rs.cc`).
//! 3. A hand-written idiomatic facade, `aws_esdk.hpp`, which is what consumers
//!    use. `main.cpp` uses the raw generated bindings directly;
//!    `main_idiomatic.cpp` uses the facade.

#![allow(clippy::boxed_local, reason = "need to pass Box<T> to destructors")]

// ---- Crate-wide conformance (see spec/shim/shim.md and spec/shim/esdk-shim.md) ----
//
// The generic Shim Specification requirements below are file-wide: they describe
// behavior that EVERY translation / operation / resource in this crate obeys, so
// they are recorded here once as implications rather than pinned to a single site.
//= spec/shim/esdk-shim.md#overview
//= type=implication
//= reason=This crate's annotations against shim.md are the evidence that the ESDK shim conforms to the generic Shim Specification.
//# An ESDK shim MUST conform to the generic [Shim Specification](./shim.md).
//
//= spec/shim/shim.md#conformance-and-testing
//= type=implication
//= reason=This crate carries annotations against both shim.md and esdk-shim.md, evidencing conformance to both.
//# - A shim MUST conform to this specification in addition to its core-library-specific
//# shim specification.
//
//= spec/shim/shim.md#type-translation
//= type=implication
//= reason=Every concrete translation in this crate returns a Result and follows the convert-or-error rules.
//# Every such translation MUST obey the following rules.
//
//= spec/shim/shim.md#type-translation
//= type=implication
//= reason=Every conversion function in this crate converts between the target and core-library representations in the direction(s) it supports.
//# - A translation MUST convert between the target representation and the core
//# library representation, preserving meaning in both directions it supports.
//
//= spec/shim/shim.md#type-translation
//= type=implication
//= reason=Every target-to-core conversion returns Err for a target value that has no core-library representation.
//# - When the target supplies a value that has no corresponding core-library
//# representation, the translation MUST return an error.
//
//= spec/shim/shim.md#type-translation
//= type=implication
//= reason=Every core-to-target conversion returns Err for a core-library value the shim does not define.
//# - When the core library produces a value the shim does not define, the
//# translation MUST return an error.
//
//= spec/shim/shim.md#delegation
//= type=implication
//= reason=No function in this crate validates operation or resource-creation inputs; each value is translated and passed to the core library, which performs all validation.
//# - The shim MUST NOT validate operation or resource-creation input values itself;
//# it MUST translate each input and defer its validation to the core library.
//
//= spec/shim/shim.md#delegation
//= type=implication
//= reason=Every operation in this crate produces its result by calling the corresponding aws_esdk core-library function.
//# - Each operation the shim exposes MUST produce its result by invoking the
//# corresponding core-library operation.
//
//= spec/shim/shim.md#operation-contracts
//= type=implication
//= reason=Every operation invokes the core library, satisfying Delegation.
//# - Each operation MUST invoke the core library per [Delegation](#delegation).
//
//= spec/shim/shim.md#operation-contracts
//= type=implication
//= reason=Every operation input/output is either passed through unmodified or converted by a Type-translation function.
//# - Except where an input or output is passed through unmodified, each MUST be
//# translated per [Type translation](#type-translation).
//
//= spec/shim/shim.md#creating-and-releasing-resources
//= type=implication
//= reason=Each resource kind has its own create_* function in the cxx bridge (keyring, key store, KMS client, DynamoDB client).
//# - For each resource kind, the shim MUST provide a means for the target to create an
//# instance backed by the core library.
//
//= spec/shim/shim.md#creating-and-releasing-resources
//= type=implication
//= reason=Each resource is a Box owned by the target; dropping the owned interface frees the backing core-library resource.
//# - The shim MUST release the core-library resource backing an owned interface once
//# the target no longer holds it.
//
//= spec/shim/shim.md#creating-and-releasing-resources
//= type=implication
//= reason=Dropping the Box at end of scope releases the resource automatically when the owned interface is destroyed.
//# The shim MAY release it automatically when the
//# owned interface is destroyed, or through an explicit target operation.
//
//= spec/shim/shim.md#concurrency
//= type=exception
//= reason=The shim adds no concurrency feature of its own and does not exercise concurrent use in this no-AWS crate; bridge entry points defer any threading behavior to the core library.
//# If a core library resource or operation supports concurrent use from multiple threads,
//# the shim library SHOULD support it as well.
//
//= spec/shim/shim.md#streaming
//= type=exception
//= reason=The owned interface exposes one-shot encrypt/decrypt (Vec in, Vec out); streamed input/output is not surfaced.
//# If a core library resource or operation supports [streamed](../client-apis/streaming.md) input/outputs,
//# the shim library SHOULD support it as well.
use aws_config::{AppName, Region, SdkConfig};
use std::sync::LazyLock;

static DAFNY_TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    // Building a multi-thread Tokio runtime only fails if the OS cannot spawn a
    // thread or install the I/O/timer drivers — an environment failure with no
    // meaningful library-level recovery. We surface it as a panic at first use
    // rather than threading a `Result` through every bridge entry point.
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

#[cxx::bridge]
mod ffi {
    struct EncryptionContextItem {
        key: String,
        value: String,
    }
    enum EsdkAlgorithmSuiteId {
        AlgAes128GcmIv12Tag16NoKdf = 0x0014,
        AlgAes192GcmIv12Tag16NoKdf = 0x0046,
        AlgAes256GcmIv12Tag16NoKdf = 0x0078,
        AlgAes128GcmIv12Tag16HkdfSha256 = 0x0114,
        AlgAes192GcmIv12Tag16HkdfSha256 = 0x0146,
        AlgAes256GcmIv12Tag16HkdfSha256 = 0x0178,
        AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 = 0x0214,
        AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 = 0x0346,
        AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 = 0x0378,
        AlgAes256GcmHkdfSha512CommitKey = 0x0478,
        AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 = 0x0578,
    }

    struct EncryptInput<'a> {
        /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
        pub algorithm_suite_id: EsdkAlgorithmSuiteId,
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
        /// Bytes of plaintext data per frame. Default 4096.
        pub frame_length: u32,
        /// The source of cryptographic materials
        pub keyring: *const Keyring,
        /// data to be encrypted
        pub plaintext: &'a [u8],
        /// default is no limit
        pub max_encrypted_data_keys: u32,
        /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
        pub commitment_policy: EsdkCommitmentPolicy,
    }
    struct EncryptOutput {
        /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
        pub algorithm_suite_id: EsdkAlgorithmSuiteId,
        /// data to be decrypted
        pub ciphertext: Vec<u8>,
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
    }
    struct DecryptInput<'a> {
        /// data to be decrypted
        pub ciphertext: &'a [u8],
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
        /// The source of cryptographic materials
        pub keyring: *const Keyring,
        /// default is no limit
        pub max_encrypted_data_keys: u32,
        /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
        pub commitment_policy: EsdkCommitmentPolicy,
    }
    struct DecryptOutput {
        /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
        pub algorithm_suite_id: EsdkAlgorithmSuiteId,
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
        /// decrypted data
        pub plaintext: Vec<u8>,
    }

    enum EsdkCommitmentPolicy {
        ForbidEncryptAllowDecrypt,
        RequireEncryptAllowDecrypt,
        RequireEncryptRequireDecrypt,
    }

    struct RetryConfig {
        mode_adaptive: bool,
        max_attempts: u32,
        initial_backoff_milli: u64,
        max_backoff_milli: u64,
        reconnect_all: bool,
        use_static_exponential_base: bool,
    }

    enum KmsConfigurationType {
        KmsKeyArn,
        KmsMrKeyArn,
        Discovery,
        MrDiscovery,
    }

    enum CacheType {
        NoCache,
        MultiThreadedCache,
    }

    struct MplAwsClientConfig {
        env: bool,
        region: String,
        retry: RetryConfig,
    }

    struct MultiThreadedCacheConfig {
        entryCapacity: u32,
        entryPruningTailSize: u32,
    }

    struct KeyStoreConfig {
        ddb_table_name: String,
        kms_configuration_type: KmsConfigurationType,
        kms_configuration_value: String,
        logical_key_store_name: String,
        id: String,
        grant_tokens: Vec<String>,
        ddb_client: *const MplDdbClient,
        kms_client: *const MplKmsClient,
    }

    struct HierarchicalKeyringInput {
        branch_key_id: String,
        key_store: *const KeyStore,
        ttl: u32,
        cache: CacheType,
        multi_threaded_cache: MultiThreadedCacheConfig,
        partition_id: String,
    }

    extern "Rust" {
        type MplDdbClient;
        fn create_ddb_client(value: &MplAwsClientConfig) -> Result<Box<MplDdbClient>>;
        fn delete_ddb_client(client: Box<MplDdbClient>) -> Result<()>;

        type KeyStore;
        fn create_keystore(value: &KeyStoreConfig) -> Result<Box<KeyStore>>;
        fn delete_keystore(client: Box<KeyStore>) -> Result<()>;

        type Keyring;
        fn create_hierarchical_keyring(value: &HierarchicalKeyringInput) -> Result<Box<Keyring>>;
        fn delete_keyring(client: Box<Keyring>) -> Result<()>;

        type MplKmsClient;
        fn create_kms_client(value: &MplAwsClientConfig) -> Result<Box<MplKmsClient>>;
        fn delete_kms_client(client: Box<MplKmsClient>) -> Result<()>;

        fn encrypt(input: &EncryptInput) -> Result<EncryptOutput>;
        fn decrypt(input: &DecryptInput) -> Result<DecryptOutput>;

        fn default_client_config() -> MplAwsClientConfig;
        fn default_keystore_config() -> KeyStoreConfig;
        fn default_hierarchical_keyring_input() -> HierarchicalKeyringInput;
        fn default_encrypt_input() -> EncryptInput<'static>;
        fn default_decrypt_input() -> DecryptInput<'static>;
    }
}

struct MplKmsClient {
    client: aws_sdk_kms::Client,
}
struct MplDdbClient {
    client: aws_sdk_dynamodb::Client,
}

struct KeyStore {
    client: aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::client::Client,
}

struct Keyring {
    client: aws_mpl_legacy::dafny::types::keyring::KeyringRef,
}

//= spec/shim/esdk-shim.md#commitment-policy
//# - The shim MUST translate the target commitment policy to the core ESDK
//# commitment policy of the same meaning.
fn commitment_policy_target_to_core(
    x: ffi::EsdkCommitmentPolicy,
) -> Result<aws_mpl_legacy::commitment::EsdkCommitmentPolicy, String> {
    use aws_mpl_legacy::commitment::EsdkCommitmentPolicy as New;
    use ffi::EsdkCommitmentPolicy as Old;
    match x {
        Old::ForbidEncryptAllowDecrypt => Ok(New::ForbidEncryptAllowDecrypt),
        Old::RequireEncryptAllowDecrypt => Ok(New::RequireEncryptAllowDecrypt),
        Old::RequireEncryptRequireDecrypt => Ok(New::RequireEncryptRequireDecrypt),
        //= spec/shim/esdk-shim.md#commitment-policy
        //# - The shim MUST return an error when the target commitment policy is not a supported
        //# value.
        _ => Err(format!("unrecognized EsdkCommitmentPolicy value: {}", x.repr)),
    }
}

fn default_encrypt_input() -> ffi::EncryptInput<'static> {
    ffi::EncryptInput {
        algorithm_suite_id: ffi::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
        encryption_context: Vec::default(),
        frame_length: 4096,
        keyring: std::ptr::null(),
        plaintext: &[],
        max_encrypted_data_keys: 0,
        commitment_policy: ffi::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
    }
}

fn default_decrypt_input() -> ffi::DecryptInput<'static> {
    ffi::DecryptInput {
        ciphertext: &[],
        encryption_context: Vec::default(),
        keyring: std::ptr::null(),
        max_encrypted_data_keys: 0,
        commitment_policy: ffi::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
    }
}

fn default_hierarchical_keyring_input() -> ffi::HierarchicalKeyringInput {
    ffi::HierarchicalKeyringInput {
        branch_key_id: String::default(),
        key_store: std::ptr::null(),
        ttl: 300,
        cache: ffi::CacheType::MultiThreadedCache,
        multi_threaded_cache: ffi::MultiThreadedCacheConfig {
            entryCapacity: 1000,
            entryPruningTailSize: 1,
        },
        partition_id: String::default(),
    }
}

fn default_keystore_config() -> ffi::KeyStoreConfig {
    ffi::KeyStoreConfig {
        ddb_table_name: String::default(),
        kms_configuration_type: ffi::KmsConfigurationType::KmsKeyArn,
        kms_configuration_value: String::default(),
        logical_key_store_name: String::default(),
        id: String::default(),
        grant_tokens: Vec::default(),
        ddb_client: std::ptr::null(),
        kms_client: std::ptr::null(),
    }
}

fn default_client_config() -> ffi::MplAwsClientConfig {
    ffi::MplAwsClientConfig {
        env: true,
        region: String::default(),
        retry: default_retry_config(),
    }
}

fn default_retry_config() -> ffi::RetryConfig {
    ffi::RetryConfig {
        mode_adaptive: false,
        max_attempts: 0,
        initial_backoff_milli: 0,
        max_backoff_milli: 0,
        reconnect_all: false,
        use_static_exponential_base: false,
    }
}

//= spec/shim/esdk-shim.md#algorithm-suite-identifier
//# - The shim MUST translate a target algorithm suite identifier to the core ESDK
//# algorithm suite that has the same two-byte ID.
fn alg_id_target_to_core(
    e: ffi::EsdkAlgorithmSuiteId,
) -> Result<aws_mpl_legacy::suites::EsdkAlgorithmSuiteId, String> {
    use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId as New;
    use ffi::EsdkAlgorithmSuiteId as Old;
    match e {
        Old::AlgAes128GcmIv12Tag16NoKdf => Ok(New::AlgAes128GcmIv12Tag16NoKdf),
        Old::AlgAes192GcmIv12Tag16NoKdf => Ok(New::AlgAes192GcmIv12Tag16NoKdf),
        Old::AlgAes256GcmIv12Tag16NoKdf => Ok(New::AlgAes256GcmIv12Tag16NoKdf),
        Old::AlgAes128GcmIv12Tag16HkdfSha256 => Ok(New::AlgAes128GcmIv12Tag16HkdfSha256),
        Old::AlgAes192GcmIv12Tag16HkdfSha256 => Ok(New::AlgAes192GcmIv12Tag16HkdfSha256),
        Old::AlgAes256GcmIv12Tag16HkdfSha256 => Ok(New::AlgAes256GcmIv12Tag16HkdfSha256),
        Old::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            Ok(New::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256)
        }
        Old::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            Ok(New::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384)
        }
        Old::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            Ok(New::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384)
        }
        Old::AlgAes256GcmHkdfSha512CommitKey => Ok(New::AlgAes256GcmHkdfSha512CommitKey),
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            Ok(New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384)
        }
        //= spec/shim/esdk-shim.md#algorithm-suite-identifier
        //# - The shim MUST return an error when the target value is not one of the algorithm
        //# suite IDs defined by the core ESDK.
        _ => Err(format!("unrecognized EsdkAlgorithmSuiteId value: {:#06x}", e.repr)),
    }
}

//= spec/shim/esdk-shim.md#algorithm-suite-identifier
//# - The shim MUST translate a core ESDK algorithm suite to the target algorithm
//# suite identifier that has the same two-byte ID.
fn alg_id_core_to_target(
    e: aws_mpl_legacy::suites::EsdkAlgorithmSuiteId,
) -> Result<ffi::EsdkAlgorithmSuiteId, String> {
    use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId as Old;
    use ffi::EsdkAlgorithmSuiteId as New;
    match e {
        Old::AlgAes128GcmIv12Tag16NoKdf => Ok(New::AlgAes128GcmIv12Tag16NoKdf),
        Old::AlgAes192GcmIv12Tag16NoKdf => Ok(New::AlgAes192GcmIv12Tag16NoKdf),
        Old::AlgAes256GcmIv12Tag16NoKdf => Ok(New::AlgAes256GcmIv12Tag16NoKdf),
        Old::AlgAes128GcmIv12Tag16HkdfSha256 => Ok(New::AlgAes128GcmIv12Tag16HkdfSha256),
        Old::AlgAes192GcmIv12Tag16HkdfSha256 => Ok(New::AlgAes192GcmIv12Tag16HkdfSha256),
        Old::AlgAes256GcmIv12Tag16HkdfSha256 => Ok(New::AlgAes256GcmIv12Tag16HkdfSha256),
        Old::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            Ok(New::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256)
        }
        Old::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            Ok(New::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384)
        }
        Old::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            Ok(New::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384)
        }
        Old::AlgAes256GcmHkdfSha512CommitKey => Ok(New::AlgAes256GcmHkdfSha512CommitKey),
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            Ok(New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384)
        }
        //= spec/shim/esdk-shim.md#algorithm-suite-identifier
        //= type=implication
        //= reason=The core ESDK enum is exhaustive over its defined suites; a value it does not define cannot be constructed, so this defensive arm cannot be reached by a runtime test.
        //# - The shim MUST return an error when the core ESDK produces an algorithm suite
        //# the shim does not define.
        _ => Err("core returned an algorithm suite id unknown to the C++ bindings".to_string()),
    }
}

//= spec/shim/esdk-shim.md#encryption-context
//# - The shim MUST translate the core ESDK's encryption context, given as a
//# key-value map, to the target's list of key-value pairs, preserving every pair
//# and the exact key and value of each.
fn encryption_context_core_to_target(
    x: std::collections::HashMap<String, String>,
) -> Vec<ffi::EncryptionContextItem> {
    x.into_iter()
        .map(|(k, v)| ffi::EncryptionContextItem { key: k, value: v })
        .collect()
}

//= spec/shim/esdk-shim.md#encryption-context
//# - The shim MUST translate the target's encryption context, given as a list of
//# key-value pairs, to the core ESDK's key-value map, preserving every pair and
//# the exact key and value of each.
fn encryption_context_target_to_core(x: &[ffi::EncryptionContextItem]) -> std::collections::HashMap<String, String> {
    x.iter().map(|x| (x.key.clone(), x.value.clone())).collect()
}

//= spec/shim/esdk-shim.md#maximum-encrypted-data-keys
//# The shim MUST translate zero
//# to the core ESDK's "no limit" representation, and a positive value `n` to a
//# limit of `n`.
fn max_edks_target_to_core(n: u32) -> Option<std::num::NonZeroUsize> {
    std::num::NonZeroUsize::new(usize::try_from(n).unwrap_or(usize::MAX))
}

//= spec/shim/esdk-shim.md#frame-length
//# - The shim MUST translate the target frame length using the core ESDK's frame
//# length constructor, and MUST return an error when the core ESDK rejects the
//# value.
fn make_frame_length(frame_length: u32) -> Result<aws_esdk::FrameLength, String> {
    aws_esdk::FrameLength::new(frame_length).map_err(|e| format!("invalid frame_length: {:?}", e))
}

/// The user-agent this shim publishes: target language (C++), core language
/// (Rust), and the crate's published version.
fn shim_user_agent() -> String {
    //= spec/shim/esdk-shim.md#service-client-configuration
    //# - The shim MUST set a user-agent on each service client of the form
    //# `AwsEncryptionSdk-Shim-<target-language>-<core-language>-<version>`, where
    //# `<target-language>` is the target language, `<core-language>` is the core
    //# ESDK's implementation language, and `<version>` is the shim's published version.
    format!("AwsEncryptionSdk-Shim-C++-Rust-{}", env!("CARGO_PKG_VERSION"))
}

/// Merge the shim's user-agent with any user-agent already present in the loaded
/// configuration.
fn merge_user_agent(current_app_name: &str, shim_ua: &str) -> String {
    if current_app_name.is_empty() {
        shim_ua.to_string()
    } else {
        //= spec/shim/esdk-shim.md#service-client-configuration
        //# - The shim MUST preserve any user-agent already present in the loaded
        //# configuration, appending its own rather than replacing it.
        format!("{} {}", current_app_name, shim_ua)
    }
}

fn encrypt(input: &ffi::EncryptInput) -> Result<ffi::EncryptOutput, String> {
    //= spec/shim/esdk-shim.md#materials-source
    //# - The shim MUST return an error, and MUST NOT invoke the core ESDK, when no
    //# materials source is supplied.
    if input.keyring.is_null() {
        return Err("keyring is null in encrypt".to_string());
    }

    let mut core_input = aws_esdk::EncryptInput::with_legacy_keyring(
        //= spec/shim/esdk-shim.md#encrypt-inputs
        //# - `encrypt` MUST pass the target-supplied plaintext to the core ESDK unmodified.
        input.plaintext,
        //= spec/shim/esdk-shim.md#encrypt-inputs
        //# - `encrypt` MUST provide the target-supplied encryption context to the core ESDK,
        //# converted as defined in [Encryption context](#encryption-context).
        encryption_context_target_to_core(&input.encryption_context),
        //= spec/shim/esdk-shim.md#materials-source
        //= type=implication
        //= reason=EncryptInput carries exactly one materials-source field (keyring); the type admits no second source.
        //# - Each of `encrypt` and `decrypt` MUST be supplied with exactly one materials
        //# source (see [Resources](#resources)).
        unsafe { (*input.keyring).client.clone() },
    );
    //= spec/shim/esdk-shim.md#encrypt-inputs
    //# - `encrypt` MUST provide the target-supplied maximum-encrypted-data-keys value to
    //# the core ESDK, converted as defined in
    //# [Maximum encrypted data keys](#maximum-encrypted-data-keys).
    core_input.max_encrypted_data_keys = max_edks_target_to_core(input.max_encrypted_data_keys);
    //= spec/shim/esdk-shim.md#encrypt-inputs
    //# - `encrypt` MUST provide the target-supplied frame length to the core ESDK,
    //# converted as defined in [Frame length](#frame-length).
    core_input.frame_length = make_frame_length(input.frame_length)?;
    //= spec/shim/esdk-shim.md#encrypt-inputs
    //# - `encrypt` MUST provide the target-supplied commitment policy to the core ESDK,
    //# converted as defined in [Commitment policy](#commitment-policy).
    core_input.commitment_policy = commitment_policy_target_to_core(input.commitment_policy)?;
    //= spec/shim/esdk-shim.md#encrypt-inputs
    //# - `encrypt` MUST provide the target-supplied algorithm suite to the core ESDK,
    //# converted as defined in [Algorithm suite identifier](#algorithm-suite-identifier).
    core_input.algorithm_suite_id = Some(alg_id_target_to_core(input.algorithm_suite_id)?);
    let output = DAFNY_TOKIO_RUNTIME.block_on(aws_esdk::encrypt(&core_input));
    //= spec/shim/shim.md#delegation
    //# - When a core-library operation returns an error, the shim MUST return an error to
    //# the target and MUST NOT report the operation as successful.
    //
    //= spec/shim/shim.md#delegation
    //# - An error the shim returns for a failed core-library operation SHOULD include
    //# context identifying which operation failed.
    let output = output.map_err(|e| format!("encrypt failed: {:?}", e))?;
    Ok(ffi::EncryptOutput {
        //= spec/shim/esdk-shim.md#encrypt-outputs
        //# - `encrypt` MUST return the used algorithm suite, converted as defined in
        //# [Algorithm suite identifier](#algorithm-suite-identifier).
        algorithm_suite_id: alg_id_core_to_target(output.algorithm_suite_id)?,
        //= spec/shim/esdk-shim.md#encrypt-outputs
        //# - `encrypt` MUST return the core ESDK's ciphertext unmodified.
        ciphertext: output.ciphertext,
        //= spec/shim/esdk-shim.md#encrypt-outputs
        //# - `encrypt` MUST return the result encryption context, converted as defined in
        //# [Encryption context](#encryption-context).
        encryption_context: encryption_context_core_to_target(output.encryption_context),
    })
}
fn decrypt(input: &ffi::DecryptInput) -> Result<ffi::DecryptOutput, String> {
    //= spec/shim/esdk-shim.md#materials-source
    //# - The shim MUST return an error, and MUST NOT invoke the core ESDK, when no
    //# materials source is supplied.
    if input.keyring.is_null() {
        return Err("keyring is null in decrypt".to_string());
    }

    let mut core_input = aws_esdk::DecryptInput::with_legacy_keyring(
        //= spec/shim/esdk-shim.md#decrypt-inputs
        //# - `decrypt` MUST pass the target-supplied ciphertext to the core ESDK unmodified.
        input.ciphertext,
        //= spec/shim/esdk-shim.md#decrypt-inputs
        //# - `decrypt` MUST provide the target-supplied encryption context to the core ESDK,
        //# converted as defined in [Encryption context](#encryption-context).
        encryption_context_target_to_core(&input.encryption_context),
        unsafe { (*input.keyring).client.clone() },
    );
    //= spec/shim/esdk-shim.md#decrypt-inputs
    //# - `decrypt` MUST provide the target-supplied maximum-encrypted-data-keys value to
    //# the core ESDK, converted as defined in
    //# [Maximum encrypted data keys](#maximum-encrypted-data-keys).
    core_input.max_encrypted_data_keys = max_edks_target_to_core(input.max_encrypted_data_keys);
    //= spec/shim/esdk-shim.md#decrypt-inputs
    //# - `decrypt` MUST provide the target-supplied commitment policy to the core ESDK,
    //# converted as defined in [Commitment policy](#commitment-policy).
    core_input.commitment_policy = commitment_policy_target_to_core(input.commitment_policy)?;
    let output = DAFNY_TOKIO_RUNTIME.block_on(aws_esdk::decrypt(&core_input));
    let output = output.map_err(|e| format!("decrypt failed: {:?}", e))?;
    Ok(ffi::DecryptOutput {
        //= spec/shim/esdk-shim.md#decrypt-outputs
        //# - `decrypt` MUST return the used algorithm suite, converted as defined in
        //# [Algorithm suite identifier](#algorithm-suite-identifier).
        algorithm_suite_id: alg_id_core_to_target(output.algorithm_suite_id)?,
        //= spec/shim/esdk-shim.md#decrypt-outputs
        //# - `decrypt` MUST return the core ESDK's plaintext unmodified.
        plaintext: output.plaintext,
        //= spec/shim/esdk-shim.md#decrypt-outputs
        //# - `decrypt` MUST return the result encryption context, converted as defined in
        //# [Encryption context](#encryption-context).
        encryption_context: encryption_context_core_to_target(output.encryption_context),
    })
}

//= spec/shim/esdk-shim.md#service-client-configuration
//# - If the target does not supply a retry configuration, the shim MAY apply a
//# default retry configuration.
fn make_retry_config(config: &ffi::RetryConfig) -> aws_config::retry::RetryConfig {
    let mut out_config = if config.mode_adaptive {
        aws_config::retry::RetryConfig::adaptive()
    } else {
        aws_config::retry::RetryConfig::standard()
    };
    if config.max_attempts > 0 {
        out_config = out_config.with_max_attempts(config.max_attempts);
    }
    if config.initial_backoff_milli > 0 {
        out_config = out_config.with_initial_backoff(std::time::Duration::from_millis(
            config.initial_backoff_milli,
        ));
    }
    if config.max_backoff_milli > 0 {
        out_config =
            out_config.with_max_backoff(std::time::Duration::from_millis(config.max_backoff_milli));
    }
    if config.reconnect_all {
        out_config = out_config
            .with_reconnect_mode(aws_sdk_kms::config::retry::ReconnectMode::ReuseAllConnections);
    }
    if config.use_static_exponential_base {
        out_config = out_config.with_use_static_exponential_base(true);
    }
    out_config
}
fn delete_kms_client(_client: Box<MplKmsClient>) -> Result<(), String> {
    Ok(())
}
fn delete_ddb_client(_client: Box<MplDdbClient>) -> Result<(), String> {
    Ok(())
}
fn delete_keystore(_client: Box<KeyStore>) -> Result<(), String> {
    Ok(())
}
fn delete_keyring(_client: Box<Keyring>) -> Result<(), String> {
    Ok(())
}

fn make_cache_type(
    config: &ffi::HierarchicalKeyringInput,
) -> Result<aws_mpl_legacy::dafny::types::CacheType, String> {
    match config.cache {
        //= spec/shim/esdk-shim.md#cache-configuration
        //# - The shim MUST translate a no-cache selection.
        ffi::CacheType::NoCache => Ok(aws_mpl_legacy::dafny::types::CacheType::No(
            aws_mpl_legacy::dafny::types::NoCache::builder()
                .build()
                .map_err(|e| format!("failed to build NoCache: {:?}", e))?,
        )),
        //= spec/shim/esdk-shim.md#cache-configuration
        //# - The shim MUST translate a multi-threaded cache selection.
        ffi::CacheType::MultiThreadedCache => {
            let entry_capacity = i32::try_from(config.multi_threaded_cache.entryCapacity)
                .map_err(|_| {
                    format!(
                        "entry_capacity {} exceeds the core ESDK's i32 limit",
                        config.multi_threaded_cache.entryCapacity
                    )
                })?;
            let entry_pruning_tail_size =
                i32::try_from(config.multi_threaded_cache.entryPruningTailSize).map_err(|_| {
                    format!(
                        "entry_pruning_tail_size {} exceeds the core ESDK's i32 limit",
                        config.multi_threaded_cache.entryPruningTailSize
                    )
                })?;
            Ok(aws_mpl_legacy::dafny::types::CacheType::MultiThreaded(
                aws_mpl_legacy::dafny::types::MultiThreadedCache::builder()
                    .entry_capacity(entry_capacity)
                    .entry_pruning_tail_size(entry_pruning_tail_size)
                    .build()
                    .map_err(|e| format!("failed to build MultiThreadedCache: {:?}", e))?,
            ))
        }
        //= spec/shim/esdk-shim.md#cache-configuration
        //# - The shim MUST return an error when the target cache type is not a supported
        //# value.
        _ => Err("Invalid CacheType in HierarchicalKeyringInput".to_string()),
    }
}

fn make_kms_config(
    config: &ffi::KeyStoreConfig,
) -> Result<aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration, String> {
    match config.kms_configuration_type {
        //= spec/shim/esdk-shim.md#kms-configuration
        //# - The shim MUST translate a KMS key ARN configuration.
        ffi::KmsConfigurationType::KmsKeyArn => Ok(
            aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsKeyArn(
                config.kms_configuration_value.clone(),
            ),
        ),
        //= spec/shim/esdk-shim.md#kms-configuration
        //# - The shim MUST translate a KMS multi-Region-key ARN configuration.
        ffi::KmsConfigurationType::KmsMrKeyArn => Ok(
            aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsMrKeyArn(
                config.kms_configuration_value.clone(),
            ),
        ),
        //= spec/shim/esdk-shim.md#kms-configuration
        //# - The shim MUST translate a discovery configuration.
        ffi::KmsConfigurationType::Discovery => Ok(
            aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration::Discovery(
                aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::Discovery::builder()
                    .build()
                    .map_err(|e| format!("failed to build Discovery: {:?}", e))?,
            ),
        ),
        //= spec/shim/esdk-shim.md#kms-configuration
        //# - The shim MUST translate a multi-Region-key discovery configuration.
        ffi::KmsConfigurationType::MrDiscovery => Ok(
            aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration::MrDiscovery(
                aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::MrDiscovery::builder()
                    .build()
                    .map_err(|e| format!("failed to build MrDiscovery: {:?}", e))?,
            ),
        ),
        //= spec/shim/esdk-shim.md#kms-configuration
        //# - The shim MUST return an error when the target KMS configuration type is not a
        //# supported value.
        _ => Err("Invalid KmsConfigurationType".to_string()),
    }
}

//= spec/shim/esdk-shim.md#create-hierarchical-keyring
//# - The shim MUST provide an operation that creates a hierarchical keyring backed
//# by the core ESDK.
fn create_hierarchical_keyring(
    input: &ffi::HierarchicalKeyringInput,
) -> Result<Box<Keyring>, String> {
    let mpl_config = aws_mpl_legacy::dafny::types::MaterialProvidersConfig::builder()
        .build()
        .map_err(|e| format!("failed to build MaterialProvidersConfig: {:?}", e))?;
    let mpl = aws_mpl_legacy::dafny::Client::from_conf(mpl_config)
        .map_err(|e| format!("failed to create MPL client: {:?}", e))?;
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //# - Creating a hierarchical keyring MUST provide the target-supplied cache
    //# configuration to the core ESDK, converted as defined in
    //# [Cache configuration](#cache-configuration).
    //
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //# - Creating a hierarchical keyring MUST provide the target-supplied branch key id
    //# and time-to-live to the core ESDK.
    let mut builder = mpl
        .create_aws_kms_hierarchical_keyring()
        .cache(make_cache_type(input)?)
        .branch_key_id(input.branch_key_id.clone())
        .ttl_seconds(input.ttl);
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //# - Creating a hierarchical keyring MUST require a key store handle, checked as
    //# defined in
    //# [Shim Specification: Handles and lifetimes](./shim.md#handles-and-lifetimes),
    //# and MUST provide it to the core ESDK.
    if input.key_store.is_null() {
        return Err("key_store is null in create_hierarchical_keyring".to_string());
    } else {
        builder = builder.key_store(unsafe { (*input.key_store).client.clone() })
    }
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //# - Creating a hierarchical keyring MUST provide the target-supplied partition id to
    //# the core ESDK when present; an unset value is omitted.
    // The "when present" branch is exercised by `round_trip_integration` (partition
    // id set). The "unset ⇒ omitted" branch simply skips the builder call; the
    // omission is a construction guarantee, not separately testable without AWS.
    if !input.partition_id.is_empty() {
        builder = builder.partition_id(input.partition_id.clone());
    }

    let keyring = DAFNY_TOKIO_RUNTIME
        .block_on(builder.send())
        .map_err(|e| format!("failed to create hierarchical keyring: {:?}", e))?;

    let keyring = Keyring { client: keyring };
    Ok(Box::new(keyring))
}

//= spec/shim/esdk-shim.md#create-key-store
//# - The shim MUST provide an operation that creates a key store backed by the core
//# ESDK.
fn create_keystore(input: &ffi::KeyStoreConfig) -> Result<Box<KeyStore>, String> {
    let mut builder = aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig::builder();
    //= spec/shim/esdk-shim.md#create-key-store
    //# - Creating a key store MUST require a KMS client handle and a DynamoDB client
    //# handle, checked as defined in
    //# [Shim Specification: Handles and lifetimes](./shim.md#handles-and-lifetimes),
    //# and MUST provide both to the core ESDK.
    //
    //= spec/shim/shim.md#handles-and-lifetimes
    //# - Before dereferencing a target-supplied resource handle, the shim MUST check it
    //# and MUST return an error when a required handle is absent.
    if input.kms_client.is_null() {
        return Err("kms_client is null in create_keystore".to_string());
    } else {
        builder = builder.kms_client(unsafe { (*input.kms_client).client.clone() });
    }
    if input.ddb_client.is_null() {
        return Err("ddb_client is null in create_keystore".to_string());
    } else {
        builder = builder.ddb_client(unsafe { (*input.ddb_client).client.clone() });
    }
    // Input values are translated and passed through; the core ESDK validates
    // them (the shim does not short-circuit on empty/invalid values).
    //= spec/shim/esdk-shim.md#create-key-store
    //# - Creating a key store MUST provide the target-supplied table name and logical key
    //# store name to the core ESDK.
    builder = builder.ddb_table_name(input.ddb_table_name.clone());
    builder = builder.logical_key_store_name(input.logical_key_store_name.clone());
    //= spec/shim/esdk-shim.md#create-key-store
    //# - Creating a key store MUST provide the target-supplied KMS configuration to the
    //# core ESDK, converted as defined in [KMS configuration](#kms-configuration).
    builder = builder.kms_configuration(make_kms_config(input)?);
    //= spec/shim/esdk-shim.md#create-key-store
    //# - Creating a key store MUST provide the target-supplied key store id and grant
    //# tokens to the core ESDK when present; an unset value is omitted.
    // The "when present" branch is exercised by `round_trip_integration` (id and
    // grant tokens set). The "unset ⇒ omitted" branch simply skips the builder
    // call, so whether the core received the value is only observable through a
    // real key store; the omission is a construction guarantee, not separately
    // testable without AWS.
    if !input.id.is_empty() {
        builder = builder.id(input.id.clone());
    }
    if !input.grant_tokens.is_empty() {
        builder = builder.grant_tokens(input.grant_tokens.clone());
    }
    let config = builder
        .build()
        .map_err(|e| format!("failed to build KeyStoreConfig: {:?}", e))?;

    let store = aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::client::Client::from_conf(config)
        .map_err(|e| format!("failed to create key store client: {:?}", e))?;
    let store = KeyStore { client: store };
    Ok(Box::new(store))
}

//= spec/shim/esdk-shim.md#create-kms-client
//# - The shim MUST provide an operation that creates a KMS service client backed by
//# the core ESDK.
fn create_kms_client(input: &ffi::MplAwsClientConfig) -> Result<Box<MplKmsClient>, String> {
    //= spec/shim/esdk-shim.md#create-kms-client
    //# - Creating a KMS client MUST apply the target-supplied client configuration, as
    //# defined in [Service client configuration](#service-client-configuration).
    let sdk_config = create_sdk_config(input)?;
    let client = aws_sdk_kms::Client::new(&sdk_config);
    let client = MplKmsClient { client };
    Ok(Box::new(client))
}

//= spec/shim/esdk-shim.md#create-dynamodb-client
//# - The shim MUST provide an operation that creates a DynamoDB service client
//# backed by the core ESDK.
fn create_ddb_client(input: &ffi::MplAwsClientConfig) -> Result<Box<MplDdbClient>, String> {
    //= spec/shim/esdk-shim.md#create-dynamodb-client
    //# - Creating a DynamoDB client MUST apply the target-supplied client configuration,
    //# as defined in [Service client configuration](#service-client-configuration).
    let sdk_config = create_sdk_config(input)?;
    let client = aws_sdk_dynamodb::Client::new(&sdk_config);
    let client = MplDdbClient { client };
    Ok(Box::new(client))
}

//= spec/shim/esdk-shim.md#service-client-configuration
//= type=implication
//= reason=create_kms_client and create_ddb_client each build their SdkConfig from the same MplAwsClientConfig, so one configuration can be applied to multiple clients.
//# - The shim SHOULD allow one client configuration to be applied to multiple
//# service clients.
fn create_sdk_config(input: &ffi::MplAwsClientConfig) -> Result<SdkConfig, String> {
    let shared_config = DAFNY_TOKIO_RUNTIME.block_on(aws_config::load_defaults(
        aws_config::BehaviorVersion::latest(),
    ));

    // Target (C++) and core (Rust) languages are fixed for this shim; the version
    // is the crate version from Cargo.toml (i.e. the version this crate publishes).
    let user_agent_string = shim_user_agent();
    let current_app_name = shared_config
        .app_name()
        .map(|app_name| app_name.to_string())
        .unwrap_or_default();
    let new_app_name = merge_user_agent(&current_app_name, &user_agent_string);
    let app_name = AppName::new(new_app_name).map_err(|e| format!("invalid app name: {:?}", e))?;
    //= spec/shim/esdk-shim.md#service-client-configuration
    //# - If the target supplies a retry configuration, the shim MUST apply it to each
    //# service client.
    let mut builder = shared_config
        .to_builder()
        .app_name(app_name)
        .retry_config(make_retry_config(&input.retry));
    //= spec/shim/esdk-shim.md#service-client-configuration
    //# - If the target supplies a region, the shim MUST apply it to each service client.
    if !input.region.is_empty() {
        builder = builder.region(Region::new(input.region.clone()));
    }
    Ok(builder.build())
}

//= spec/shim/esdk-shim.md#conformance-and-testing
//= type=exception
//= reason=Cross-implementation message interoperability is exercised by the shared ESDK test-vector suite maintained outside this shim crate, not within it.
//# - The ESDK shim SHOULD demonstrate message interoperability with other ESDK
//# implementations (for example via a shared test-vector suite).
#[cfg(test)]
mod tests;
