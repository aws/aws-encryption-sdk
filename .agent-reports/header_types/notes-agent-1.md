# Agent 1 Notes — header_types.rs

## Spec-Aligned Structure Analysis (Step 6.8)

### 1. Logical Flow of Relevant Spec Sections

The spec sections relevant to `header_types.rs` define **type enumerations and field constraints**:

- `#version` → field length constraint (1 byte)
- `#supported-versions` → enum values: `01` = v1.0, `02` = v2.0
- `#type` → field length constraint (1 byte) + value validation
- `#supported-types` → enum values: `80` = Customer AED
- `#content-type` → field length constraint (1 byte) + value validation
- `#supported-content-types` → enum values: `01` = NonFramed, `02` = Framed

### 2. Where Each Requirement Is Fulfilled in Code

| Requirement | Code Construct |
|---|---|
| "The supported versions MUST be:" | `enum MessageFormatVersion` definition |
| "- `01` MUST be version 1.0" | `V1 = 1` variant |
| "- `02` MUST be version 2.0" | `V2 = 2` variant |
| "The supported types MUST be:" | `enum MessageType` definition |
| "- `80` MUST be Customer Authenticated Encrypted Data" | `TypeCustomerAed = 0x80` variant |
| "The length of the serialized version field MUST be 1 byte." | `write_u8`/`read_u8` calls in `write_msg_format_version`/`read_msg_format_version` |
| "The length of the serialized type field MUST be 1 byte." | `write_u8`/`read_u8` calls in `write_msg_type`/`read_msg_type` |
| "The type (hex) of this field MUST be a value that exists in the following table:" | `read_msg_type` match arms + error |
| "The length of the serialized content type field MUST be 1 byte." | `write_u8`/`read_u8` calls in `write_content_type`/`read_content_type` |

### 3. Sub-items Under Normative Requirements

- `#supported-versions` has 2 sub-items (list items for `01` and `02`)
- `#supported-types` has 1 sub-item (list item for `80`)
- `#supported-content-types` has 2 sub-items (already annotated)

### 4. Most Likely Structural Mistake

The implementer may be tempted to annotate the field-length requirements
(e.g., "The length of the serialized version field MUST be 1 byte")
at the enum definition rather than at the serialization function.
The field-length constraint is fulfilled by the `read_u8`/`write_u8` call,
not by the enum definition itself.

The enum definition fulfills the "supported X MUST be" requirements.
The serialization functions fulfill the "length MUST be 1 byte" requirements.

## Potential Spec Gaps

### 1. `read_msg_format_version` rejects unknown versions
- **Code location**: `header_types.rs:43` — `_ => ser_err("Unsupported Version.")`
- **Behavior**: Returns error on any version byte that isn't 0x01 or 0x02
- **Why it matters**: Interoperability — future versions would be rejected
- **Suggested spec requirement**: "If the version field contains a value not listed in the supported versions table, deserialization MUST fail."

### 2. `read_msg_type` rejects unknown types
- **Code location**: `header_types.rs:52` — `_ => ser_err("Unsupported Message Type.")`
- **Behavior**: Returns error on any type byte that isn't 0x80
- **Why it matters**: Interoperability — ensures only known message types are processed
- **Suggested spec requirement**: "If the type field contains a value not listed in the supported types table, deserialization MUST fail."

### 3. `read_content_type` rejects unknown content types
- **Code location**: `header_types.rs:63` — `_ => ser_err("Unsupported Content Type.")`
- **Behavior**: Returns error on any content type byte that isn't 0x01 or 0x02
- **Why it matters**: Already partially covered by "The value (hex) of this field MUST be a value that exists in the following table" but the error behavior is implicit.
