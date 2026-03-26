# Work Item: Add data-format header-authentication-version annotations and tests

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `header-authentication-version-1-0` and `header-authentication-version-2-0`
- **Duvet Target**: `specification/data-format/message-header.md#header-authentication-version-1-0` and `specification/data-format/message-header.md#header-authentication-version-2-0`

## Type of Work
NEW_IMPLEMENTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The V1 Header Authentication MUST be serialized as, in order,
  IV,
  and Authentication Tag.
  ```
- **Current State**: missing
- **Sub-items**: None (the "in order, IV, and Authentication Tag" is part of the normative quote itself)

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The V2 Header Authentication MUST be serialized as the Authentication Tag only.
  ```
- **Current State**: missing

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs`
```rust
pub(crate) fn write_header_auth_tag_v1(
    w: &mut dyn SafeWrite,
    header_auth: &HeaderAuth,
) -> Result<(), Error> {
    match header_auth {
        HeaderAuth::AESMac {
            header_iv,
            header_auth_tag,
        } => {
            //= specification/client-apis/encrypt.md#v1-authentication-tag
            //# - [IV](../data-format/message-header.md#iv): MUST have the value of the IV used in the calculation above,
            //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
            write_bytes(w, header_iv)?;
            //= specification/client-apis/encrypt.md#v1-authentication-tag
            //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
            //# of the authentication tag calculated above.
            write_bytes(w, header_auth_tag)
        }
    }
}
pub(crate) fn write_header_auth_tag_v2(
    w: &mut dyn SafeWrite,
    header_auth: &HeaderAuth,
) -> Result<(), Error> {
    match header_auth {
        HeaderAuth::AESMac {
            header_auth_tag, ..
        } => write_bytes(
            w,
            //= specification/client-apis/encrypt.md#v2-authentication-tag
            //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
            //# of the authentication tag calculated above.
            header_auth_tag
        ),
    }
}
```

### Test File: `NEW FILE NEEDED: AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`
No existing test file for header auth serialization format.

## Implementation Guidance

### Adding data-format implementation annotations

1. In `write_header_auth_tag_v1`, add the V1 data-format annotation at the top of the match arm body, before the existing client-apis annotations. This annotation describes the serialization order (IV then Auth Tag), which is exactly what the two `write_bytes` calls implement:

```rust
HeaderAuth::AESMac {
    header_iv,
    header_auth_tag,
} => {
    //= specification/data-format/message-header.md#header-authentication-version-1-0
    //# The V1 Header Authentication MUST be serialized as, in order,
    //# IV,
    //# and Authentication Tag.

    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //# - [IV](...): MUST have the value ...
    write_bytes(w, header_iv)?;
    ...
```

2. In `write_header_auth_tag_v2`, add the V2 data-format annotation before the `write_bytes` call:

```rust
HeaderAuth::AESMac {
    header_auth_tag, ..
} => {
    //= specification/data-format/message-header.md#header-authentication-version-2-0
    //# The V2 Header Authentication MUST be serialized as the Authentication Tag only.
    write_bytes(w, header_auth_tag)
}
```

Note: `write_header_auth_tag_v2` currently uses an inline expression style (`=> write_bytes(w, ... header_auth_tag)`). The annotation must be placed so the next executable line is the `write_bytes` call. This may require reformatting the match arm to use a block body.

### Adding test annotations

Create `tests/test_header_auth.rs` following the pattern in `tests/test_header_structure.rs` and `tests/test_v1_header_body.rs`:
- Use `test_keyring()` (raw AES keyring, no KMS needed)
- Use `encrypt_v1()` helper for V1 tests (requires non-committing algorithm suite)
- Use default encrypt for V2 tests (default algorithm suite is V2/committing)
- Round-trip encrypt/decrypt proves serialization format is correct

Tests need `type=test` annotations for:
- `specification/data-format/message-header.md#header-authentication-version-1-0`
- `specification/data-format/message-header.md#header-authentication-version-2-0`

### Spec-Aligned Structure
The spec describes this flow:
1. V1: serialize IV, then Authentication Tag (in order) → annotate at the match arm body in `write_header_auth_tag_v1` before the two `write_bytes` calls
2. V2: serialize Authentication Tag only → annotate at the match arm body in `write_header_auth_tag_v2` before the single `write_bytes` call

### Pattern references
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs` — example of data-format annotations alongside client-apis annotations in the same function
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs` — example of data-format test using raw AES keyring and round-trip pattern
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs` — example of V1-specific test with non-committing algorithm suite

## Targeted Tests
- `test_v1_header_auth_serialization_order` — Encrypt with V1 algorithm suite, decrypt successfully, proving V1 header auth was serialized as IV then Authentication Tag
- `test_v2_header_auth_serialization` — Encrypt with V2 (default) algorithm suite, decrypt successfully, proving V2 header auth was serialized as Authentication Tag only

## Success Criteria
```bash
cargo test test_header_auth
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `header-authentication-version-1-0` and `header-authentication-version-2-0`
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
