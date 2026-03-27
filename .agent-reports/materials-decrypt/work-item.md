# Work Item: Fix 3 Remaining Gaps in decrypt.md#get-the-decryption-materials (materials.rs)

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `get-the-decryption-materials`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials`

## Type of Work
ADD_TESTS

## Context

A previous cycle (commit 695713fd) added 8 new test annotations. After re-running duvet, 3 requirements remain incomplete. All 3 already have implementation annotations in `src/materials.rs`, but those annotations use the `specification/` symlink prefix. Duvet extracts requirements from `aws-encryption-sdk-specification/` and does NOT match annotations using the `specification/` prefix to those requirements. The implementation annotations are therefore invisible to duvet.

The fix is to add `type=test` annotations in the test file using the `aws-encryption-sdk-specification/` prefix. The existing `type=implication` annotations in `materials.rs` also need their path prefix corrected, but since `implication` satisfies both implementation and test, adding test annotations alone will close the gaps.

## Requirements to Address

### Requirement 1 (ID 860)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
  encrypt MUST yield an error.
  ```
- **Current State**: incomplete (has `type=implication` in materials.rs at line 210 using wrong path prefix; no test annotation)

### Requirement 2 (ID 861)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
  configured in the [client](client.md) decrypt MUST yield an error.
  ```
- **Current State**: incomplete (has `type=implication` in materials.rs at line 215 using wrong path prefix; test exists in test_post_cmm_validation.rs but uses `specification/` prefix)

### Requirement 3 (ID 865)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the key derivation algorithm is the [identity KDF](../framework/algorithm-suites.md#identity-kdf),
  then the derived data key MUST be the same as the plaintext data key.
  ```
- **Current State**: incomplete (no annotation found for this requirement in the decrypt path; the encrypt path has annotations in key_derivation.rs but those target the encrypt spec section)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs`
```rust
// Line 210-214: existing implication annotation for req 860
//= specification/client-apis/decrypt.md#get-the-decryption-materials
//= type=implication
//= reason=The CMM resolves the algorithm suite from the header; unsupported ESDK suites fail during CMM processing
//# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
//# encrypt MUST yield an error.

// Line 215-217: existing annotation for req 861
//= specification/client-apis/decrypt.md#get-the-decryption-materials
//# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
//# configured in the [client](client.md) decrypt MUST yield an error.
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs`
```rust
// Existing test annotations use aws-encryption-sdk-specification/ prefix
//= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
//= type=test
//# ...
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_post_cmm_validation.rs`
```rust
// Line 75: existing test for req 861 but uses wrong prefix
//= specification/client-apis/decrypt.md#get-the-decryption-materials
//= type=test
//# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
//# configured in the [client](client.md) decrypt MUST yield an error.
```

## Implementation Guidance

### For Requirements 860 and 861 (post-CMM validation)
- The test `test_post_cmm_commitment_policy_decrypt` in `test_post_cmm_validation.rs` already tests requirement 861 but uses the `specification/` prefix. Change it to `aws-encryption-sdk-specification/`.
- The test `test_post_cmm_esdk_suite_decrypt` in `test_post_cmm_validation.rs` already tests requirement 860 but uses the `specification/` prefix. Change it to `aws-encryption-sdk-specification/`.
- Pattern: see existing test annotations in `test_get_decryption_materials.rs` which correctly use `aws-encryption-sdk-specification/`.

### For Requirement 865 (identity KDF on decrypt)
- The identity KDF requirement for decrypt is fulfilled in `src/key_derivation.rs` (the `derive_data_key` function handles identity KDF by returning the plaintext data key unchanged).
- A test annotation needs to be added to an existing test that exercises the identity KDF path during decryption (e.g., a test using `ALG_AES_256_GCM_IV12_TAG16_NO_KDF`).
- The test in `test_key_derivation.rs` already tests identity KDF but targets the encrypt spec section. Add a parallel test annotation for the decrypt section.
- Pattern: follow `test_get_decryption_materials.rs` for the annotation format.

### Spec-Aligned Structure
The spec describes this flow:
1. Check commitment policy against parsed algorithm suite → annotate at commitment policy validation
2. Obtain decryption materials from CMM → annotate at `cmm.decrypt_materials()` call
3. Construct CMM input fields → annotate at each field assignment
4. Post-CMM validation (ESDK support, commitment policy) → annotate at `validate_commitment_policy_on_decrypt`
5. Derive data key (including identity KDF case) → annotate at `derive_data_key` call

Sub-items to annotate individually:
- Req 860 "algorithm suite not supported for ESDK" → at the post-CMM ESDK suite validation
- Req 861 "algorithm suite not supported by commitment policy" → at `validate_commitment_policy_on_decrypt`
- Req 865 "identity KDF returns plaintext data key unchanged" → at the identity KDF branch in `derive_data_key`

### Most Likely Structural Mistake
The implementer may be tempted to only fix the path prefix in `materials.rs` (changing `specification/` to `aws-encryption-sdk-specification/`). While that would fix the implementation annotations, the requirements also need `type=test` annotations. The simplest fix is to correct the path prefix in the existing test files (`test_post_cmm_validation.rs`) and add a new test annotation for requirement 865.

## Targeted Tests
- `test_post_cmm_esdk_suite_decrypt` — tests that unsupported ESDK suites fail on decrypt (req 860)
- `test_post_cmm_commitment_policy_decrypt` — tests that commitment policy violations fail on decrypt (req 861)
- `test_identity_kdf_decrypt` (NEW or existing test using NO_KDF suite) — tests identity KDF returns plaintext data key unchanged (req 865)

## Success Criteria
```bash
cargo test test_post_cmm
cargo test test_key_derivation
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `get-the-decryption-materials` section
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
