## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/shared_header_functions.rs` — Added 3 duvet annotations for message ID length (V1, V2) and byte interpretation requirements
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs` — Added `type=test` annotation for V1 16-byte message ID length requirement
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs` — Added new `test_v2_header_message_id` test with `type=test` annotation for V2 32-byte message ID length requirement

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/shared_header_functions.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs
```

### Requirements Addressed
- ✅ `The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.` — implemented at `read_message_id_v1` + tested in `test_v1_header_message_id`
- ✅ `The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.` — implemented at `read_message_id_v2` + tested in `test_v2_header_message_id`
- ✅ `The message ID MUST be interpreted as bytes.` — annotated as `type=implication` at `write_message_id` (structural: `Vec<u8>` type enforces byte interpretation, not runtime-testable)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs`, `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs`
- **Number of `type=test` annotations added**: 2 for 2 testable requirements (the 3rd requirement uses `type=implication` which satisfies both implementation and test checks)
- **Test function names**: `test_v1_header_message_id` (existing, added annotation), `test_v2_header_message_id` (new)

### Proposed Commit Message

```
feat(message-header): add message ID length and byte interpretation annotations

Add missing duvet annotations for the message-id section of the
message header specification:
- V1 message ID length (16 bytes) at read_message_id_v1
- V2 message ID length (32 bytes) at read_message_id_v2
- Byte interpretation (implication) at write_message_id

Add corresponding test annotations and a new V2 message ID test.

Spec: aws-encryption-sdk-specification/data-format/message-header.md#message-id
```

### Duvet Verification (actual command output)
```
$ make duvet
[Pre-existing errors in compliance_exceptions/encrypt.txt — no message-id errors]
```

### Test Results (actual command output)
```
$ cargo test test_v1_header_message_id
test test_v1_header_message_id ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out

$ cargo test test_v2_header_message_id
test test_v2_header_message_id ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

### Notes
- The existing `write_message_id` randomness annotation uses `specification/` path prefix (via symlink). New annotations use `aws-encryption-sdk-specification/` to match the TOML target. Both resolve to the same spec via symlink.
- The "interpreted as bytes" requirement is `type=implication` because no test can assert "interpreted as bytes" — it's a structural property of the `Vec<u8>` type.
- Pre-existing clippy errors (missing docs on unrelated functions) and duvet errors (in compliance_exceptions/encrypt.txt) are not related to these changes.
