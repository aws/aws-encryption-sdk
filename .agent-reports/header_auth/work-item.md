# Work Item: Add Missing "authentication tag MUST be interpreted as bytes" Annotation

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `authentication-tag`
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The authentication tag MUST be interpreted as bytes.
  ```
- **Current State**: missing — no annotation exists in `header_auth.rs` for this requirement. An annotation exists in `encrypt.rs` using the `specification/` prefix, but the `header_auth.rs` file uses `aws-encryption-sdk-specification/` prefix for `data-format/message-header.md` annotations and is the natural home for this data-format requirement.
- **Sub-items**: none

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs`

The IV section already has the "interpreted as bytes" annotation as a pattern to follow:

```rust
pub(crate) fn read_header_auth_tag_v1(
    r: &mut dyn SafeRead,
    suite: &AlgorithmSuite,
    raw: &mut dyn SafeWrite,
) -> Result<HeaderAuth, Error> {
    //= aws-encryption-sdk-specification/data-format/message-header.md#iv
    //= type=implication
    //= reason=read_vec reads exactly get_iv_length(suite) bytes, enforcing the IV length equals the algorithm suite's IV length
    //# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    //= aws-encryption-sdk-specification/data-format/message-header.md#iv
    //= type=implication
    //= reason=the IV is stored as Vec<u8> and handled as raw bytes throughout
    //# The IV MUST be interpreted as bytes.
    let header_iv = read_vec(r, get_iv_length(suite) as usize, raw)?;
    //= aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag
    //= type=implication
    //= reason=read_vec reads exactly get_tag_length(suite) bytes, enforcing the tag length equals the algorithm suite's authentication tag length
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
    Ok(HeaderAuth::AESMac {
        header_iv,
        header_auth_tag,
    })
}
```

The `read_header_auth_tag_v2` function has the same pattern — length annotation present, "interpreted as bytes" missing:

```rust
pub(crate) fn read_header_auth_tag_v2(
    r: &mut dyn SafeRead,
    suite: &AlgorithmSuite,
    raw: &mut dyn SafeWrite,
) -> Result<HeaderAuth, Error> {
    //= aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag
    //= type=implication
    //= reason=read_vec reads exactly get_tag_length(suite) bytes, enforcing the tag length equals the algorithm suite's authentication tag length
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
    let header_iv = vec![0u8; get_iv_length(suite) as usize];
    Ok(HeaderAuth::AESMac {
        header_iv,
        header_auth_tag,
    })
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`

The IV "interpreted as bytes" test annotation exists as a pattern:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_iv_length_and_bytes() {
    //= aws-encryption-sdk-specification/data-format/message-header.md#iv
    //= type=test
    //# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    let pt = b"v1 iv length test";

    //= aws-encryption-sdk-specification/data-format/message-header.md#iv
    //= type=test
    //# The IV MUST be interpreted as bytes.
    let result = round_trip_v1(pt).await;
    assert_eq!(result, pt, "successful V1 round-trip proves IV was serialized with correct length and interpreted as bytes");
}
```

No corresponding test exists for the authentication tag "interpreted as bytes" requirement.

## Implementation Guidance

- Add `type=implication` annotation with `reason=` to `read_header_auth_tag_v1` immediately after the existing length annotation and before `let header_auth_tag = read_vec(...)`. Follow the exact pattern used for the IV "interpreted as bytes" annotation.
- Add the same annotation to `read_header_auth_tag_v2` in the same position.
- Add a `type=test` annotation in `test_header_auth.rs`. The existing `test_v1_header_auth_tag_length` and `test_v2_header_auth_tag_length` tests are the natural place — add the test annotation to one or both of these tests.
- Use `aws-encryption-sdk-specification/` prefix (not `specification/`) to match the existing convention in this file for `data-format/message-header.md` annotations.
- Reference pattern: the IV annotations in `read_header_auth_tag_v1` (lines 96-99 of `header_auth.rs`).

### Spec-Aligned Structure
The spec describes two properties of the authentication tag:
1. Length constraint → annotate at `read_vec(r, get_tag_length(suite)...)` (already done)
2. Byte interpretation → annotate at the same `let header_auth_tag = read_vec(...)` line

Annotations to add:
- `The authentication tag MUST be interpreted as bytes.` → at `let header_auth_tag = read_vec(...)` in both `read_header_auth_tag_v1` and `read_header_auth_tag_v2`
- Same quote with `type=test` → in existing test functions `test_v1_header_auth_tag_length` and/or `test_v2_header_auth_tag_length`

## Targeted Tests
- `test_v1_header_auth_tag_length` — existing test that proves V1 auth tag round-trips correctly; add test annotation here
- `test_v2_header_auth_tag_length` — existing test that proves V2 auth tag round-trips correctly; add test annotation here

## Success Criteria
```bash
cargo test -p aws-esdk --test test_header_auth
duvet report  # from AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/
make duvet    # from repo root
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `data-format/message-header.md#authentication-tag`
- [ ] The "interpreted as bytes" requirement has `type=implication` annotations in both `read_header_auth_tag_v1` and `read_header_auth_tag_v2`
- [ ] The "interpreted as bytes" requirement has corresponding `type=test` annotations
