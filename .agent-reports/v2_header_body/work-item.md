# Work Item: Add Missing Serialization Order Annotations for V2 Header Body

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md` and `aws-encryption-sdk-specification/data-format/message-header.md`
- **Section**: `v2-header` and `header-body-version-2-0`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header` and `aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-2-0`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.
  ```
- **Current State**: missing
- **Source TOML**: `compliance/aws-encryption-sdk-specification/client-apis/encrypt/v2-header.toml`

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The V2 Header Body MUST be serialized as, in order,
  Version,
  Algorithm Suite ID,
  Message ID,
  AAD,
  Encrypted Data Keys,
  Content Type,
  Frame Length,
  and Algorithm Suite Data.
  ```
- **Current State**: missing
- **Source TOML**: `compliance/aws-encryption-sdk-specification/data-format/message-header/header-body-version-2-0.toml`

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs`
```rust
pub(crate) fn write_v2_header_body(
    w: &mut dyn SafeWrite,
    body: &V2HeaderBody,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#v2-header
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0
    //# then the [message header body](../data-format/message-header.md#header-body-version-2-0) MUST be serialized with the following specifics:

    //= specification/client-apis/encrypt.md#v2-header
    //# - [Version](../data-format/message-header.md#version): MUST be serialized according to the
    //# [Version](../data-format/message-header.md#version) specification.
    //# The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).
    //= specification/data-format/message-header.md#header-body-version-2-0
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    write_msg_format_version(w, MessageFormatVersion::V2)?;
    // ... remaining write calls in order ...
    write_bytes(w, &body.suite_data)
}
```

The `read_v2_header_body` function reads fields in the same order (algorithm_suite, message_id, encryption_context, encrypted_data_keys, content_type, frame_length, suite_data).

### Test File: NEW FILE NEEDED: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs`

No test file currently exists for v2 header body serialization.

## Implementation Guidance

### Adding the two missing implementation annotations

Both requirements describe the same structural property: the serialization order of V2 header body fields. The code already fulfills this — the `write_v2_header_body` function writes fields in the correct order, and `read_v2_header_body` reads them in the same order.

1. **Requirement 1** (`encrypt.md#v2-header` — serialization order): Add this annotation in `write_v2_header_body`, immediately after the last existing annotation block and before `write_bytes(w, &body.suite_data)` (the last write call). This is the point where the full ordering is complete. Alternatively, place it right after the opening annotation block (the "MUST be serialized with the following specifics" annotation) since the entire function body IS the ordering.

2. **Requirement 2** (`data-format/message-header.md#header-body-version-2-0` — "The V2 Header Body MUST be serialized as, in order..."): Add this annotation at the top of `write_v2_header_body`, grouped with the existing opening annotation. This is the data-format spec's version of the same ordering constraint.

**Recommended placement**: Both annotations should go at the top of `write_v2_header_body`, right after the existing "MUST be serialized with the following specifics" annotation and before the first `write_msg_format_version` call. The function body's sequential write calls are the fulfillment of the ordering requirement.

Use `type=implication` with a `reason=` for both, since the ordering is structurally enforced by the sequential function body — it's not a runtime check.

### Adding test annotations

Create a new test file `tests/test_v2_header_body.rs` that encrypts data using the public API (which uses a V2 algorithm suite by default) and then inspects the serialized ciphertext to verify the V2 header body field ordering.

Follow the pattern in `tests/test_construct_the_body.rs`:
- Use `aws_esdk::*` public API
- Use `fixtures::*` helpers
- Use `#[tokio::test(flavor = "multi_thread")]`
- Encrypt with a committing algorithm suite (default is V2)
- Parse the ciphertext bytes to verify field order

The test should:
1. Encrypt a small plaintext
2. Read the first byte and verify it's `0x02` (Version)
3. Read the next 2 bytes as Algorithm Suite ID
4. Read the next 32 bytes as Message ID (V2 uses 32-byte message IDs)
5. Parse the AAD section (variable length, self-describing)
6. Parse the EDK section (variable length)
7. Read 1 byte as Content Type
8. Read 4 bytes as Frame Length
9. Read remaining header bytes as Algorithm Suite Data
10. Verify the order matches the spec

### Spec-Aligned Structure
The spec describes this flow:
1. Version (1 byte, value `02`) → annotate at `write_msg_format_version` call (already done)
2. Algorithm Suite ID (2 bytes) → annotate at `write_esdk_suite_id` call (already done)
3. Message ID (32 bytes) → annotate at `write_message_id` call (already done)
4. AAD (variable) → annotate at `write_aad_section` call (already done)
5. Encrypted Data Keys (variable) → annotate at `write_edks` call (already done)
6. Content Type (1 byte) → annotate at `write_content_type` call (already done)
7. Frame Length (4 bytes) → annotate at `write_u32` call (already done)
8. Algorithm Suite Data (variable) → annotate at `write_bytes` call (already done)
9. **Overall ordering constraint** → annotate at function body start (MISSING — this work item)

### Patterns to follow
- `tests/test_construct_the_body.rs` — test file structure, async test pattern, encrypt-then-inspect approach
- `tests/test_construct_a_frame.rs` — duvet test annotation format
- `src/message/v2_header_body.rs` lines 17-19 — existing annotation grouping pattern

### Structural mistake to avoid
Do NOT annotate the ordering requirement at the function signature or at a single write call. The ordering is fulfilled by the ENTIRE function body's sequential structure. Annotate at the function body start, before the first write call, where the sequential ordering begins.

## Targeted Tests
- `test_v2_header_body_serialization_order` — Encrypt with default (V2) algorithm suite, parse ciphertext header bytes, verify fields appear in spec-mandated order: Version, Algorithm Suite ID, Message ID, AAD, EDKs, Content Type, Frame Length, Algorithm Suite Data.

## Success Criteria
```bash
cargo test test_v2_header_body
make duvet
```
- [ ] Test passes
- [ ] duvet report shows no gaps for `encrypt.md#v2-header` section
- [ ] duvet report shows no gaps for `data-format/message-header.md#header-body-version-2-0` section
- [ ] Both ordering requirements have implementation annotations (not `type=todo`)
- [ ] Both ordering requirements have corresponding `type=test` annotations
