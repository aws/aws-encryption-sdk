# Work Item: Add Test Annotations for decrypt-the-message-body in body.rs

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `decrypt-the-message-body`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Regular frame deserialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
  ```
- **Current State**: needs-test

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
  ```
- **Current State**: needs-test

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If deserializing [framed data](../data-format/message-body.md#framed-data),
  the Decrypt operation MUST use the first 4 bytes of a frame to determine
  whether the operation will deserialize the frame as a [final frame](../data-format/message-body.md#final-frame)
  or [regular frame](../data-format/message-body.md#regular-frame).
  ```
- **Current State**: needs-test

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the first 4 bytes have a value of 0xFFFF,
  then the Decrypt operation MUST deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
  ```
- **Current State**: needs-test

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Otherwise, the Decrypt operation MUST deserialize the bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
  ```
- **Current State**: needs-test

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If deserializing a [final frame](../data-format/message-body.md#final-frame),
  the Decrypt operation MUST ensure that the length of the encrypted content field is
  less than or equal to the frame length deserialized in the message header.
  ```
- **Current State**: needs-test

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Once at least a single frame is deserialized (or the entire body in the un-framed case),
  the Decrypt operation MUST decrypt and authenticate the frame (or body) using the
  [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
  specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
  ```
- **Current State**: needs-test

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If this is framed data and the first frame sequentially, this value MUST be 1.
  ```
- **Current State**: needs-test

### Requirement 9
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Otherwise, this value MUST be 1 greater than the value of the sequence number
  of the previous frame.
  ```
- **Current State**: needs-test

### Requirement 10
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
  equal to the length of the plaintext that was encrypted.
  ```
- **Current State**: needs-test

### Requirement 11
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If this decryption fails, this operation MUST immediately halt and fail.
  ```
- **Current State**: needs-test

### Requirement 12
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  This operation MUST NOT release any unauthenticated plaintext.
  ```
- **Current State**: needs-test

### Requirement 13
- **Level**: SHOULD
- **Exact Quote** (from TOML):
  ```toml
  - If the streamed Decrypt operation is using an algorithm suite with a signature algorithm,
  all plaintext decrypted from regular frames SHOULD be released as soon as the above calculation,
  including tag verification, succeeds.
  ```
- **Current State**: needs-test

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
```rust
pub(crate) fn read_and_decrypt_framed_message_body(
    r: &mut dyn SafeRead,
    w: &mut dyn SafeWrite,
    header: &HeaderInfo,
    key: &[u8],
    raw: &mut dyn SafeWrite,
    fail_if_multi_frame: bool,
) -> Result<Vec<u8>, Error> {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=implementation
    //# Regular frame deserialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=implementation
    //# Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# If this is framed data and the first frame sequentially, this value MUST be 1.
    let mut expected_frame: u32 = START_SEQUENCE_NUMBER;
    // ... (full function in body.rs)
}
```

### Test File: `NEW FILE NEEDED: AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs`
```rust
// No test file exists yet for decrypt-the-message-body annotations
```

## Implementation Guidance
- Follow the pattern established in `test_construct_the_body.rs` and `test_construct_a_frame.rs`
- Use the same `test_keyring()`, `encrypt_with_frame_length()`, and `round_trip()` helper pattern
- Tests prove decrypt correctness via round-trip: encrypt → decrypt → assert plaintext matches
- Tests prove error handling by tampering with ciphertext bytes and asserting decrypt fails
- For the "first frame sequentially, this value MUST be 1" requirement, a single-frame decrypt proves seq=1
- For the "1 greater than the value of the sequence number of the previous frame" requirement, a multi-frame decrypt proves incrementing
- For "If this decryption fails, this operation MUST immediately halt and fail", tamper with auth tag or encrypted content and assert error
- For "This operation MUST NOT release any unauthenticated plaintext", tamper with ciphertext and verify no partial output
- For the SHOULD requirement about streaming release, a multi-frame round-trip with a signing algorithm suite proves frames are released after tag verification
- Reference `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs` for the `find_body_start()` and `count_frames()` helpers
- Reference `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/fixtures.rs` for shared test fixtures

### Spec-Aligned Structure
The spec describes this flow for framed data decryption:
1. Read first 4 bytes → annotate at frame type determination test
2. If 0xFFFFFFFF → deserialize as final frame → annotate at final frame deserialization test
3. Otherwise → deserialize as regular frame → annotate at regular frame deserialization test
4. Validate sequence numbers (first=1, subsequent=prev+1) → annotate at sequence number tests
5. Validate final frame content length <= frame length → annotate at content length validation test
6. Decrypt and authenticate each frame → annotate at round-trip decrypt test
7. On decryption failure → halt and fail → annotate at tampered ciphertext test
8. Do not release unauthenticated plaintext → annotate at tampered ciphertext test (verify no partial output)

## Targeted Tests
- `test_decrypt_regular_frame_deserialization` — round-trip multi-frame message proves regular frame deserialization conforms to spec
- `test_decrypt_final_frame_deserialization` — round-trip proves final frame deserialization conforms to spec
- `test_decrypt_uses_first_4_bytes_to_determine_frame_type` — multi-frame decrypt proves frame type detection works
- `test_decrypt_final_frame_detected_by_endframe_marker` — single-frame decrypt proves 0xFFFFFFFF detection
- `test_decrypt_regular_frame_detected_without_endframe` — multi-frame decrypt proves regular frame path
- `test_decrypt_final_frame_content_length_validation` — tamper final frame content length to exceed frame length, assert error
- `test_decrypt_authenticates_each_frame` — round-trip proves authenticated decryption
- `test_decrypt_first_frame_sequence_number_is_one` — single-frame decrypt proves seq=1
- `test_decrypt_sequence_numbers_increment` — multi-frame decrypt proves incrementing sequence numbers
- `test_decrypt_content_length_in_aad` — round-trip proves content length in AAD is correct
- `test_decrypt_fails_on_tampered_auth_tag` — tamper auth tag, assert immediate failure
- `test_decrypt_no_unauthenticated_plaintext_released` — tamper ciphertext, verify no partial output
- `test_decrypt_streaming_releases_regular_frames` — multi-frame round-trip with signing suite

## Success Criteria
```bash
cargo test test_decrypt_the_message_body
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `decrypt-the-message-body` section
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
