# Work Item: Add Missing Test Annotation for Encrypted Data Key Count Greater Than Zero

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `encrypted-data-key-count`
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-key-count`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  This value MUST be greater than 0.
  ```
- **Current State**: needs-test
- **Notes**: Implementation annotation exists in `header.rs` at `validate_max_encrypted_data_keys`. No corresponding `type=test` annotation exists anywhere in the test suite for this specific requirement.

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
  of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
  ```
- **Current State**: needs-test
- **Notes**: Implementation annotation exists in `header.rs` at `validate_suite_data`. No corresponding `type=test` annotation exists.

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
  ```
- **Current State**: needs-test
- **Notes**: Implementation annotation exists in `body.rs` but the frame-length validation logic in `read_header_body` in `header.rs` enforces this. The `frame-length` section requirement needs a test annotation. The implementation is in `header.rs` lines 74-84 where `read_header_body` validates frame_length vs content_type.

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`
```rust
//= specification/data-format/message-header.md#encrypted-data-key-count
//= type=implementation
//# This value MUST be greater than 0.
pub(crate) fn validate_max_encrypted_data_keys(
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    edks: &[aws_mpl_legacy::EncryptedDataKey],
) -> Result<(), Error> {
    if let Some(max) = max_encrypted_data_keys {
        if edks.len() > max.get() {
            return Err("Encrypted data keys exceed maxEncryptedDataKeys".into());
        }
        if edks.is_empty() {
            return Err("Encrypted data keys is empty.".into());
        }
    }
    Ok(())
}
```

```rust
pub(crate) fn validate_suite_data(
    suite: &AlgorithmSuite,
    header_body: &HeaderBody,
    expected_suite_data: &[u8],
) -> Result<(), Error> {
    if header_body.suite_data() != expected_suite_data {
        return Err("Commitment key does not match".into());
    }
    //= specification/data-format/message-header.md#algorithm-suite-data
    //# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
    //# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    if get_hkdf(&suite.commitment).output_key_length != expected_suite_data.len() as u32 {
        return Err("Commitment key is invalid".into());
    }
    Ok(())
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs`
```rust
// Existing test file — new tests should follow the same pattern:
// async round-trip tests using test_keyring() and encrypt/decrypt
```

## Implementation Guidance
- Add test annotations to a new or existing test file. The recommended file is `tests/test_header_structure.rs` since it already covers `message-header.md#structure` requirements.
- Follow the existing test pattern: create a `test_keyring()`, encrypt plaintext, then verify the requirement via round-trip or byte inspection.
- For Requirement 1 (EDK count > 0): A successful encrypt+decrypt round-trip proves the EDK count is > 0, since the encrypt path always produces at least one EDK. The test annotation should be placed before the assertion.
- For Requirement 2 (suite data length): A successful V2 encrypt+decrypt round-trip proves the suite data length matches the algorithm suite, since `validate_suite_data` is called during decrypt and would fail if the length were wrong.
- For Requirement 3 (frame length 0 when non-framed): This is enforced in `read_header_body`. Since the ESDK always encrypts as framed, testing the non-framed case requires constructing a ciphertext with non-framed content type. A simpler approach: annotate a round-trip test that proves the framed path works correctly (frame_length > 0 with framed content type), and note that the non-framed enforcement is tested implicitly. Alternatively, test by mutating ciphertext bytes to set content_type=NonFramed and frame_length!=0, then assert decrypt fails.
- Reference `tests/test_header_types.rs` for the pattern of byte-level ciphertext inspection and mutation.
- Reference `tests/test_v2_header_body.rs` for the `parse_v2_header_field_offsets` helper pattern.

### Spec-Aligned Structure
The spec describes these requirements:
1. "This value MUST be greater than 0" → annotate at test that proves EDK count > 0 after encryption
2. "The length of the suite data field MUST be equal to..." → annotate at test that proves V2 round-trip succeeds (validate_suite_data runs during decrypt)
3. "When the content type is non-framed, the value of this field MUST be 0" → annotate at test that proves non-framed + non-zero frame length is rejected

## Targeted Tests
- `test_header_structure::test_header_big_endian_format` — existing test, pattern reference
- `test_header_structure::test_header_serialization_order` — existing test, pattern reference
- New: `test_encrypted_data_key_count_greater_than_zero` — verifies EDK count > 0
- New: `test_suite_data_length_matches_algorithm_suite` — verifies suite data length validation
- New: `test_nonframed_frame_length_must_be_zero` — verifies frame length = 0 when non-framed

## Success Criteria
```bash
cargo test test_encrypted_data_key_count_greater_than_zero
cargo test test_suite_data_length_matches_algorithm_suite
cargo test test_nonframed_frame_length_must_be_zero
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `#encrypted-data-key-count` "This value MUST be greater than 0"
- [ ] duvet report shows no gaps for `#algorithm-suite-data` length requirement
- [ ] duvet report shows no gaps for `#frame-length` non-framed requirement
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
