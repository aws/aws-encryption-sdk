# Work Item: Annotate Version, Type, and Content-Type Definitions in header_types.rs

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md`
- **Sections**: `#supported-versions`, `#supported-types`, `#version`, `#type`, `#content-type`
- **Duvet Targets**:
  - `specification/data-format/message-header.md#supported-versions`
  - `specification/data-format/message-header.md#supported-types`
  - `specification/data-format/message-header.md#version`
  - `specification/data-format/message-header.md#type`
  - `specification/data-format/message-header.md#content-type`

## Type of Work
ADD_ANNOTATIONS (missing implementation annotations on existing code)

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The supported versions MUST be:
  ```
- **Current State**: missing
- **Sub-items**:
  ```toml
  - `01` MUST be version 1.0
  ```
  ```toml
  - `02` MUST be version 2.0
  ```

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - `01` MUST be version 1.0
  ```
- **Current State**: missing

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - `02` MUST be version 2.0
  ```
- **Current State**: missing

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The supported types MUST be:
  ```
- **Current State**: missing
- **Sub-items**:
  ```toml
  - `80` MUST be Customer Authenticated Encrypted Data
  ```

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - `80` MUST be Customer Authenticated Encrypted Data
  ```
- **Current State**: missing

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized version field MUST be 1 byte.
  ```
- **Current State**: missing

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized type field MUST be 1 byte.
  ```
- **Current State**: missing

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The type (hex) of this field MUST be a value that exists in the following table:
  ```
- **Current State**: missing

### Requirement 9
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the serialized content type field MUST be 1 byte.
  ```
- **Current State**: missing

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_types.rs`
```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum MessageFormatVersion {
    V1 = 1,
    V2 = 2,
}

pub(crate) fn write_msg_format_version(
    w: &mut dyn SafeWrite,
    data: MessageFormatVersion,
) -> Result<(), Error> {
    write_u8(w, data as u8)
}
pub(crate) fn write_msg_type(w: &mut dyn SafeWrite, data: MessageType) -> Result<(), Error> {
    write_u8(w, data as u8)
}
pub(crate) fn write_content_type(w: &mut dyn SafeWrite, data: ContentType) -> Result<(), Error> {
    write_u8(w, data as u8)
}

pub(crate) fn read_msg_format_version(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageFormatVersion, Error> {
    let version = read_u8(r, raw)?;
    match version {
        val if val == MessageFormatVersion::V1 as u8 => Ok(MessageFormatVersion::V1),
        val if val == MessageFormatVersion::V2 as u8 => Ok(MessageFormatVersion::V2),
        _ => ser_err("Unsupported Version."),
    }
}
pub(crate) fn read_msg_type(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageType, Error> {
    let msg_type = read_u8(r, raw)?;
    match msg_type {
        val if val == MessageType::TypeCustomerAed as u8 => Ok(MessageType::TypeCustomerAed),
        _ => ser_err("Unsupported Message Type."),
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum MessageType {
    #[default]
    TypeCustomerAed = 0x80,
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs`
```rust
// Existing tests cover only content-type:
// - test_content_type_framed_value
// - test_content_type_nonframed_value
// - test_content_type_invalid_value_rejected
```

## Implementation Guidance

### Implementation Annotations (in header_types.rs)

Follow the exact pattern already used for `ContentType` enum annotations.

1. **`MessageFormatVersion` enum** — annotate like `ContentType` with `type=implication`:
   - Parent annotation `"The supported versions MUST be:"` before the enum
   - Sub-item `"- \`01\` MUST be version 1.0"` before `V1 = 1`
   - Sub-item `"- \`02\` MUST be version 2.0"` before `V2 = 2`

2. **`MessageType` enum** — same pattern:
   - Parent annotation `"The supported types MUST be:"` before the enum
   - Sub-item `"- \`80\` MUST be Customer Authenticated Encrypted Data"` before `TypeCustomerAed = 0x80`

3. **Field-length annotations** — annotate at the serialization functions with `type=implication` + `reason=`:
   - `"The length of the serialized version field MUST be 1 byte."` before `write_msg_format_version` (or `read_msg_format_version`) — the `write_u8`/`read_u8` call structurally constrains the field to 1 byte
   - `"The length of the serialized type field MUST be 1 byte."` before `write_msg_type` (or `read_msg_type`)
   - `"The length of the serialized content type field MUST be 1 byte."` before `write_content_type` (or `read_content_type`)

4. **Value validation annotation** — `"The type (hex) of this field MUST be a value that exists in the following table:"` before `read_msg_type` match block (same pattern as existing `read_content_type` annotation)

### Spec-Aligned Structure

The spec describes these type definitions:
1. `#supported-versions` → annotate at `enum MessageFormatVersion` definition (implication)
2. `#supported-versions` sub-items → annotate at each variant (implication)
3. `#supported-types` → annotate at `enum MessageType` definition (implication)
4. `#supported-types` sub-item → annotate at `TypeCustomerAed` variant (implication)
5. `#version` field length → annotate at `write_msg_format_version` or `read_msg_format_version` (implication)
6. `#type` field length → annotate at `write_msg_type` or `read_msg_type` (implication)
7. `#type` value validation → annotate at `read_msg_type` match (implementation, like existing `read_content_type`)
8. `#content-type` field length → annotate at `write_content_type` or `read_content_type` (implication)

Sub-items to annotate individually:
- `"- \`01\` MUST be version 1.0"` → at `V1 = 1` enum variant
- `"- \`02\` MUST be version 2.0"` → at `V2 = 2` enum variant
- `"- \`80\` MUST be Customer Authenticated Encrypted Data"` → at `TypeCustomerAed = 0x80` enum variant

### Pattern to Follow

Reference: the existing `ContentType` enum annotations at lines 183-196 of `header_types.rs`:
```rust
//= specification/data-format/message-header.md#supported-content-types
//= type=implication
//= reason=The enum definition structurally constrains valid content types to exactly these two variants.
//# The supported content types MUST be:
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum ContentType {
    //= specification/data-format/message-header.md#supported-content-types
    //= type=implication
    //# - `01` for [Non-Framed](message-body.md#non-framed-data)
    NonFramed = 1,
    //= specification/data-format/message-header.md#supported-content-types
    //= type=implication
    //# - `02` for [Framed](message-body.md#framed-data)
    #[default]
    Framed = 2,
}
```

And the existing `read_content_type` annotation at lines 53-54:
```rust
//= specification/data-format/message-header.md#content-type
//# The value (hex) of this field MUST be a value that exists in the following table:
pub(crate) fn read_content_type(
```

### Test Annotations

Tests are needed for the newly annotated requirements. Follow the pattern in `test_header_types.rs`.

For `#supported-versions`:
- Test that a V1 ciphertext has version byte `0x01` at offset 0
- Test that a V2 ciphertext has version byte `0x02` at offset 0

For `#supported-types`:
- Test that a V1 ciphertext has type byte `0x80` at offset 1

For `#version` and `#type` field lengths:
- These are structural (1-byte read/write) — `type=implication` satisfies both impl and test

For `#content-type` field length:
- Structural — `type=implication` satisfies both impl and test

For `#type` value validation:
- Test that an invalid type byte (e.g., `0x00`) causes deserialization to fail

## Targeted Tests
- `test_content_type_framed_value` — existing, covers `#supported-content-types` `02` sub-item
- `test_content_type_nonframed_value` — existing, covers `#supported-content-types` `01` sub-item
- `test_content_type_invalid_value_rejected` — existing, covers `#content-type` value validation
- NEW: `test_version_v1_value` — verify V1 ciphertext starts with `0x01`
- NEW: `test_version_v2_value` — verify V2 ciphertext starts with `0x02`
- NEW: `test_type_customer_aed_value` — verify V1 ciphertext has `0x80` at offset 1
- NEW: `test_type_invalid_value_rejected` — verify invalid type byte causes error

## Success Criteria
```bash
cargo test test_header_types
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `#supported-versions`, `#supported-types`, `#version`, `#type`, `#content-type` sections
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test` or `type=implication`
