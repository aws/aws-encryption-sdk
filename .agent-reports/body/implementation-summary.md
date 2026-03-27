## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs` — NEW: 13 test functions with `type=test` annotations for all 13 requirements in `decrypt.md#decrypt-the-message-body`

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs
```

### Requirements Addressed
- ✅ "Regular frame deserialization MUST conform to the [Regular Frame]..." — tested via multi-frame round-trip
- ✅ "Final frame deserialization MUST conform to the [Final Frame]..." — tested via single-frame round-trip
- ✅ "the Decrypt operation MUST use the first 4 bytes of a frame to determine..." — tested via multi-frame round-trip
- ✅ "If the first 4 bytes have a value of 0xFFFF..." — tested via single-frame decrypt (only final frame)
- ✅ "Otherwise, the Decrypt operation MUST deserialize the bytes according to the [regular frame spec]..." — tested via multi-frame decrypt
- ✅ "the Decrypt operation MUST ensure that the length of the encrypted content field is less than or equal to the frame length..." — tested by tampering content length to exceed frame length
- ✅ "the Decrypt operation MUST decrypt and authenticate the frame..." — tested via multi-frame round-trip
- ✅ "If this is framed data and the first frame sequentially, this value MUST be 1." — tested via single-frame decrypt
- ✅ "Otherwise, this value MUST be 1 greater than the value of the sequence number of the previous frame." — tested via multi-frame decrypt
- ✅ "The [content length]...MUST have a value equal to the length of the plaintext that was encrypted." — tested via round-trip with mixed frame sizes
- ✅ "If this decryption fails, this operation MUST immediately halt and fail." — tested by tampering auth tag
- ✅ "This operation MUST NOT release any unauthenticated plaintext." — tested by tampering ciphertext and verifying error (no partial output)
- ✅ "all plaintext decrypted from regular frames SHOULD be released as soon as the above calculation..." — tested via multi-frame round-trip with signing algorithm suite

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs`
- **Number of `type=test` annotations added**: 13 for 13 requirements
- **Test function names**:
  1. `test_decrypt_regular_frame_deserialization`
  2. `test_decrypt_final_frame_deserialization`
  3. `test_decrypt_uses_first_4_bytes_to_determine_frame_type`
  4. `test_decrypt_final_frame_detected_by_endframe_marker`
  5. `test_decrypt_regular_frame_detected_without_endframe`
  6. `test_decrypt_final_frame_content_length_validation`
  7. `test_decrypt_authenticates_each_frame`
  8. `test_decrypt_first_frame_sequence_number_is_one`
  9. `test_decrypt_sequence_numbers_increment`
  10. `test_decrypt_content_length_in_aad`
  11. `test_decrypt_fails_on_tampered_auth_tag`
  12. `test_decrypt_no_unauthenticated_plaintext_released`
  13. `test_decrypt_streaming_releases_regular_frames`

### Proposed Commit Message

```
test(decrypt): add test annotations for decrypt-the-message-body

Add 13 test functions covering all requirements in
decrypt.md#decrypt-the-message-body. Tests verify:
- Regular and final frame deserialization conformance
- Frame type detection via first 4 bytes (0xFFFFFFFF vs sequence number)
- Final frame content length validation (<= frame length)
- Authenticated decryption of each frame
- Sequence number correctness (first=1, subsequent=prev+1)
- Content length in AAD matches plaintext length
- Immediate failure on decryption error
- No unauthenticated plaintext release
- Regular frame release with signing algorithm suite

Spec: aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 32ms
    Scanning sources
     Scanned 158 sources 2ms
     Parsing annotations
      Parsed 1008 annotations 30ms
     Loading specifications
      Loaded 14 specifications 20ms
     Mapping sections
      Mapped 132 sections 20ms
    Matching references
     Matched 1870 references 2ms
     Sorting references
      Sorted 1870 references 12ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 22ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 1ms
```

### Test Results (actual command output)
```
$ cargo test --test test_decrypt_the_message_body
running 13 tests
test test_decrypt_final_frame_detected_by_endframe_marker ... ok
test test_decrypt_regular_frame_deserialization ... ok
test test_decrypt_streaming_releases_regular_frames ... ok
test test_decrypt_first_frame_sequence_number_is_one ... ok
test test_decrypt_final_frame_content_length_validation ... ok
test test_decrypt_no_unauthenticated_plaintext_released ... ok
test test_decrypt_fails_on_tampered_auth_tag ... ok
test test_decrypt_final_frame_deserialization ... ok
test test_decrypt_content_length_in_aad ... ok
test test_decrypt_sequence_numbers_increment ... ok
test test_decrypt_regular_frame_detected_without_endframe ... ok
test test_decrypt_authenticates_each_frame ... ok
test test_decrypt_uses_first_4_bytes_to_determine_frame_type ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### Notes
- All clippy warnings are pre-existing (missing_docs on encrypt_stream, collapsible_if in header parsing) — none from the new test file
- Test patterns follow the established convention from `test_construct_the_body.rs` and `test_construct_a_frame.rs`
- Round-trip tests prove correctness because any mismatch in AAD, IV, key, or tag would cause AES-GCM authenticated decryption to fail
- Negative tests (tampered auth tag, tampered ciphertext, invalid content length) prove error handling
