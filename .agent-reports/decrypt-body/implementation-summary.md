## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs` — added `test_decrypt_final_frame_held_until_signature_verification` test with `type=test` annotation

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs
```

### Requirements Addressed
- ✅ "Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature) successfully completes." — tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_decrypt_the_message_body.rs`
- **Number of `type=test` annotations added**: 1 for 1 requirement
- **Test function names**: `test_decrypt_final_frame_held_until_signature_verification`

### Proposed Commit Message

```
test(decrypt): add type=test annotation for final frame hold-back requirement

Add test verifying that final frame plaintext is not released until
signature verification completes. The test encrypts with ECDSA P384,
tampers with the signature, and asserts decrypt fails — proving the
final frame was held back pending signature verification.

Spec: aws-encryption-sdk-specification/client-apis/decrypt.md#decrypt-the-message-body
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 28ms
    Scanning sources
     Scanned 165 sources 2ms
     Parsing annotations
      Parsed 1322 annotations 25ms
     Loading specifications
      Loaded 13 specifications 18ms
     Mapping sections
      Mapped 141 sections 17ms
    Matching references
     Matched 2372 references 2ms
     Sorting references
      Sorted 2372 references 16ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 21ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 969µs
```

### Test Results (actual command output)
```
$ cargo test test_decrypt_final_frame_held_until_signature_verification
running 1 test
test test_decrypt_final_frame_held_until_signature_verification ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.02s
```

### Notes
- This was the only missing `type=test` annotation for the decrypt-the-message-body section (33/33 now covered)
- Implementation annotations already existed at lines 215 and 452 of decrypt.rs from cycle 1
