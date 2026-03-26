## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/encrypted_data_keys.rs` — Added implementation annotation to `write_edks` for serialization order requirement; removed misplaced `#encrypted-data-key-count` annotation from `read_edks`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypted_data_keys.rs` — New test file with round-trip test proving EDK serialization order

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/encrypted_data_keys.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypted_data_keys.rs
```

### Requirements Addressed
- ✅ "The Encrypted Data Keys MUST be serialized as, in order, Encrypted Data Key Count, and Encrypted Data Key Entries." — implemented + tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypted_data_keys.rs`
- **Number of `type=test` annotations added**: 1 for 1 requirement
- **Test function names**: `test_encrypted_data_keys_serialization_order`

### Proposed Commit Message

```
feat(message-header): add duvet annotations for encrypted data keys serialization order

Add implementation annotation to write_edks for the requirement that
Encrypted Data Keys MUST be serialized as count then entries in order.
Remove misplaced encrypted-data-key-count annotation from read_edks
(the actual enforcement is in header.rs::validate_max_encrypted_data_keys).
Add round-trip test proving correct serialization order.

Spec: aws-encryption-sdk-specification/data-format/message-header.md#encrypted-data-keys
```

### Duvet Verification (actual command output)
```
$ make duvet
[extract phase completed successfully]
[report phase completed with 2 pre-existing errors in compliance_exceptions/encrypt.txt - unrelated to this change]
```

### Test Results (actual command output)
```
$ cargo test test_encrypted_data_keys_serialization_order
running 1 test
test test_encrypted_data_keys_serialization_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- The 2 duvet report errors are pre-existing in `compliance_exceptions/encrypt.txt` (about `construct-a-frame` section quotes that don't match) — unrelated to this change.
- The 8 test failures in `test_authentication_tag` are due to missing AWS credentials (KMS tests) — unrelated to this change.
- The clippy warnings/errors are all pre-existing (unused doc comments, unreachable patterns, missing docs on `encrypt_stream`) — none introduced by this change.
- The annotation quote was verified against the TOML file at `compliance/aws-encryption-sdk-specification/data-format/message-header/encrypted-data-keys.toml`.
- No markdown links in the quoted requirement text, so no cross-reference annotations needed.
