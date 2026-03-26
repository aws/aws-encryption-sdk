# Pre-Implementation Reasoning — v1_header_body annotations

## 1. Logical steps in this spec section

The spec describes serializing a V1 header body with these fields in order:
1. Version (value = 01)
2. Type (value = Customer Authenticated Encrypted Data)
3. Algorithm Suite ID (value = the algorithm suite used)
4. Message ID (generated with good randomness)
5. AAD (encryption context, excluding required EC keys)
6. Encrypted Data Keys (from encryption materials)
7. Content Type (value = 02)
8. Reserved (per spec)
9. IV Length (matches algorithm suite IV length)
10. Frame Length (frame size determined above)

Plus two "meta" requirements:
- Parent: "MUST be serialized with the following specifics"
- Serialization order: "MUST follow Header Body Version 1.0 specification"

## 2. Point of fulfillment for each requirement

| Req | Fulfilled at |
|-----|-------------|
| Parent (Req 1) | Function body top — the function itself is the serialization |
| Version MUST be serialized (Req 2) | `write_msg_format_version(w, MessageFormatVersion::V1)?;` |
| Value MUST correspond to 1.0 (Req 3) | Same line — `MessageFormatVersion::V1` |
| Type MUST be serialized (Req 4) | `write_msg_type(w, body.message_type)?;` |
| Type value MUST be CAED (Req 5) | Same line — `body.message_type` |
| Algorithm Suite ID MUST be serialized (Req 6) | `write_esdk_suite_id(w, &body.algorithm_suite)?;` |
| Algorithm Suite value (Req 7) | Same line — `&body.algorithm_suite` |
| Message ID MUST be serialized (Req 8) | `write_message_id(w, &body.message_id)?;` |
| Randomness MUST (Req 9) | Same line — message_id was generated upstream |
| AAD MUST be serialized (Req 10) | `write_aad_section(w, &body.encryption_context)?;` |
| AAD value MUST be EC (Req 11) | Same line — `&body.encryption_context` |
| EDKs MUST be serialized (Req 12) | `write_edks(w, &body.encrypted_data_keys)?;` |
| EDK value (Req 13) | Same line — `&body.encrypted_data_keys` |
| Content Type MUST be serialized (Req 14) | `write_content_type(w, body.content_type)?;` |
| Content Type value = 02 (Req 15) | Same line — `body.content_type` |
| Reserved MUST be serialized (Req 16) | `write_bytes(w, &RESERVED_BYTES)?;` |
| IV Length MUST be serialized (Req 17) | `write_u8(w, get_iv_length(&body.algorithm_suite))?;` |
| IV Length value (Req 18) | Same line — `get_iv_length(...)` |
| Frame Length MUST be serialized (Req 19) | `write_u32(w, body.frame_length)` |
| Frame Length value (Req 20) | Same line — `body.frame_length` |
| Serialization order (Req 21) | Function body top — the sequential write calls |
| Data-format serialization order (Req 22) | Function body top — same |

## 3. Sub-items?

Each field is a sub-item of the parent requirement. Each has a "MUST be serialized" + a value constraint. These are separate TOML entries, so each gets its own annotation block. Following the v2 pattern, we stack the "MUST be serialized" and value constraint together before each write call.

## 4. Can a reviewer read this top-to-bottom?

Yes — the function is already structured as sequential write calls matching the spec order. Each annotation block goes immediately before its write call. The parent + serialization order annotations go at the top. This mirrors v2_header_body.rs exactly.

## 5. Existing similar code

`v2_header_body.rs::write_v2_header_body` — exact same pattern. Follow it.
