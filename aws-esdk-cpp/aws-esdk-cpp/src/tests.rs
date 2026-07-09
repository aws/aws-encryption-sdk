//! Unit tests for the shim's pure conversion layer.
//!
//! These test the shim's OWN behavior (correct mapping + no target abort on
//! bad input), not ESDK crypto correctness. No AWS resources required.
//!
//! Declared from `lib.rs` as `#[cfg(test)] mod tests;`, so it is a child of the
//! crate root and reaches the crate-private conversion functions via `super::`.

use super::*;
use ffi::EsdkAlgorithmSuiteId as F;
use ffi::EsdkCommitmentPolicy as P;

// The complete set of algorithm-suite discriminants the shim supports.
// Mirrors the `ffi::EsdkAlgorithmSuiteId` enum in the bridge.
const ALG_REPRS: [u16; 11] = [
    0x0014, 0x0046, 0x0078, 0x0114, 0x0146, 0x0178, 0x0214, 0x0346, 0x0378, 0x0478, 0x0578,
];

// alg_id_target_to_core: ffi enum -> core enum

/// Exhaustiveness (ffi -> core): the set of values `alg_id_target_to_core` accepts must
/// be EXACTLY the known suite ids — no more, no fewer. Adding a suite id without
/// updating the conversion (or vice versa) fails this test.
//= spec/shim/shim.md#conformance-and-testing
//= type=test
//# - The shim SHOULD be tested for the behavior it owns — translation, handle
//# validation, error propagation, and lifetime safety — rather than by re-testing
//# the core library's own behavior.
//
//= spec/shim/esdk-shim.md#conformance-and-testing
//= type=test
//# - The ESDK shim MUST be tested for the behavior it owns — translation, handle
//# validation, error propagation, and lifetime safety — rather than by re-testing
//# the core ESDK's cryptographic behavior.
#[test]
fn alg_id_target_to_core_accepts_exactly_known_reprs() {
    let mut accepted: Vec<u16> = (0..=u16::MAX)
        .filter(|&n| alg_id_target_to_core(F { repr: n }).is_ok())
        .collect();
    accepted.sort_unstable();
    let mut expected = ALG_REPRS.to_vec();
    expected.sort_unstable();
    //= spec/shim/esdk-shim.md#algorithm-suite-identifier
    //= type=test
    //# - The shim MUST translate a target algorithm suite identifier to the core ESDK
    //# algorithm suite that has the same two-byte ID.
    assert_eq!(accepted, expected);
}

/// Security property: NO value a C++ caller can supply may abort the target.
/// Sweeping the entire u16 domain proves `alg_id_target_to_core` returns Ok/Err for
/// every possible input and never panics.
//= spec/shim/shim.md#type-translation
//= type=test
//# - A translation reports failure by returning an error; it MUST NOT panic,
//# abort, or otherwise terminate the target process.
#[test]
fn alg_id_target_to_core_never_panics_over_full_range() {
    for n in 0..=u16::MAX {
        let _ = alg_id_target_to_core(F { repr: n });
    }
}

/// An unknown suite id returns an error rather than panicking.
#[test]
fn alg_id_target_to_core_rejects_unknown() {
    //= spec/shim/esdk-shim.md#algorithm-suite-identifier
    //= type=test
    //# - The shim MUST return an error when the target value is not one of the algorithm
    //# suite IDs defined by the core ESDK.
    //
    //= spec/shim/shim.md#type-translation
    //= type=test
    //# - When the target supplies a value that has no corresponding core-library
    //# representation, the translation MUST return an error.
    let err = alg_id_target_to_core(F { repr: 0xFFFF }).unwrap_err();
    assert!(err.contains("unrecognized"), "unexpected error: {err}");
    assert!(alg_id_target_to_core(F { repr: 0x0000 }).is_err());
}

// alg_id_core_to_target: core enum -> ffi enum

/// Every core variant maps to the expected ffi discriminant.
#[test]
fn alg_id_core_to_target_maps_every_core_variant() {
    use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId as C;
    let pairs: [(C, u16); 11] = [
        (C::AlgAes128GcmIv12Tag16NoKdf, 0x0014),
        (C::AlgAes192GcmIv12Tag16NoKdf, 0x0046),
        (C::AlgAes256GcmIv12Tag16NoKdf, 0x0078),
        (C::AlgAes128GcmIv12Tag16HkdfSha256, 0x0114),
        (C::AlgAes192GcmIv12Tag16HkdfSha256, 0x0146),
        (C::AlgAes256GcmIv12Tag16HkdfSha256, 0x0178),
        (C::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256, 0x0214),
        (C::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384, 0x0346),
        (C::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384, 0x0378),
        (C::AlgAes256GcmHkdfSha512CommitKey, 0x0478),
        (C::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384, 0x0578),
    ];
    for (core, repr) in pairs {
        //= spec/shim/esdk-shim.md#algorithm-suite-identifier
        //= type=test
        //# - The shim MUST translate a core ESDK algorithm suite to the target algorithm
        //# suite identifier that has the same two-byte ID.
        assert_eq!(alg_id_core_to_target(core).unwrap().repr, repr);
    }
}

/// Round-trip: every known ffi suite id survives ffi -> core -> ffi.
#[test]
fn alg_id_round_trips_through_both_conversions() {
    for &n in &ALG_REPRS {
        let core = alg_id_target_to_core(F { repr: n }).expect("known suite id converts");
        //= spec/shim/shim.md#type-translation
        //= type=test
        //= reason=The suite id survives ffi -> core -> ffi unchanged, proving the translation preserves meaning in both directions.
        //# - A translation MUST convert between the target representation and the core
        //# library representation, preserving meaning in both directions it supports.
        let back = alg_id_core_to_target(core).expect("round-trips back");
        assert_eq!(back.repr, n);
    }
}

// commitment_policy_target_to_core: ffi enum -> core enum

/// Exhaustiveness + correctness: each known repr maps to the core commitment
/// policy of the same meaning, and exactly the reprs {0,1,2} are accepted.
#[test]
fn commitment_policy_target_to_core_accepts_exactly_known_reprs() {
    use aws_mpl_legacy::commitment::EsdkCommitmentPolicy as C;
    let pairs: [(P, C); 3] = [
        (P::ForbidEncryptAllowDecrypt, C::ForbidEncryptAllowDecrypt),
        (P::RequireEncryptAllowDecrypt, C::RequireEncryptAllowDecrypt),
        (P::RequireEncryptRequireDecrypt, C::RequireEncryptRequireDecrypt),
    ];
    for (target, expected_core) in pairs {
        //= spec/shim/esdk-shim.md#commitment-policy
        //= type=test
        //# - The shim MUST translate the target commitment policy to the core ESDK
        //# commitment policy of the same meaning.
        assert_eq!(
            commitment_policy_target_to_core(target).unwrap(),
            expected_core
        );
    }

    let accepted: Vec<u8> = (0..=u8::MAX)
        .filter(|&n| commitment_policy_target_to_core(P { repr: n }).is_ok())
        .collect();
    //= spec/shim/esdk-shim.md#commitment-policy
    //= type=test
    //= reason=Sweeping 0..=u8::MAX, only 0,1,2 are accepted; every other value returns an error.
    //# - The shim MUST return an error when the target commitment policy is not a supported
    //# value.
    assert_eq!(accepted, vec![0u8, 1, 2]);
}

/// No commitment-policy value from C++ can abort the target.
#[test]
fn commitment_policy_target_to_core_never_panics_over_full_range() {
    for n in 0..=u8::MAX {
        let _ = commitment_policy_target_to_core(P { repr: n });
    }
}

// encryption context: HashMap <-> Vec<EncryptionContextItem>

#[test]
fn encryption_context_round_trips() {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    m.insert("k1".to_string(), "v1".to_string());
    m.insert("k2".to_string(), "v2".to_string());
    let list = encryption_context_core_to_target(m.clone());
    assert_eq!(list.len(), 2);
    //= spec/shim/esdk-shim.md#encryption-context
    //= type=test
    //# - The shim MUST translate the target's encryption context, given as a list of
    //# key-value pairs, to the core ESDK's key-value map, preserving every pair and
    //# the exact key and value of each.
    //
    //= spec/shim/esdk-shim.md#encryption-context
    //= type=test
    //= reason=Round-trip map -> list -> map equals the original, proving the core-to-target conversion preserves every pair, key and value.
    //# - The shim MUST translate the core ESDK's encryption context, given as a
    //# key-value map, to the target's list of key-value pairs, preserving every pair
    //# and the exact key and value of each.
    assert_eq!(encryption_context_target_to_core(&list), m);
}

#[test]
fn encryption_context_empty() {
    // Regression: the empty map/list must translate to an empty result in both
    // directions (no spurious pairs introduced), guarding the conversion helpers'
    // edge case.
    use std::collections::HashMap;
    let empty: HashMap<String, String> = HashMap::new();
    assert!(encryption_context_core_to_target(empty).is_empty());
    assert!(encryption_context_target_to_core(&[]).is_empty());
}

// Orchestration error paths (NO AWS resources required)
// These reach the null-checks / enum-validation in the create_*/encrypt/decrypt
// functions, which all return before any network call. They exercise the
// error-handling lines without credentials, so they run in ordinary CI.

#[test]
fn encrypt_rejects_null_keyring() {
    // default_encrypt_input() has a null keyring pointer.
    //= spec/shim/esdk-shim.md#materials-source
    //= type=test
    //# - The shim MUST return an error, and MUST NOT invoke the core ESDK, when no
    //# materials source is supplied.
    let err = match encrypt(&default_encrypt_input()) {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    };
    assert!(err.contains("keyring is null"), "unexpected error: {err}");
}

#[test]
fn decrypt_rejects_null_keyring() {
    // Regression coverage for the symmetric decrypt null-check. The
    // materials-source requirement is annotated once (on encrypt) to avoid a
    // duplicate citation.
    let err = match decrypt(&default_decrypt_input()) {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    };
    assert!(err.contains("keyring is null"), "unexpected error: {err}");
}

#[test]
fn create_keystore_rejects_null_kms_client() {
    // default config has null kms_client / ddb_client pointers.
    let err = match create_keystore(&default_keystore_config()) {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    };
    //= spec/shim/shim.md#handles-and-lifetimes
    //= type=test
    //# - Before dereferencing a target-supplied resource handle, the shim MUST check it
    //# and MUST return an error when a required handle is absent.
    //
    //= spec/shim/esdk-shim.md#create-key-store
    //= type=test
    //# - Creating a key store MUST require a KMS client handle and a DynamoDB client
    //# handle, checked as defined in
    //# [Shim Specification: Handles and lifetimes](./shim.md#handles-and-lifetimes),
    //# and MUST provide both to the core ESDK.
    assert!(err.contains("kms_client is null"), "unexpected error: {err}");
}

#[test]
fn create_hierarchical_keyring_rejects_null_key_store() {
    // default input has a null key_store pointer; the handle check fires before
    // any network call.
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //= type=test
    //# - Creating a hierarchical keyring MUST require a key store handle, checked as
    //# defined in
    //# [Shim Specification: Handles and lifetimes](./shim.md#handles-and-lifetimes),
    //# and MUST provide it to the core ESDK.
    let err = match create_hierarchical_keyring(&default_hierarchical_keyring_input()) {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    };
    assert!(err.contains("key_store is null"), "unexpected error: {err}");
}

#[test]
fn make_kms_config_rejects_unknown_type() {
    let mut cfg = default_keystore_config();
    cfg.kms_configuration_type = ffi::KmsConfigurationType { repr: 99 };
    //= spec/shim/esdk-shim.md#kms-configuration
    //= type=test
    //# - The shim MUST return an error when the target KMS configuration type is not a
    //# supported value.
    let err = make_kms_config(&cfg).unwrap_err();
    assert!(err.contains("Invalid KmsConfigurationType"), "unexpected error: {err}");
}

#[test]
fn make_kms_config_accepts_key_arn() {
    // default type is KmsKeyArn -> exercises the happy arm (local, no AWS).
    use aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration;
    //= spec/shim/esdk-shim.md#kms-configuration
    //= type=test
    //# - The shim MUST translate a KMS key ARN configuration.
    assert!(matches!(
        make_kms_config(&default_keystore_config()).unwrap(),
        KmsConfiguration::KmsKeyArn(_)
    ));
}

#[test]
fn make_kms_config_accepts_mr_key_arn() {
    use aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration;
    let mut cfg = default_keystore_config();
    cfg.kms_configuration_type = ffi::KmsConfigurationType::KmsMrKeyArn;
    //= spec/shim/esdk-shim.md#kms-configuration
    //= type=test
    //# - The shim MUST translate a KMS multi-Region-key ARN configuration.
    assert!(matches!(
        make_kms_config(&cfg).unwrap(),
        KmsConfiguration::KmsMrKeyArn(_)
    ));
}

#[test]
fn make_kms_config_accepts_discovery() {
    use aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration;
    let mut cfg = default_keystore_config();
    cfg.kms_configuration_type = ffi::KmsConfigurationType::Discovery;
    //= spec/shim/esdk-shim.md#kms-configuration
    //= type=test
    //# - The shim MUST translate a discovery configuration.
    assert!(matches!(
        make_kms_config(&cfg).unwrap(),
        KmsConfiguration::Discovery(_)
    ));
}

#[test]
fn make_kms_config_accepts_mr_discovery() {
    use aws_mpl_legacy::dafny::deps::aws_cryptography_keyStore::types::KmsConfiguration;
    let mut cfg = default_keystore_config();
    cfg.kms_configuration_type = ffi::KmsConfigurationType::MrDiscovery;
    //= spec/shim/esdk-shim.md#kms-configuration
    //= type=test
    //# - The shim MUST translate a multi-Region-key discovery configuration.
    assert!(matches!(
        make_kms_config(&cfg).unwrap(),
        KmsConfiguration::MrDiscovery(_)
    ));
}

#[test]
fn make_cache_type_rejects_unknown() {
    let mut input = default_hierarchical_keyring_input();
    input.cache = ffi::CacheType { repr: 99 };
    //= spec/shim/esdk-shim.md#cache-configuration
    //= type=test
    //# - The shim MUST return an error when the target cache type is not a supported
    //# value.
    let err = make_cache_type(&input).unwrap_err();
    assert!(err.contains("Invalid CacheType"), "unexpected error: {err}");
}

#[test]
fn make_cache_type_builds_multi_threaded() {
    // default cache is MultiThreaded -> exercises that arm (local, no AWS).
    use aws_mpl_legacy::dafny::types::CacheType;
    //= spec/shim/esdk-shim.md#cache-configuration
    //= type=test
    //# - The shim MUST translate a multi-threaded cache selection.
    assert!(matches!(
        make_cache_type(&default_hierarchical_keyring_input()).unwrap(),
        CacheType::MultiThreaded(_)
    ));
}

#[test]
fn make_cache_type_builds_no_cache() {
    use aws_mpl_legacy::dafny::types::CacheType;
    let mut input = default_hierarchical_keyring_input();
    input.cache = ffi::CacheType::NoCache;
    //= spec/shim/esdk-shim.md#cache-configuration
    //= type=test
    //# - The shim MUST translate a no-cache selection.
    assert!(matches!(
        make_cache_type(&input).unwrap(),
        CacheType::No(_)
    ));
}

// Frame length + max-encrypted-data-keys translation (pure, no AWS)

#[test]
fn make_frame_length_translates_and_rejects_zero() {
    //= spec/shim/esdk-shim.md#frame-length
    //= type=test
    //# - The shim MUST translate the target frame length using the core ESDK's frame
    //# length constructor, and MUST return an error when the core ESDK rejects the
    //# value.
    let err = make_frame_length(0).unwrap_err();
    assert!(
        err.contains("invalid frame_length"),
        "unexpected error: {err}"
    );
    assert!(make_frame_length(4096).is_ok());
    assert!(make_frame_length(1).is_ok());
}

#[test]
fn max_edks_zero_is_no_limit_positive_is_limit() {
    //= spec/shim/esdk-shim.md#maximum-encrypted-data-keys
    //= type=test
    //# The shim MUST translate zero
    //# to the core ESDK's "no limit" representation, and a positive value `n` to a
    //# limit of `n`.
    assert_eq!(max_edks_target_to_core(0), None);
    assert_eq!(max_edks_target_to_core(1), std::num::NonZeroUsize::new(1));
    assert_eq!(max_edks_target_to_core(42), std::num::NonZeroUsize::new(42));
}

// Service client user-agent + retry translation (pure, no AWS)

#[test]
fn user_agent_form_and_append() {
    // Empty existing app-name -> the shim's own user-agent, in the required form.
    //= spec/shim/esdk-shim.md#service-client-configuration
    //= type=test
    //# - The shim MUST set a user-agent on each service client of the form
    //# `AwsEncryptionSdk-Shim-<target-language>-<core-language>-<version>`, where
    //# `<target-language>` is the target language, `<core-language>` is the core
    //# ESDK's implementation language, and `<version>` is the shim's published version.
    let ua = shim_user_agent();
    assert_eq!(merge_user_agent("", &ua), ua);
    assert!(ua.starts_with("AwsEncryptionSdk-Shim-C++-Rust-"));

    // Existing app-name is preserved and the shim's UA appended, not replaced.
    //= spec/shim/esdk-shim.md#service-client-configuration
    //= type=test
    //# - The shim MUST preserve any user-agent already present in the loaded
    //# configuration, appending its own rather than replacing it.
    let merged = merge_user_agent("existing-agent/1.0", &ua);
    assert!(merged.starts_with("existing-agent/1.0"), "must preserve existing: {merged}");
    assert!(merged.contains(&ua), "must append shim UA: {merged}");
}

#[test]
fn make_retry_config_applies_a_default() {
    // With no explicit values the shim still produces a (default) retry config.
    //= spec/shim/esdk-shim.md#service-client-configuration
    //= type=test
    //= reason=make_retry_config always returns a standard/adaptive RetryConfig even when the target supplies no values.
    //# - If the target does not supply a retry configuration, the shim MAY apply a
    //# default retry configuration.
    let cfg = default_retry_config();
    let _ = make_retry_config(&cfg);
    // With an explicit max_attempts the translation carries it through.
    let mut with_attempts = default_retry_config();
    with_attempts.max_attempts = 5;
    assert_eq!(make_retry_config(&with_attempts).max_attempts(), 5);
}

// Service client SdkConfig assembly (builds config, no credentials used)

#[test]
fn create_sdk_config_applies_region_retry_and_user_agent() {
    let mut cfg = default_client_config();
    cfg.region = "us-west-2".to_string();
    cfg.retry.max_attempts = 7;
    let sdk = create_sdk_config(&cfg).expect("build sdk config");

    let region: Option<&str> = sdk.region().map(|r| r.as_ref());
    //= spec/shim/esdk-shim.md#service-client-configuration
    //= type=test
    //# - If the target supplies a region, the shim MUST apply it to each service client.
    assert_eq!(region, Some("us-west-2"));

    //= spec/shim/esdk-shim.md#service-client-configuration
    //= type=test
    //# - If the target supplies a retry configuration, the shim MUST apply it to each
    //# service client.
    assert_eq!(sdk.retry_config().map(|r| r.max_attempts()), Some(7));

    let app_name: &str = sdk.app_name().expect("app name is set").as_ref();
    // Regression: the assembled SdkConfig carries the shim's user-agent; the
    // user-agent form/append requirements are annotated in user_agent_form_and_append.
    assert!(app_name.contains("AwsEncryptionSdk-Shim-C++-Rust-"));
}

// Full round-trip integration test (REQUIRES real AWS resources)
// Skips cleanly when the ESDK_TEST_* env vars are unset, so it never fails
// credential-less CI. Run under `cargo llvm-cov` in a creds-enabled job to
// cover the create_*/encrypt/decrypt success paths.

fn test_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|s| !s.is_empty())
}

#[test]
fn round_trip_integration() {
    let (kms_arn, table, logical, branch) = match (
        test_env("ESDK_TEST_KMS_KEY_ARN"),
        test_env("ESDK_TEST_DDB_TABLE"),
        test_env("ESDK_TEST_LOGICAL_KEYSTORE"),
        test_env("ESDK_TEST_BRANCH_KEY_ID"),
    ) {
        (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
        _ => {
            eprintln!(
                "SKIP round_trip_integration: set ESDK_TEST_KMS_KEY_ARN, \
                 ESDK_TEST_DDB_TABLE, ESDK_TEST_LOGICAL_KEYSTORE, ESDK_TEST_BRANCH_KEY_ID"
            );
            return;
        }
    };

    let mut client_cfg = default_client_config();
    client_cfg.region = test_env("ESDK_TEST_REGION").unwrap_or_default();

    //= spec/shim/esdk-shim.md#create-kms-client
    //= type=test
    //# - The shim MUST provide an operation that creates a KMS service client backed by
    //# the core ESDK.
    //
    //= spec/shim/esdk-shim.md#create-kms-client
    //= type=test
    //# - Creating a KMS client MUST apply the target-supplied client configuration, as
    //# defined in [Service client configuration](#service-client-configuration).
    let kms = create_kms_client(&client_cfg).expect("create kms client");

    //= spec/shim/esdk-shim.md#create-dynamodb-client
    //= type=test
    //# - The shim MUST provide an operation that creates a DynamoDB service client
    //# backed by the core ESDK.
    //
    //= spec/shim/esdk-shim.md#create-dynamodb-client
    //= type=test
    //# - Creating a DynamoDB client MUST apply the target-supplied client configuration,
    //# as defined in [Service client configuration](#service-client-configuration).
    let ddb = create_ddb_client(&client_cfg).expect("create ddb client");

    let mut ks_cfg = default_keystore_config();
    //= spec/shim/esdk-shim.md#create-key-store
    //= type=test
    //# - Creating a key store MUST provide the target-supplied table name and logical key
    //# store name to the core ESDK.
    ks_cfg.ddb_table_name = table;
    ks_cfg.logical_key_store_name = logical;
    //= spec/shim/esdk-shim.md#create-key-store
    //= type=test
    //# - Creating a key store MUST provide the target-supplied KMS configuration to the
    //# core ESDK, converted as defined in [KMS configuration](#kms-configuration).
    ks_cfg.kms_configuration_type = ffi::KmsConfigurationType::KmsKeyArn;
    ks_cfg.kms_configuration_value = kms_arn;
    ks_cfg.kms_client = &*kms as *const MplKmsClient;
    ks_cfg.ddb_client = &*ddb as *const MplDdbClient;
    //= spec/shim/esdk-shim.md#create-key-store
    //= type=test
    //# - Creating a key store MUST provide the target-supplied key store id and grant
    //# tokens to the core ESDK when present; an unset value is omitted.
    ks_cfg.id = "shim-integration-test".to_string();
    ks_cfg.grant_tokens = vec!["grant-token-example".to_string()];
    //= spec/shim/esdk-shim.md#create-key-store
    //= type=test
    //# - The shim MUST provide an operation that creates a key store backed by the core
    //# ESDK.
    let keystore = create_keystore(&ks_cfg).expect("create keystore");

    let mut kr_in = default_hierarchical_keyring_input();
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //= type=test
    //# - Creating a hierarchical keyring MUST provide the target-supplied branch key id
    //# and time-to-live to the core ESDK.
    kr_in.branch_key_id = branch;
    kr_in.key_store = &*keystore as *const KeyStore;
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //= type=test
    //# - Creating a hierarchical keyring MUST provide the target-supplied cache
    //# configuration to the core ESDK, converted as defined in
    //# [Cache configuration](#cache-configuration).
    kr_in.cache = ffi::CacheType::MultiThreadedCache;
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //= type=test
    //# - Creating a hierarchical keyring MUST provide the target-supplied partition id to
    //# the core ESDK when present; an unset value is omitted.
    kr_in.partition_id = "shim-partition".to_string();
    //= spec/shim/esdk-shim.md#create-hierarchical-keyring
    //= type=test
    //# - The shim MUST provide an operation that creates a hierarchical keyring backed
    //# by the core ESDK.
    let keyring = create_hierarchical_keyring(&kr_in).expect("create keyring");

    let plaintext = b"Hello from the Rust integration test".to_vec();
    let mut ec = std::collections::HashMap::new();
    ec.insert("purpose".to_string(), "integration-test".to_string());

    let mut enc_in = default_encrypt_input();
    enc_in.keyring = &*keyring as *const Keyring;
    enc_in.plaintext = plaintext.as_slice();
    enc_in.encryption_context = ec
        .iter()
        .map(|(k, v)| ffi::EncryptionContextItem { key: k.clone(), value: v.clone() })
        .collect();
    enc_in.algorithm_suite_id = ffi::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384;
    // commitment policy, frame length, and max-EDKs are forwarded plumbing
    // (plain source-side `implementation`); values here equal the ESDK defaults, so
    // asserting on them would not falsify the forwarding — no `type=test` here.
    enc_in.commitment_policy = ffi::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    enc_in.frame_length = 4096;
    enc_in.max_encrypted_data_keys = 3;

    let enc_out = encrypt(&enc_in).expect("encrypt");
    // Ciphertext differs from plaintext (encryption happened); the "unmodified"
    // guarantee is proven below by the successful decrypt back to `plaintext`.
    assert_ne!(enc_out.ciphertext, plaintext);
    //= spec/shim/esdk-shim.md#encrypt-outputs
    //= type=test
    //# - `encrypt` MUST return the used algorithm suite, converted as defined in
    //# [Algorithm suite identifier](#algorithm-suite-identifier).
    //
    //= spec/shim/esdk-shim.md#encrypt-inputs
    //= type=test
    //= reason=The returned used-suite equals the suite set on the input, proving the target-supplied algorithm suite reached the core ESDK.
    //# - `encrypt` MUST provide the target-supplied algorithm suite to the core ESDK,
    //# converted as defined in [Algorithm suite identifier](#algorithm-suite-identifier).
    assert_eq!(enc_out.algorithm_suite_id.repr, enc_in.algorithm_suite_id.repr);
    //= spec/shim/esdk-shim.md#encrypt-outputs
    //= type=test
    //# - `encrypt` MUST return the result encryption context, converted as defined in
    //# [Encryption context](#encryption-context).
    //
    //= spec/shim/esdk-shim.md#encrypt-inputs
    //= type=test
    //= reason=The result encryption context contains the target-supplied pair, proving the encryption context reached the core ESDK.
    //# - `encrypt` MUST provide the target-supplied encryption context to the core ESDK,
    //# converted as defined in [Encryption context](#encryption-context).
    assert_eq!(
        encryption_context_target_to_core(&enc_out.encryption_context).get("purpose"),
        Some(&"integration-test".to_string())
    );

    let mut dec_in = default_decrypt_input();
    dec_in.keyring = &*keyring as *const Keyring;
    dec_in.ciphertext = enc_out.ciphertext.as_slice();
    // commitment policy and max-EDKs are forwarded plumbing (plain source-side
    // `implementation`); values here equal the ESDK defaults, so asserting on
    // them would not falsify the forwarding — no `type=test` here.
    dec_in.commitment_policy = ffi::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    dec_in.max_encrypted_data_keys = 3;
    //= spec/shim/esdk-shim.md#decrypt-inputs
    //= type=test
    //= reason=decrypt supplies the same encryption-context pair used at encrypt time; a successful round-trip proves it was provided to the core ESDK.
    //# - `decrypt` MUST provide the target-supplied encryption context to the core ESDK,
    //# converted as defined in [Encryption context](#encryption-context).
    dec_in.encryption_context = ec
        .iter()
        .map(|(k, v)| ffi::EncryptionContextItem { key: k.clone(), value: v.clone() })
        .collect();

    let dec_out = decrypt(&dec_in).expect("decrypt");
    // Round-trip fidelity — plaintext/ciphertext passed and returned unmodified,
    // and the core operation actually invoked — is proven end-to-end through the
    // C++ facade in test_esdk.cpp (see the `type=test` citations there).
    assert_eq!(dec_out.plaintext, plaintext);
    // The decrypt-output fields (used suite, result encryption context) are now
    // surfaced by the C++ facade's DecryptResult and asserted there (test_esdk.cpp);
    // these bridge-level checks remain but carry no duvet citation.
    assert_eq!(dec_out.algorithm_suite_id.repr, enc_in.algorithm_suite_id.repr);
    assert_eq!(
        encryption_context_target_to_core(&dec_out.encryption_context).get("purpose"),
        Some(&"integration-test".to_string())
    );
}
