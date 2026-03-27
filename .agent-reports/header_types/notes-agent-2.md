# Pre-Implementation Reasoning — header_types

## 1. Logical steps in this spec section

1. Define supported versions (enum `MessageFormatVersion` with V1=1, V2=2)
2. Define supported types (enum `MessageType` with TypeCustomerAed=0x80)
3. Serialize/deserialize version as 1 byte (`write_u8`/`read_u8`)
4. Serialize/deserialize type as 1 byte (`write_u8`/`read_u8`)
5. Validate type value on deserialization (match block in `read_msg_type`)
6. Serialize/deserialize content type as 1 byte (`write_u8`/`read_u8`)

## 2. Point of fulfillment for each requirement

| Requirement | Point of fulfillment |
|---|---|
| "The supported versions MUST be:" | `enum MessageFormatVersion` definition |
| "- `01` MUST be version 1.0" | `V1 = 1` variant |
| "- `02` MUST be version 2.0" | `V2 = 2` variant |
| "The supported types MUST be:" | `enum MessageType` definition |
| "- `80` MUST be Customer Authenticated Encrypted Data" | `TypeCustomerAed = 0x80` variant |
| "The length of the serialized version field MUST be 1 byte." | `write_u8` call in `write_msg_format_version` |
| "The length of the serialized type field MUST be 1 byte." | `write_u8` call in `write_msg_type` |
| "The type (hex) of this field MUST be a value that exists in the following table:" | `match` block in `read_msg_type` |
| "The length of the serialized content type field MUST be 1 byte." | `write_u8` call in `write_content_type` |

## 3. Sub-items

- `#supported-versions` has two sub-items: `01` → V1, `02` → V2
- `#supported-types` has one sub-item: `80` → TypeCustomerAed
- No sub-items for field-length requirements

## 4. Reviewer readability

The enum annotations go directly on the enum and its variants (Pattern 4).
The field-length annotations go on the write functions (type=implication).
The value-validation annotation goes on `read_msg_type` (like existing `read_content_type`).
All annotations are within 1-2 lines of the code they describe.

## 5. Existing similar code

`ContentType` enum at lines 183-196 of `header_types.rs` — exact pattern to follow.
`read_content_type` annotation at line 53-54 — pattern for value validation.
