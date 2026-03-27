# Work Item: Add Missing IV and Authentication Tag Annotations + Fix Quote Mismatch in header_auth.rs

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md` and `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `iv`, `authentication-tag`, `v1-authentication-tag`
- **Duvet Targets**:
  - `aws-encryption-sdk-specification/data-format/message-header.md#iv`
  - `aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag`
  - `aws-encryption-sdk-specification/client-apis/encrypt.md#v1-authentication-tag`

## Type of Work
FIX_ANNOTATION + ADD_TESTS

## Requirements to Address

### Requirement 1 (FIX_ANNOTATION)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  With the authentication tag calculated,
  if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
  this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
  ```
- **Current State**: The implementation annotation in `header_auth.rs` line 17-22 has "is 1.0," (with comma) but the TOML/spec has "is 1.0" (no comma). The quote must be corrected to match the TOML exactly.
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/encrypt.md#v1-authentication-tag`

### Requirement 2 (MISSING IMPLEMENTATION)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
  ```
- **Current State**: missing
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#iv`

### Requirement 3 (MISSING IMPLEMENTATION)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The IV MUST be interpreted as bytes.
  ```
- **Current State**: missing
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#iv`

### Requirement 4 (MISSING IMPLEMENTATION)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
  ```
- **Current State**: missing
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag`

### Requirement 5 (ALREADY COVERED — NO ACTION NEEDED)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The authentication tag MUST be interpreted as bytes.
  ```
- **Current State**: Already annotated in `encrypt.rs` line 481-483 with `type=implication`. No additional annotation needed in `header_auth.rs`.
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag`

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs`

Fix the v1 annotation quote (line 17-22) — remove the comma after "1.0":
```rust
        //= specification/client-apis/encrypt.md#v1-authentication-tag
        //# With the authentication tag calculated, if the message format version associated
        //# with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0, 
        //# this operation MUST serialize the
        //# [message header authentication](../data-format/message-header.md#header-authentication-version-1-0)
        //# with the following specifics:
```

Add IV length and "interpreted as bytes" annotations in `read_header_auth_tag_v1` at the `read_vec` call for IV (line ~86):
```rust
    let header_iv = read_vec(r, get_iv_length(suite) as usize, raw)?;
```

Add authentication tag length annotation in `read_header_auth_tag_v1` and `read_header_auth_tag_v2` at the `read_vec` call for tag:
```rust
    let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`

Existing tests that exercise these requirements through round-trip:
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_v1_header_auth_serialization_order() { ... }

#[tokio::test(flavor = "multi_thread")]
async fn test_v2_header_auth_serialization() { ... }
```

## Implementation Guidance

### Fix Requirement 1 (Quote Mismatch)
In `header_auth.rs`, fix the v1 annotation at line 17-22. Replace:
```
//# With the authentication tag calculated, if the message format version associated
//# with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0, 
```
With (remove comma after 1.0, match TOML exactly):
```
//# With the authentication tag calculated,
//# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
//# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-1-0) with the following specifics:
```

### Add Requirements 2-4 (Missing Annotations)
Follow the pattern from `body.rs` lines 88-98 which annotates identical structural requirements:

```rust
//= specification/data-format/message-body.md#regular-frame-iv
//= type=implication
//# The IV length MUST be equal to the IV length of the algorithm suite...
let mut iv = vec![0u8; get_iv_length(&header.suite) as usize];
```

For `read_header_auth_tag_v1`, annotate the `read_vec` for IV:
```rust
//= aws-encryption-sdk-specification/data-format/message-header.md#iv
//= type=implication
//= reason=read_vec reads exactly get_iv_length(suite) bytes, enforcing the IV length equals the algorithm suite's IV length
//# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
//= aws-encryption-sdk-specification/data-format/message-header.md#iv
//= type=implication
//= reason=the IV is stored as Vec<u8> and handled as raw bytes throughout
//# The IV MUST be interpreted as bytes.
let header_iv = read_vec(r, get_iv_length(suite) as usize, raw)?;
```

For `read_header_auth_tag_v1` and `read_header_auth_tag_v2`, annotate the `read_vec` for tag:
```rust
//= aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag
//= type=implication
//= reason=read_vec reads exactly get_tag_length(suite) bytes, enforcing the tag length equals the algorithm suite's authentication tag length
//# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
let header_auth_tag = read_vec(r, get_tag_length(suite) as usize, raw)?;
```

### Add Test Annotations
Add `type=test` annotations to the existing round-trip tests in `test_header_auth.rs`. The `test_v1_header_auth_serialization_order` test already exercises IV length and byte interpretation through successful round-trip. Add annotations to it:

```rust
//= aws-encryption-sdk-specification/data-format/message-header.md#iv
//= type=test
//# The length of the serialized IV MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.

//= aws-encryption-sdk-specification/data-format/message-header.md#iv
//= type=test
//# The IV MUST be interpreted as bytes.

//= aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag
//= type=test
//# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
```

### Spec-Aligned Structure
The spec describes this flow:
1. Header Authentication Version 1.0: IV then Authentication Tag → annotate at `write_header_auth_tag_v1` (already done) and `read_header_auth_tag_v1` (add IV/tag length annotations)
2. Header Authentication Version 2.0: Authentication Tag only → annotate at `write_header_auth_tag_v2` (already done) and `read_header_auth_tag_v2` (add tag length annotation)
3. IV section: length constraint + byte interpretation → annotate at `read_vec` for IV in `read_header_auth_tag_v1`
4. Authentication Tag section: length constraint → annotate at `read_vec` for tag in both `read_header_auth_tag_v1` and `read_header_auth_tag_v2`

Sub-items to annotate individually:
- IV length MUST be equal → at `read_vec(r, get_iv_length(suite) as usize, raw)` in `read_header_auth_tag_v1`
- IV MUST be interpreted as bytes → at same `read_vec` call
- Auth tag length MUST be equal → at `read_vec(r, get_tag_length(suite) as usize, raw)` in both v1 and v2 read functions

### Pattern References
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs` lines 88-98 — identical pattern for IV/tag length implication annotations
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/encrypt.rs` lines 481-483 — pattern for "interpreted as bytes" implication annotation

## Targeted Tests
- `test_v1_header_auth_serialization_order` — V1 round-trip proves IV and tag are correctly serialized/deserialized with correct lengths
- `test_v2_header_auth_serialization` — V2 round-trip proves tag is correctly serialized/deserialized with correct length
- `test_v1_encrypt_header_auth_tag_serialization` — V1 encrypt-specific round-trip

## Success Criteria
```bash
cargo test --manifest-path AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/Cargo.toml test_header_auth
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `data-format/message-header.md#iv`
- [ ] duvet report shows no gaps for `data-format/message-header.md#authentication-tag`
- [ ] duvet report shows no gaps for `client-apis/encrypt.md#v1-authentication-tag`
- [ ] The v1 annotation quote mismatch is fixed (comma removed after "1.0")
- [ ] All requirements have `type=implication` (not `type=todo`) for structural properties
- [ ] All implementations have corresponding `type=test`
