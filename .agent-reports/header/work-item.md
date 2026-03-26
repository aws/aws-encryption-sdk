# Work Item: Add Missing `#structure` Serialization Order Annotation and Tests

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `structure`
- **Duvet Target**: `specification/data-format/message-header.md#structure`

## Type of Work
ADD_TESTS (missing implementation annotation + missing test annotations for both requirements)

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The header MUST be serialized as, in order,
  Header Body,
  and Header Authentication.
  ```
- **Current State**: missing (no implementation annotation anywhere in codebase)

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The message header is a sequence of bytes that MUST be in big-endian format.
  ```
- **Current State**: needs-test (implementation annotation exists in `header.rs:29` and `serialize_functions.rs:86`, but no `type=test` annotation exists anywhere)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`
```rust
/// Serialize the message header (body + auth tag) to the output stream.
pub(crate) fn serialize_header(
    header: &HeaderInfo,
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    let mut w = Vec::new();
    serialize_functions::write_bytes(&mut w, &header.raw_header)?;
    header_auth::write_header_auth_tag(&mut w, &header.header_auth, &header.suite)?;
    serialize_functions::write_bytes(out, &w)?;
    serialize_functions::write_bytes(dw, &w)?;
    Ok(())
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_encrypt_decrypt() {
    // ... sets up KMS keyring, encrypts, decrypts, asserts plaintext matches
    // This exercises serialize_header via encrypt() -> step_construct_header() -> serialize_header()
}
```

## Implementation Guidance

### For Requirement 1 (missing implementation annotation):
- Add the annotation immediately before the `serialize_header` function body's first write call, or at the function signature level.
- The function writes `raw_header` (Header Body) then calls `write_header_auth_tag` (Header Authentication) — this is the exact serialization order the spec requires.
- Place the annotation before `let mut w = Vec::new();` inside `serialize_header`, since that's where the ordered serialization begins.

### For Requirement 2 (needs test):
- The `test_encrypt_decrypt` test in `test_encrypt_decrypt.rs` exercises the full encrypt→decrypt round-trip, which includes header serialization (big-endian) and the serialization order (body then auth).
- Add `type=test` annotations for both requirements to this test.
- Follow the pattern used in `test_construct_a_frame.rs` and `test_construct_the_body.rs` for test annotations.

### Spec-Aligned Structure
The spec describes this flow:
1. Header is big-endian bytes → annotate at `serialize_functions::write_u16` (already done) and at `write_header_body` (already done)
2. Header = Header Body + Header Authentication in order → annotate at `serialize_header` body (the two `write_bytes`/`write_header_auth_tag` calls)

### Pattern to Follow
Reference `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_a_frame.rs` for test annotation patterns — it has multiple `type=test` annotations on a single test function.

## Targeted Tests
- `test_encrypt_decrypt` — exercises full encrypt path including `serialize_header` which writes header body then header auth in order, using big-endian serialization

## Success Criteria
```bash
cargo test test_encrypt_decrypt
make duvet
```
- [ ] `test_encrypt_decrypt` passes
- [ ] duvet report shows no gaps for `specification/data-format/message-header.md#structure`
- [ ] Requirement 1 has `type=implementation` annotation at `serialize_header`
- [ ] Both requirements have corresponding `type=test` annotations
