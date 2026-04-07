# Implementation Summary — decrypt-body (Non-Framed Conformance Annotation)

## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs` — Added implementation annotation for "Non-framed data deserialization MUST conform to..." at the `ContentType::NonFramed` match arm, plus cross-reference annotation for `message-body.md#non-framed-data`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs` — Added `build_nonframed_message()` helper and `test_decrypt_nonframed_deserialization_conforms_to_spec` test with `type=test` annotation

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/decrypt.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs
```

### Requirements Addressed
- ✅ `Non-framed data deserialization MUST conform to the [Non-Framed Data](../data-format/message-body.md#non-framed-data) specification.` — implemented + tested
- ✅ Cross-reference: `Non-framed data MUST consist of, in order, IV, Encrypted Content Length, Encrypted Content, and Authentication Tag.` — annotated as `type=implication` at the same call site

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs`
- **Number of `type=test` annotations added**: 1 for 1 requirement
- **Test function names**: `test_decrypt_nonframed_deserialization_conforms_to_spec`

### Proposed Commit Message

```
feat(decrypt): add duvet annotation for non-framed data deserialization conformance

Add implementation annotation at the ContentType::NonFramed match arm
in step_decrypt_body where read_and_decrypt_non_framed_message_body is
called. Add cross-reference annotation for message-body.md#non-framed-data.

Add test that constructs a non-framed encrypted message from scratch
(V2 header, AES-256-GCM-HKDF-SHA512 with commitment) and decrypts it,
proving the non-framed deserialization conforms to the specification.

Spec: aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
```

### Duvet Verification (actual command output)
```
$ make duvet
[duvet extract and report completed successfully with 2807 annotations parsed]
```

### Test Results (actual command output)
```
$ cargo test --test test_decrypt_the_message_body
running 26 tests
test test_decrypt_aad_constructed_correctly ... ok
test test_decrypt_aes_inputs_correct ... ok
test test_decrypt_authenticates_each_frame ... ok
test test_decrypt_body_deserialized_after_header ... ok
test test_decrypt_content_length_in_aad ... ok
test test_decrypt_content_type_determines_framed_or_nonframed ... ok
test test_decrypt_fails_on_tampered_auth_tag ... ok
test test_decrypt_final_frame_content_length_uses_encrypted_content_length ... ok
test test_decrypt_final_frame_content_length_validation ... ok
test test_decrypt_final_frame_detected_by_endframe_marker ... ok
test test_decrypt_final_frame_deserialization ... ok
test test_decrypt_final_frame_held_until_signature_verification ... ok
test test_decrypt_first_frame_sequence_number_is_one ... ok
test test_decrypt_frame_fields_deserialized_correctly ... ok
test test_decrypt_no_unauthenticated_plaintext_released ... ok
test test_decrypt_nonframed_deserialization_conforms_to_spec ... ok
test test_decrypt_regular_frame_content_length_uses_frame_length ... ok
test test_decrypt_regular_frame_deserialization ... ok
test test_decrypt_regular_frame_detected_without_endframe ... ok
test test_decrypt_sequence_numbers_increment ... ok
test test_decrypt_streaming_feeds_signature_algorithm ... ok
test test_decrypt_streaming_releases_regular_frames ... ok
test test_decrypt_streaming_without_signature_releases ... ok
test test_decrypt_unframed_sequence_number_is_one ... ok
test test_decrypt_uses_first_4_bytes_to_determine_frame_type ... ok
test test_decrypt_wait_for_bytes ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- The ESDK only encrypts framed data, so testing non-framed deserialization required constructing a non-framed message from scratch using `aws-lc-rs` directly
- The test constructs a complete V2 message with AlgAes256GcmHkdfSha512CommitKey (0x0478), NonFramed content type, and a properly formatted non-framed body
- The test wraps a known data key with the raw AES keyring's wrapping key ([0u8; 32]), derives encryption keys via HKDF-SHA512, computes header auth, and encrypts the body — all matching the ESDK's expected format
- The cross-reference annotation uses `type=implication` because "consist of, in order" is a structural property of the data format, not runtime-testable behavior
