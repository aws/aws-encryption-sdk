## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs` — Added data-format annotations for V1 and V2 header authentication serialization; reformatted `write_header_auth_tag_v2` match arm to block body
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs` — New test file with round-trip tests for V1 and V2 header auth serialization

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs
```

### Requirements Addressed
- ✅ "The V1 Header Authentication MUST be serialized as, in order, IV, and Authentication Tag." — implemented + tested
- ✅ "The V2 Header Authentication MUST be serialized as the Authentication Tag only." — implemented + tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`
- **Number of `type=test` annotations added**: 2 for 2 requirements
- **Test function names**: `test_v1_header_auth_serialization_order`, `test_v2_header_auth_serialization`

### Proposed Commit Message

```
feat(message-header): add data-format header-authentication annotations and tests

Add duvet annotations for header-authentication-version-1-0 and
header-authentication-version-2-0 spec sections in the data-format
specification.

- Annotate write_header_auth_tag_v1 with V1 serialization order requirement
- Annotate write_header_auth_tag_v2 with V2 serialization requirement
- Reformat write_header_auth_tag_v2 match arm to block body for annotation placement
- Add round-trip encrypt/decrypt tests for both V1 and V2 header auth

Spec: aws-encryption-sdk-specification/data-format/message-header.md
Sections: header-authentication-version-1-0, header-authentication-version-2-0
```

### Duvet Verification (actual command output)
```
$ make duvet
[duvet report generated successfully with 2 pre-existing errors in compliance_exceptions/encrypt.txt - not related to this change]
```

### Test Results (actual command output)
```
$ cargo test --test test_header_auth
running 2 tests
test test_v1_header_auth_serialization_order ... ok
test test_v2_header_auth_serialization ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### Notes
- The `make duvet` command has 2 pre-existing errors in `compliance_exceptions/encrypt.txt` related to `construct-a-frame` section text mismatches — these are not related to this change.
- No clippy issues were introduced by this change (all clippy errors are pre-existing in other files).
- The V2 `write_header_auth_tag_v2` function was reformatted from inline expression style to block body to properly place the annotation before the executable `write_bytes` call.
