# Work Item: Add Missing Test Annotations for decrypt.md#get-the-decryption-materials

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `get-the-decryption-materials`
- **Duvet Target**: `specification/client-apis/decrypt.md#get-the-decryption-materials`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the parsed [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
  is not supported by the [commitment policy](client.md#commitment-policy)
  configured in the [client](client.md) decrypt MUST yield an error.
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — `validate_commitment_policy_on_decrypt()` call before CMM

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The CMM used MUST be the input CMM, if supplied.
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — annotation before `create_cmm_from_input()`

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM](../framework/default-cmm.md)
  from the input [keyring](../framework/keyring-interface.md).
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — annotation before `create_cmm_from_input()`

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The data key used as input for all decryption described below MUST be a data key derived from the plaintext data key
  included in the [decryption materials](../framework/structures.md#decryption-materials).
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — `key_derivation::derive_keys()` call

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The algorithm suite used as input for all decryption described below MUST be the algorithm suite
  included in the [decryption materials](../framework/structures.md#decryption-materials).
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — `let suite = &dec_mat.algorithm_suite`

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the [algorithm suite](../framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) supports [key commitment](../framework/algorithm-suites.md#key-commitment)
  then the [commit key](../framework/algorithm-suites.md#commit-key) MUST be derived from the plaintext data key
  using the [commit key derivation](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — `if v2_header_body::has_hkdf(&suite.commitment)` block

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The derived commit key MUST equal the commit key stored in the message header.
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — `header::validate_suite_data()` call

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The algorithm suite used to derive a data key from the plaintext data key MUST be
  the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
  [algorithm suite](../framework/algorithm-suites.md) associated with
  the returned decryption materials.
  ```
- **Current State**: needs-test
- **Implementation Location**: `decrypt.rs:step_get_decryption_materials` — `key_derivation::derive_keys()` uses `suite` from materials

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs`
```rust
// Step 2: Get the decryption materials
async fn step_get_decryption_materials(
    ciphertext: &mut dyn SafeRead,
    header_body: &header_types::HeaderBody,
    raw_header: Vec<u8>,
    input_source: Option<MaterialSource>,
    encryption_context: &EncryptionContext,
    commitment_policy: EsdkCommitmentPolicy,
) -> Result<DecryptState, Error> {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The CMM used MUST be the input CMM, if supplied.
    //# If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM](../framework/default-cmm.md)
    //# from the input [keyring](../framework/keyring-interface.md).
    let cmm = materials::create_cmm_from_input(input_source).await?;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the parsed [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    aws_mpl_legacy::commitment::validate_commitment_policy_on_decrypt(...)?;

    let dec_mat = materials::get_decryption_materials(...).await?;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The algorithm suite used as input for all decryption described below MUST be the algorithm suite
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The algorithm suite used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) associated with
    //# the returned decryption materials.
    let suite = &dec_mat.algorithm_suite;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The data key used as input for all decryption described below MUST be a data key derived from the plaintext data key
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    let derived_data_keys = key_derivation::derive_keys(...)?;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the [algorithm suite]... supports [key commitment]...
    //# then the [commit key]... MUST be derived from the plaintext data key...
    if v2_header_body::has_hkdf(&suite.commitment) {
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# The derived commit key MUST equal the commit key stored in the message header.
        header::validate_suite_data(...)?;
    }
    ...
}
```

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs`
```rust
pub(crate) async fn get_modern_decryption_materials(
    cmm: aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    // ... builds DecryptMaterialsInput with all 5 fields annotated ...
    let materials = cmm.decrypt_materials(&input).await?;
    // ... post-CMM commitment validation ...
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs`
```rust
// Existing tests cover: obtain materials via CMM, CMM call construction,
// algorithm suite ID, commitment policy, encrypted data keys,
// encryption context, reproduced encryption context, wrong keyring failure.
// All are round-trip tests using test_keyring() -> encrypt -> decrypt.
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_post_cmm_validation.rs`
```rust
// Existing tests cover: post-CMM commitment policy on decrypt (positive + negative),
// identity KDF on decrypt. These already have type=test annotations.
```

## Implementation Guidance
- All 8 missing requirements need `type=test` annotations only — the implementation annotations already exist
- Follow the existing pattern in `test_get_decryption_materials.rs` and `test_post_cmm_validation.rs`: round-trip encrypt/decrypt tests with `test_keyring()`
- **Annotation path convention**: The duvet config references specs via `specification/` (a symlink to `aws-encryption-sdk-specification/`). Existing test files use both `aws-encryption-sdk-specification/` and `specification/` prefixes — both resolve correctly. New test annotations SHOULD use `aws-encryption-sdk-specification/` to match the convention in `test_get_decryption_materials.rs`, which is the primary test file for this section.
- Requirements 1-3 (pre-CMM commitment check, CMM resolution) can be tested with simple round-trip tests that exercise the decrypt path
- Requirements 4-5 (data key derivation, algorithm suite from materials) are proven by successful round-trip decryption
- Requirements 6-7 (key commitment derivation and equality) need a test using a committing algorithm suite (v2 header)
- Requirement 8 (KDF algorithm from materials) is proven by successful round-trip with an HKDF suite
- Group related requirements into minimal tests to avoid redundancy

### Spec-Aligned Structure
The spec describes this flow:
1. Pre-CMM commitment policy check → annotate test at round-trip with non-committing suite + require policy
2. CMM resolution (input CMM) → annotate test at round-trip using CMM input
3. CMM resolution (keyring → default CMM) → annotate test at round-trip using keyring input
4. Data key derivation from materials → annotate test at successful round-trip decrypt
5. Algorithm suite from materials → annotate test at successful round-trip decrypt
6. Key commitment derivation → annotate test at round-trip with committing suite
7. Commit key equality → annotate test at round-trip with committing suite
8. KDF algorithm from materials → annotate test at round-trip with HKDF suite

### Suggested Test Groupings
- **Test A**: Round-trip with keyring input (covers Req 3: default CMM construction, Req 4: data key derivation, Req 5: algorithm suite from materials, Req 8: KDF algorithm)
- **Test B**: Round-trip with CMM input (covers Req 2: input CMM used)
- **Test C**: Pre-CMM commitment policy failure (covers Req 1: parsed suite vs commitment policy)
- **Test D**: Round-trip with committing suite (covers Req 6: commit key derivation, Req 7: commit key equality)

## Targeted Tests
- `test_get_decryption_materials::test_obtain_decryption_materials_via_cmm` — existing, can be extended
- `test_get_decryption_materials::test_decrypt_fails_with_wrong_keyring` — existing negative test
- `test_post_cmm_validation::test_post_cmm_commitment_policy_decrypt` — existing, covers post-CMM commitment
- `test_post_cmm_validation::test_decrypt_non_committing_with_require_policy_fails` — existing, covers post-CMM commitment failure
- `test_post_cmm_validation::test_identity_kdf_decrypt` — existing, covers identity KDF
- New tests needed for: pre-CMM commitment check, CMM vs keyring input, data key derivation, algorithm suite usage, key commitment derivation/equality, KDF algorithm

## Success Criteria
```bash
cargo test test_get_decryption_materials
cargo test test_post_cmm_validation
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `get-the-decryption-materials` section
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
