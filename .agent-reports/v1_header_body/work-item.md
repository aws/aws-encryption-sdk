# Work Item: Add Missing `encrypt.md#v1-header` Implementation Annotations to `v1_header_body.rs`

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `v1-header`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/encrypt.md#v1-header`
- **Secondary Target**: `aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-1-0`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

The source file `v1_header_body.rs` has ZERO implementation annotations for `encrypt.md#v1-header` (21 MUST requirements). The companion file `v2_header_body.rs` has all equivalent `encrypt.md#v2-header` annotations and serves as the exact pattern to follow. Test annotations already exist in `tests/test_v1_header_body.rs` for all 21 requirements.

Additionally, the `data-format/message-header.md#header-body-version-1-0` serialization order requirement is missing its implementation annotation.

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
  then the [message header body](../data-format/message-header.md#header-body-version-10) MUST be serialized with the following specifics:
  ```
- **Current State**: missing
- **Placement**: At the top of `write_v1_header_body`, before any write calls

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The serialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification.
  ```
- **Current State**: missing
- **Placement**: At the top of `write_v1_header_body`, as `type=implication` with reason (structural enforcement by sequential writes)

### Requirement 3 (data-format spec)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The V1 Header Body MUST be serialized as, in order,
  Version,
  Type,
  Algorithm Suite ID,
  Message ID,
  AAD,
  Encrypted Data Keys,
  Content Type,
  Reserved,
  IV Length,
  and Frame Length.
  ```
- **Current State**: missing
- **Placement**: At the top of `write_v1_header_body`, as `type=implication` with reason (structural enforcement by sequential writes)

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Version](../data-format/message-header.md#version): MUST be serialized according to the
  [Version](../data-format/message-header.md#version) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_msg_format_version` call

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
  ```
- **Current State**: missing
- **Placement**: Before `write_msg_format_version` call (same block as Req 4)

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Type](../data-format/message-header.md#type): MUST be serialized according to the
  [Type](../data-format/message-header.md#type) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_msg_type` call

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
  ```
- **Current State**: missing
- **Placement**: Before `write_msg_type` call (same block as Req 6)

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
  [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_esdk_suite_id` call

### Requirement 9
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
  ```
- **Current State**: missing
- **Placement**: Before `write_esdk_suite_id` call (same block as Req 8)

### Requirement 10
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
  [Message ID](../data-format/message-header.md#message-id) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_message_id` call

### Requirement 11
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The process used to generate this identifier MUST use a good source of randomness
  to make the chance of duplicate identifiers negligible.
  ```
- **Current State**: missing
- **Placement**: Before `write_message_id` call (same block as Req 10)

### Requirement 12
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
  [AAD](../data-format/message-header.md#aad) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_aad_section` call

### Requirement 13
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
  in the [encryption materials](../framework/structures.md#encryption-materials),
  and this serialization MUST NOT contain any key value pairs listed in
  the [encryption material's](../framework/structures.md#encryption-materials)
  [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
  ```
- **Current State**: missing
- **Placement**: Before `write_aad_section` call (same block as Req 12)

### Requirement 14
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
  [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_edks` call

### Requirement 15
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the serialization of the
  [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
  ```
- **Current State**: missing
- **Placement**: Before `write_edks` call (same block as Req 14)

### Requirement 16
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
  [Content Type](../data-format/message-header.md#content-type) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_content_type` call

### Requirement 17
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be [02](../data-format/message-header.md#supported-content-types).
  ```
- **Current State**: missing
- **Placement**: Before `write_content_type` call (same block as Req 16)

### Requirement 18
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Reserved](../data-format/message-header.md#reserved): MUST be serialized according to the
  [Reserved](../data-format/message-header.md#reserved) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_bytes(w, &RESERVED_BYTES)` call

### Requirement 19
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [IV Length](../data-format/message-header.md#iv-length): MUST be serialized according to the
  [IV Length](../data-format/message-header.md#iv-length) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_u8(w, get_iv_length(...))` call

### Requirement 20
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST match the [IV length](../framework/algorithm-suites.md#iv-length)
  specified by the [algorithm suite](../framework/algorithm-suites.md).
  ```
- **Current State**: missing
- **Placement**: Before `write_u8(w, get_iv_length(...))` call (same block as Req 19)

### Requirement 21
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
  [Frame Length](../data-format/message-header.md#frame-length) specification.
  ```
- **Current State**: missing
- **Placement**: Before `write_u32(w, body.frame_length)` call

### Requirement 22
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the value of the frame size determined above.
  ```
- **Current State**: missing
- **Placement**: Before `write_u32(w, body.frame_length)` call (same block as Req 21)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs`
```rust
pub(crate) fn write_v1_header_body(
    w: &mut dyn SafeWrite,
    body: &V1HeaderBody,
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#header-body-version-1-0
    //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
    write_msg_format_version(w, MessageFormatVersion::V1)?;
    write_msg_type(w, body.message_type)?;
    write_esdk_suite_id(w, &body.algorithm_suite)?;
    //= specification/data-format/message-header.md#message-id
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    write_message_id(w, &body.message_id)?;
    write_aad_section(w, &body.encryption_context)?;
    write_edks(w, &body.encrypted_data_keys)?;
    write_content_type(w, body.content_type)?;
    write_bytes(w, &RESERVED_BYTES)?;
    //= specification/data-format/message-header.md#iv-length
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    write_u8(w, get_iv_length(&body.algorithm_suite))?;
    write_u32(w, body.frame_length)
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs`
All 21 `encrypt.md#v1-header` requirements already have `type=test` annotations. No test changes needed.

## Implementation Guidance
- **Pattern to follow**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs` — this file has the identical structure with all `encrypt.md#v2-header` annotations. Mirror its annotation placement exactly.
- This is annotation-only work. No code logic changes are needed.
- The existing `data-format` annotations should be kept. The new `encrypt.md#v1-header` annotations are ADDITIONAL annotations on the same code lines.
- Use `specification/client-apis/encrypt.md#v1-header` as the annotation path (using the `specification/` symlink, matching the convention in the existing codebase).
- For the serialization order requirements (Req 2 and Req 3), use `type=implication` with `reason=` explaining structural enforcement, matching the v2 pattern.

### Spec-Aligned Structure
The spec describes this flow — annotate each at the corresponding write call:
1. Top-level "MUST be serialized with the following specifics" → at function entry, before any writes
2. Serialization order (encrypt.md) → `type=implication` at function entry
3. Serialization order (data-format) → `type=implication` at function entry
4. Version → at `write_msg_format_version`
5. Type → at `write_msg_type`
6. Algorithm Suite ID → at `write_esdk_suite_id`
7. Message ID → at `write_message_id`
8. AAD → at `write_aad_section`
9. Encrypted Data Keys → at `write_edks`
10. Content Type → at `write_content_type`
11. Reserved → at `write_bytes(w, &RESERVED_BYTES)`
12. IV Length → at `write_u8(w, get_iv_length(...))`
13. Frame Length → at `write_u32(w, body.frame_length)`

Sub-items to annotate individually:
- Each field's "MUST be serialized according to" + its value constraint → before the corresponding write call
- Group the "serialized according to" and "value MUST" quotes in the same annotation block before the write call

### Most Likely Structural Mistake
The implementer may be tempted to put all annotations at the top of the function. Instead, each field's annotations MUST go immediately before the write call for that field, matching the v2 pattern. The top-level and serialization-order annotations go at the top; field-specific annotations go before each write.

## Targeted Tests
- `test_v1_header_body::test_v1_header_serialized` — tests top-level serialization
- `test_v1_header_body::test_v1_header_version` — tests version field
- `test_v1_header_body::test_v1_header_type` — tests type field
- `test_v1_header_body::test_v1_header_algorithm_suite_id` — tests algorithm suite ID
- `test_v1_header_body::test_v1_header_message_id` — tests message ID
- `test_v1_header_body::test_v1_header_aad` — tests AAD
- `test_v1_header_body::test_v1_header_encrypted_data_keys` — tests EDKs
- `test_v1_header_body::test_v1_header_content_type` — tests content type
- `test_v1_header_body::test_v1_header_reserved` — tests reserved bytes
- `test_v1_header_body::test_v1_header_iv_length` — tests IV length
- `test_v1_header_body::test_v1_header_frame_length` — tests frame length
- `test_v1_header_body::test_v1_header_serialization_order` — tests serialization order

## Success Criteria
```bash
cargo test --test test_v1_header_body
make duvet
```
- [ ] Each test passes (no code changes, tests already pass)
- [ ] duvet report shows no gaps for `encrypt.md#v1-header` section
- [ ] duvet report shows no gaps for `data-format/message-header.md#header-body-version-1-0` section
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test` (already present)
