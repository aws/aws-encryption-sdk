# Agent 1 Notes — header.rs / message-header.md#structure

## Spec-Aligned Structure Analysis (Step 6.8)

### 1. Spec Section Logical Flow

The `#structure` section describes the top-level message header format:
1. The header is big-endian bytes
2. The header is composed of Header Body + Header Authentication, serialized in that order

### 2. Where Each Requirement Is Fulfilled in Code

| Requirement | Code Construct |
|---|---|
| "MUST be in big-endian format" | `write_u16`/`write_u32` in `serialize_functions.rs` using `.to_be_bytes()`, and `write_header_body` in `header.rs` |
| "MUST be serialized as, in order, Header Body, and Header Authentication" | `serialize_header` in `header.rs` — writes `raw_header` then `header_auth` |

### 3. Sub-items

No sub-items under these requirements.

### 4. Most Likely Structural Mistake

The implementer might be tempted to annotate the serialization order requirement at the `write_header_body` function, but that only writes the body — not the body+auth sequence. The correct location is `serialize_header` which writes both in order.

Another risk: the big-endian requirement is already annotated in two places (header.rs and serialize_functions.rs). The duplicate in serialize_functions.rs is arguably more precise (it's on the actual `.to_be_bytes()` call), but both are valid.

## Potential Spec Gaps (Step 6.7)

### 1. Frame length validation on deserialization

- **Code location**: `header.rs`, `read_header_body()`, lines ~72-85
- **Behavior**: After reading the header body, the code validates that framed content has frame_length > 0 and non-framed content has frame_length == 0
- **Why it matters**: Correctness — prevents invalid header states from being accepted during decryption
- **Suggested spec requirement**: "When deserializing, if the content type is Framed, the frame length MUST be greater than 0. If the content type is Non-Framed, the frame length MUST be 0."

Note: The `frame-length.toml` has "When the content type is non-framed, the value of this field MUST be 0" which covers the non-framed case. But the framed case (frame_length > 0) is not in the spec — it's only enforced in code.

### 2. Max encrypted data keys validation is conditional

- **Code location**: `header.rs`, `validate_max_encrypted_data_keys()`
- **Behavior**: The validation only runs when `max_encrypted_data_keys` is `Some`. When `None`, no validation occurs (including no empty-check).
- **Why it matters**: Correctness — the spec says "This value MUST be greater than 0" unconditionally, but the code only enforces this when a max is configured.
- **Suggested spec requirement**: Clarify whether the >0 check is always required or only when max_encrypted_data_keys is configured.

## Coverage Summary for header.rs

### Annotated in header.rs (implementation):
- `#structure` — "MUST be in big-endian format" ✅
- `#encrypted-data-key-count` — "This value MUST be greater than 0." ✅
- `#message-id` — "MUST use a good source of randomness" ✅
- `#algorithm-suite-data` — "length MUST be equal to Algorithm Suite Data Length" ✅

### Missing from header.rs:
- `#structure` — "The header MUST be serialized as, in order, Header Body, and Header Authentication." ❌ (at `serialize_header`)

### No test annotations exist for any `#structure` requirements anywhere in the codebase.
