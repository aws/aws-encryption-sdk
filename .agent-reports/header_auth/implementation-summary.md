## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs` — Fixed v1 annotation quote (removed comma after "1.0"), added IV length/bytes and auth tag length `type=implication` annotations in `read_header_auth_tag_v1` and `read_header_auth_tag_v2`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs` — Added 3 new test functions with `type=test` annotations for IV length, IV bytes, and auth tag length requirements

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs
```

### Requirements Addressed
- ✅ Req 1 (quote fix): `With the authentication tag calculated, if the message format version associated with the [algorithm suite](...) is 1.0 this operation MUST serialize the [message header authentication](...) with the following specifics:` — fixed quote (removed comma after "1.0")
- ✅ Req 2: `The length of the serialized IV MUST be equal to the [IV length](...) value of the [algorithm suite](...) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.` — implemented (implication) + tested
- ✅ Req 3: `The IV MUST be interpreted as bytes.` — implemented (implication) + tested
- ✅ Req 4: `The length of the serialized authentication tag MUST be equal to the [authentication tag length](...) of the [algorithm suite](...) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.` — implemented (implication, in both v1 and v2) + tested (v1 and v2)
- ✅ Req 5: Already covered in encrypt.rs (no action needed)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`
- **Number of `type=test` annotations added**: 4 for 3 requirements (IV length, IV bytes, auth tag length v1, auth tag length v2)
- **Test function names**: `test_v1_header_auth_iv_length_and_bytes`, `test_v1_header_auth_tag_length`, `test_v2_header_auth_tag_length`

### Proposed Commit Message

```
feat(message-header): add IV and authentication tag data-format annotations

Add duvet annotations for data-format/message-header.md#iv and
data-format/message-header.md#authentication-tag spec sections.
Fix v1-authentication-tag quote mismatch (remove comma after "1.0").

- Annotate read_header_auth_tag_v1 with IV length and IV bytes
  implication annotations
- Annotate read_header_auth_tag_v1 and read_header_auth_tag_v2 with
  authentication tag length implication annotations
- Add round-trip tests for IV length, IV bytes, and auth tag length
- Fix v1 annotation quote to match TOML exactly

Spec: aws-encryption-sdk-specification/data-format/message-header.md
Sections: iv, authentication-tag
Spec: aws-encryption-sdk-specification/client-apis/encrypt.md
Section: v1-authentication-tag
```

### Duvet Verification (actual command output)
```
$ make duvet
[output shown above - succeeded with no errors]
```

### Test Results (actual command output)
```
$ cargo test --test test_header_auth
running 7 tests
test test_v1_header_auth_iv_length_and_bytes ... ok
test test_v1_encrypt_header_auth_tag_serialization ... ok
test test_v1_header_auth_serialization_order ... ok
test test_v1_header_auth_tag_length ... ok
test test_v2_encrypt_header_auth_tag_serialization ... ok
test test_v2_header_auth_serialization ... ok
test test_v2_header_auth_tag_length ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- All clippy errors are pre-existing (none introduced by this change)
- The `type=implication` annotations follow the pattern established in `body.rs` lines 88-98 for identical structural requirements
- The v1 annotation quote fix removes a trailing comma after "1.0" and reformats to match the TOML exactly
- Auth tag length annotation is placed in both `read_header_auth_tag_v1` and `read_header_auth_tag_v2` since both functions enforce the tag length via `read_vec(r, get_tag_length(suite) as usize, raw)`
