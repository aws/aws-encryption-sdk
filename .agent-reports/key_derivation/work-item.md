# Work Item: Add Test Annotations for Encrypt Key Derivation Requirements in key_derivation.rs

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `get-the-encryption-materials`
- **Duvet Target**: `specification/client-apis/encrypt.md#get-the-encryption-materials`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The algorithm used to derive a data key from the plaintext data key MUST be
  the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
  [algorithm suite](../framework/algorithm-suites.md) defined above.
  ```
- **Current State**: needs-test
- **Sub-items**: See Requirements 2 and 3 below (these are sub-items of this requirement in the spec)

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
  then the derived data key MUST be the same as the plaintext data key.
  ```
- **Current State**: needs-test

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - If the key derivation algorithm is [HKDF](../framework/algorithm-suites.md#hkdf),
  the derivation process used MUST be the process described in [HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key).
  ```
- **Current State**: needs-test

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/key_derivation.rs`
```rust
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The algorithm used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) defined above.
    match &suite.kdf {
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
        //# then the derived data key MUST be the same as the plaintext data key.
        DerivationAlgorithm::Identity => Ok(ExpandedKeyMaterial {
            data_key: plaintext_data_key.to_vec(),
            commitment_key: None,
        }),
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - If the key derivation algorithm is [HKDF](../framework/algorithm-suites.md#hkdf),
        //# the derivation process used MUST be the process described in [HKDF Encryption Key](../transitive-requirements.md#hkdf-encryption-key).
        DerivationAlgorithm::Hkdf(hkdf) => {
            // ... HKDF derivation logic
        }
    }
```

### Test File: `NEW FILE NEEDED: AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_key_derivation.rs`

## Implementation Guidance
- Follow the test pattern in `tests/test_v1_header_body.rs` — uses `raw_aes_keyring` (no KMS needed), sets `ForbidEncryptAllowDecrypt` commitment policy for v1 suites
- Follow the test pattern in `tests/test_get_decryption_materials.rs` — uses `round_trip` helper for encrypt-then-decrypt verification
- For the identity KDF test: use `AlgAes256GcmIv12Tag16NoKdf` algorithm suite with `ForbidEncryptAllowDecrypt` commitment policy. A successful round-trip proves the identity KDF produced the correct derived key (same as plaintext key).
- For the HKDF test: use `AlgAes256GcmIv12Tag16HkdfSha256` algorithm suite with `ForbidEncryptAllowDecrypt` commitment policy. A successful round-trip proves the HKDF derivation produced the correct derived key.
- For the parent requirement (algorithm used MUST be the KDF from the suite): a successful round-trip with any algorithm suite proves the correct KDF was used, since using the wrong KDF would produce the wrong key and decryption would fail.
- The default v2 algorithm suite (with commitment) also exercises HKDF via `expand_key_material()`, so a default round-trip test covers the v2 path.

### Spec-Aligned Structure
The spec describes this flow:
1. [parent requirement: algorithm MUST be the KDF from the suite] → annotate test at the top of a round-trip test that exercises key derivation
2. [identity KDF sub-item] → annotate test at a round-trip test using a no-KDF algorithm suite
3. [HKDF sub-item] → annotate test at a round-trip test using an HKDF algorithm suite

Sub-items to annotate individually:
- "The algorithm used to derive a data key..." → at a general round-trip test proving correct KDF selection
- "If the key derivation algorithm is the identity KDF..." → at a test using `AlgAes256GcmIv12Tag16NoKdf`
- "If the key derivation algorithm is HKDF..." → at a test using `AlgAes256GcmIv12Tag16HkdfSha256`

## Targeted Tests
- `test_key_derivation_uses_suite_kdf` — round-trip with default (v2 committing) suite proves correct KDF algorithm is used
- `test_identity_kdf_derived_key_equals_plaintext_key` — round-trip with `AlgAes256GcmIv12Tag16NoKdf` proves identity KDF returns plaintext key unchanged
- `test_hkdf_derivation_process` — round-trip with `AlgAes256GcmIv12Tag16HkdfSha256` proves HKDF derivation produces correct key

## Success Criteria
```bash
cargo test test_key_derivation
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for encrypt.md#get-the-encryption-materials key derivation requirements
- [ ] All 3 requirements have `type=implementation` annotations (already present)
- [ ] All 3 implementations have corresponding `type=test` annotations (new)
