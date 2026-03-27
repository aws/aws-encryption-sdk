# Work Item: Add Test Annotations for encrypt.md#input and decrypt.md#input in types.rs

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md` and `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `input`
- **Duvet Target**: `specification/client-apis/encrypt.md#input` and `specification/client-apis/decrypt.md#input`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Encrypt operation MUST accept a required [plaintext](#plaintext) argument.
  ```
- **Current State**: has `implication` annotation on `EncryptInput.plaintext` field, needs `test`
- **Sub-items**: none

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Encrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument.
  ```
- **Current State**: has `implication` annotation on `EncryptInput.source` field, needs `test`
- **Sub-items**: none

### Requirement 3
- **Level**: SHOULD
- **Exact Quote** (from TOML):
  ```toml
  The keyring and CMM inputs SHOULD be optional.
  ```
- **Current State**: missing — no annotation exists anywhere
- **Sub-items**: none

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
  ```
- **Current State**: has `implementation` annotation on `EncryptInput::validate()`, needs `test`
- **Sub-items**: none

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail.
  ```
- **Current State**: has `implementation` annotation on `EncryptInput::validate()`, needs `test`
- **Sub-items**: none

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Encrypt operation MUST accept an optional [Algorithm Suite](#algorithm-suite) argument.
  ```
- **Current State**: has `implication` annotation on `EncryptInput.algorithm_suite_id` field, needs `test`
- **Sub-items**: none

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Encrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
  ```
- **Current State**: has `implication` annotation on `EncryptInput.encryption_context` field, needs `test`
- **Sub-items**: none

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Encrypt operation MUST accept an optional [Frame Length](#frame-length) argument.
  ```
- **Current State**: has `implication` annotation on `EncryptInput.frame_length` field, needs `test`
- **Sub-items**: none

### Requirement 9
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Decrypt operation MUST accept a required [Encrypted Message](#encrypted-message) argument.
  ```
- **Current State**: has `implementation` annotation on `DecryptInput.ciphertext` field, needs `test`
- **Sub-items**: none

### Requirement 10
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Decrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument.
  ```
- **Current State**: has `implementation` annotation on `DecryptInput.source` field, needs `test`
- **Sub-items**: none

### Requirement 11
- **Level**: SHOULD
- **Exact Quote** (from TOML):
  ```toml
  The keyring and CMM inputs SHOULD be optional.
  ```
- **Current State**: missing — no annotation exists for decrypt input's SHOULD
- **Sub-items**: none

### Requirement 12
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
  ```
- **Current State**: has `implication` annotation on `DecryptInput.encryption_context` field, needs `test`
- **Sub-items**: none

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs`
```rust
// EncryptInput struct (lines 186-236) has implication annotations for:
// - plaintext field
// - source field (CMM/keyring)
// - algorithm_suite_id field
// - encryption_context field
// - frame_length field
// And implementation annotations on validate() method (lines 296-302)

// DecryptInput struct (lines 383-410) has implementation annotations for:
// - ciphertext field
// - source field
// - encryption_context field (implication)
// And implementation annotations on validate() method (lines 455-461)
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs`
```rust
// Already has tests for client.md#initialization (commitment policy, max EDKs)
// This is the natural home for encrypt/decrypt input structure tests
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`
```rust
// test_bad_encrypt_input (line 172) — tests encrypt with source=None, but has NO duvet annotation
// test_bad_decrypt_input (line 39) — tests decrypt with source=None, HAS duvet annotations
```

## Implementation Guidance

### For structural "accept" requirements (Requirements 1, 2, 6, 7, 8, 9, 10, 12):
- These are `implication`-type requirements — the struct field's existence IS the implementation.
- Add `type=test` annotations to unit tests that construct `EncryptInput`/`DecryptInput` and verify the fields exist and can be set.
- Pattern to follow: see `test_encrypt_input_custom_commitment_policy` and `test_encrypt_input_custom_max_edks` in `test_create_esdk_client.rs` — these construct the input, set a field, and assert the value.
- Add tests in `test_create_esdk_client.rs` alongside the existing client initialization tests.

### For SHOULD requirements (Requirements 3, 11):
- The `MaterialSource` is `Option<MaterialSource>`, making CMM/keyring optional by construction.
- Add `implication` annotations on the `source: Option<MaterialSource>` field declarations in both `EncryptInput` and `DecryptInput`.
- **Placement**: On the `pub source: Option<MaterialSource>` field in each struct.

### For validate/fail requirements (Requirements 4, 5):
- The existing `test_bad_encrypt_input` test in `test_encrypt_decrypt.rs` already tests this behavior.
- Add duvet `type=test` annotations to that existing test, matching the pattern used in `test_bad_decrypt_input`.

### Spec-Aligned Structure
The spec describes this flow:
1. Accept required plaintext → annotate test at assertion that `EncryptInput` has `plaintext` field
2. Accept CMM/keyring → annotate test at assertion that `EncryptInput` has `source` field
3. CMM/keyring SHOULD be optional → annotate `implication` at `source: Option<MaterialSource>` field
4. Validate exactly one → annotate test at `test_bad_encrypt_input` assertion
5. Fail if not exactly one → annotate test at `test_bad_encrypt_input` assertion
6. Accept optional Algorithm Suite → annotate test at assertion that field is `Option`
7. Accept optional Encryption Context → annotate test at assertion that field exists
8. Accept optional Frame Length → annotate test at assertion that field exists

Sub-items to annotate individually:
- Each "accept" requirement → at a test that constructs the input and verifies the field
- Each "validate/fail" requirement → at the test that sets source=None and asserts error

### Key Pattern Reference
Follow the pattern in `test_create_esdk_client.rs` lines 121-142:
```rust
#[test]
fn test_encrypt_input_custom_commitment_policy() {
    //= specification/client-apis/client.md#initialization
    //= type=test
    //# - On client initialization,
    //# the caller MUST have the option to provide a [commitment policy](#commitment-policy).
    let mut input = EncryptInput::default();
    input.commitment_policy =
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    assert_eq!(
        input.commitment_policy,
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt
    );
}
```

## Targeted Tests
- `test_bad_encrypt_input` — already exists in `test_encrypt_decrypt.rs`, needs duvet annotations added
- `test_bad_decrypt_input` — already exists with duvet annotations (no changes needed)
- NEW: `test_encrypt_input_accepts_plaintext` — verify EncryptInput accepts plaintext bytes
- NEW: `test_encrypt_input_accepts_cmm_and_keyring` — verify EncryptInput accepts MaterialSource
- NEW: `test_encrypt_input_accepts_optional_algorithm_suite` — verify field is Option
- NEW: `test_encrypt_input_accepts_optional_encryption_context` — verify field exists
- NEW: `test_encrypt_input_accepts_optional_frame_length` — verify field exists
- NEW: `test_decrypt_input_accepts_encrypted_message` — verify DecryptInput accepts ciphertext bytes
- NEW: `test_decrypt_input_accepts_cmm_and_keyring` — verify DecryptInput accepts MaterialSource
- NEW: `test_decrypt_input_accepts_optional_encryption_context` — verify field exists

## Success Criteria
```bash
cargo test test_encrypt_input_accepts
cargo test test_decrypt_input_accepts
cargo test test_bad_encrypt_input
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `encrypt.md#input` section
- [ ] duvet report shows no gaps for `decrypt.md#input` section
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
- [ ] SHOULD requirements (3, 11) have `type=implication` annotations on source fields
