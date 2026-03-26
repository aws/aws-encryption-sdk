# Work Item: Add encrypt.md#v1-header Annotations to write_v1_header_body

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `v1-header`
- **Duvet Target**: `aws-encryption-sdk-specification/client-apis/encrypt.md#v1-header`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

The `write_v1_header_body` function in `v1_header_body.rs` implements the V1 header serialization but has zero annotations for the `encrypt.md#v1-header` spec section. The v2 counterpart (`v2_header_body.rs`) already has full annotations for `encrypt.md#v2-header`. This work adds the matching annotations to the V1 path, plus the missing serialization-order annotation from the data-format spec.

There are 21 MUST requirements in the TOML with 0 annotations. Additionally, 1 MUST requirement from `header-body-version-1-0` (serialization order) is missing.

### Requirement 1 (parent)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
  then the [message header body](../data-format/message-header.md#header-body-version-10) MUST be serialized with the following specifics:
  ```
- **Current State**: missing

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Version](../data-format/message-header.md#version): MUST be serialized according to the
  [Version](../data-format/message-header.md#version) specification.
  ```
- **Current State**: missing

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
  ```
- **Current State**: missing

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Type](../data-format/message-header.md#type): MUST be serialized according to the
  [Type](../data-format/message-header.md#type) specification.
  ```
- **Current State**: missing

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
  ```
- **Current State**: missing

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
  [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
  ```
- **Current State**: missing

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
  ```
- **Current State**: missing

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
  [Message ID](../data-format/message-header.md#message-id) specification.
  ```
- **Current State**: missing

### Requirement 9
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The process used to generate this identifier MUST use a good source of randomness
  to make the chance of duplicate identifiers negligible.
  ```
- **Current State**: missing

### Requirement 10
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
  [AAD](../data-format/message-header.md#aad) specification.
  ```
- **Current State**: missing

### Requirement 11
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

### Requirement 12
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
  [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
  ```
- **Current State**: missing

### Requirement 13
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the serialization of the
  [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
  ```
- **Current State**: missing

### Requirement 14
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
  [Content Type](../data-format/message-header.md#content-type) specification.
  ```
- **Current State**: missing

### Requirement 15
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be [02](../data-format/message-header.md#supported-content-types).
  ```
- **Current State**: missing

### Requirement 16
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Reserved](../data-format/message-header.md#reserved): MUST be serialized according to the
  [Reserved](../data-format/message-header.md#reserved) specification.
  ```
- **Current State**: missing

### Requirement 17
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [IV Length](../data-format/message-header.md#iv-length): MUST be serialized according to the
  [IV Length](../data-format/message-header.md#iv-length) specification.
  ```
- **Current State**: missing

### Requirement 18
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST match the [IV length](../framework/algorithm-suites.md#iv-length)
  specified by the [algorithm suite](../framework/algorithm-suites.md).
  ```
- **Current State**: missing

### Requirement 19
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
  [Frame Length](../data-format/message-header.md#frame-length) specification.
  ```
- **Current State**: missing

### Requirement 20
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be the value of the frame size determined above.
  ```
- **Current State**: missing

### Requirement 21
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The serialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification.
  ```
- **Current State**: missing

### Requirement 22 (data-format spec — serialization order)
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
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-1-0`

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

### Test File: NEW FILE NEEDED: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs`
No dedicated test file exists. Existing integration tests in `test_encrypt_decrypt.rs` exercise V1 header serialization indirectly through the encrypt/decrypt round-trip, but no `type=test` annotations exist for these specs.

### Pattern File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs`
The V2 counterpart has the exact annotation pattern to follow. Each `write_*` call is preceded by the corresponding `encrypt.md#v2-header` annotation block.

## Implementation Guidance

- **This is annotation-only work** — no logic changes needed. The code already correctly serializes the V1 header. The gap is missing duvet annotations.
- Follow the exact pattern from `v2_header_body.rs` — each `write_*` call gets the corresponding `encrypt.md#v1-header` annotation block immediately before it.
- The annotation prefix is `specification/` (not `aws-encryption-sdk-specification/`) — this file uses the symlink path.
- Keep existing `data-format/message-header.md` annotations in place. The new `encrypt.md#v1-header` annotations are **additional** annotations on the same code lines.
- For the parent requirement (Req 1) and serialization order (Req 21, 22), annotate at the top of the function body, before the first `write_*` call — matching the v2 pattern.
- For test annotations, create a new test file or add `type=test` annotations to existing integration tests that exercise V1 header serialization.

### Spec-Aligned Structure
The spec describes this serialization flow — annotate each at the corresponding `write_*` call:

1. Parent requirement (MUST be serialized with specifics) → annotate at function body top
2. Version MUST be serialized + value MUST correspond to 1.0 → at `write_msg_format_version(w, MessageFormatVersion::V1)?;`
3. Type MUST be serialized + value MUST correspond to Customer AED → at `write_msg_type(w, body.message_type)?;`
4. Algorithm Suite ID MUST be serialized + value MUST correspond to suite → at `write_esdk_suite_id(w, &body.algorithm_suite)?;`
5. Message ID MUST be serialized + randomness MUST → at `write_message_id(w, &body.message_id)?;`
6. AAD MUST be serialized + value MUST be EC serialization → at `write_aad_section(w, &body.encryption_context)?;`
7. Encrypted Data Keys MUST be serialized + value MUST be EDK serialization → at `write_edks(w, &body.encrypted_data_keys)?;`
8. Content Type MUST be serialized + value MUST be 02 → at `write_content_type(w, body.content_type)?;`
9. Reserved MUST be serialized → at `write_bytes(w, &RESERVED_BYTES)?;`
10. IV Length MUST be serialized + value MUST match suite → at `write_u8(w, get_iv_length(...))?;`
11. Frame Length MUST be serialized + value MUST be frame size → at `write_u32(w, body.frame_length)`
12. Serialization order MUST follow spec → at function body top (alongside parent)
13. Data-format serialization order → at function body top

Sub-items to annotate individually:
- Each field's "MUST be serialized" quote → at the corresponding `write_*` call
- Each field's value constraint quote → at the same `write_*` call (stacked annotation block)

## Targeted Tests
- `test_encrypt_decrypt` — exercises V1 header serialization through full encrypt/decrypt round-trip (can add `type=test` annotations)
- `test_encrypt_decrypt_ec` — exercises V1 header with encryption context
- `test_encrypt_decrypt_single_full_frame` — exercises V1 header with various frame lengths

## Success Criteria
```bash
cargo test test_encrypt_decrypt
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `encrypt.md#v1-header` section
- [ ] duvet report shows no gaps for `data-format/message-header.md#header-body-version-1-0` section
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
