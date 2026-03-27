# Pre-Implementation Reasoning: v1-header annotations

## 1. Logical steps in this spec section

The spec section describes serializing a V1 header body with these fields in order:
1. Version (0x01)
2. Type (Customer Authenticated Encrypted Data)
3. Algorithm Suite ID
4. Message ID (random)
5. AAD (encryption context)
6. Encrypted Data Keys
7. Content Type (0x02 = framed)
8. Reserved (4 zero bytes)
9. IV Length (from algorithm suite)
10. Frame Length

## 2. Point of fulfillment for each requirement

| Req | Quote summary | Fulfillment point |
|-----|--------------|-------------------|
| 1 | "MUST be serialized with the following specifics" | Top of function, before any writes |
| 2 | "serialization order MUST follow" | Top of function (implication - structural) |
| 3 | "V1 Header Body MUST be serialized as, in order" | Top of function (implication - structural) |
| 4-5 | Version MUST be serialized / value MUST be 1.0 | Before `write_msg_format_version` |
| 6-7 | Type MUST be serialized / value MUST be Customer AED | Before `write_msg_type` |
| 8-9 | Algorithm Suite ID MUST be serialized / value MUST correspond | Before `write_esdk_suite_id` |
| 10-11 | Message ID MUST be serialized / MUST use good randomness | Before `write_message_id` |
| 12-13 | AAD MUST be serialized / value MUST be encryption context | Before `write_aad_section` |
| 14-15 | EDKs MUST be serialized / value MUST be EDKs from materials | Before `write_edks` |
| 16-17 | Content Type MUST be serialized / value MUST be 02 | Before `write_content_type` |
| 18 | Reserved MUST be serialized | Before `write_bytes(w, &RESERVED_BYTES)` |
| 19-20 | IV Length MUST be serialized / value MUST match suite | Before `write_u8(w, get_iv_length(...))` |
| 21-22 | Frame Length MUST be serialized / value MUST be frame size | Before `write_u32(w, body.frame_length)` |

## 3. Sub-items

The main requirement (Req 1) has a list of sub-items (each field). Each sub-item is a separate `[[spec]]` entry in the TOML. They should be annotated individually at each write call.

## 4. Reviewer readability

The function is already structured as sequential write calls matching the spec order. Each annotation pair (serialization + value) goes immediately before its write call. Top-level and order annotations go at the top. This mirrors v2_header_body.rs exactly.

## 5. Existing similar code

`v2_header_body.rs::write_v2_header_body` — exact same pattern, just for V2. Mirror its annotation placement.
