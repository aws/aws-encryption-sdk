# Agent 1 Notes — shared_header_functions.rs

## Spec Section Logical Flow (Step 6.8)

The `message-header.md#message-id` section describes:
1. Purpose: uniquely identifies the message
2. Randomness: MUST use good source of randomness (annotated)
3. V1 length: MUST be 16 bytes (NOT annotated)
4. V2 length: MUST be 32 bytes (NOT annotated)
5. Interpretation: MUST be interpreted as bytes (NOT annotated)

### Where each requirement is fulfilled in code

| Requirement | Code construct |
|---|---|
| Randomness | `generate_message_id` in `header.rs` calls `generate_random_bytes` |
| V1 = 16 bytes | `read_message_id_v1` passes `MESSAGE_ID_LEN_V1` (16) to `read_vec` |
| V2 = 32 bytes | `read_message_id_v2` passes `MESSAGE_ID_LEN_V2` (32) to `read_vec` |
| Interpreted as bytes | `MessageId` is `Vec<u8>`, `write_bytes` writes raw bytes |

### Most likely structural mistake
Annotating the `MESSAGE_ID_LEN_V1`/`MESSAGE_ID_LEN_V2` constants in `header_types.rs` instead of the call sites in `shared_header_functions.rs`. The constants define values; the functions enforce the requirement.

## Additional Gaps in shared_header_functions.rs (not selected)

### Algorithm Suite ID — 2-byte length
- **TOML quote**: `The length of the serialized algorithm suite ID field MUST be 2 bytes.`
- **Status**: Missing annotation everywhere in codebase
- **Code**: `read_esdk_suite_id` uses `[0; 2]` buffer — structurally enforces 2 bytes
- **Priority**: Lower than message-id gaps (single requirement vs cluster of 3)

### Message ID randomness — incomplete quote
- The annotation at line 47-49 quotes `implementations MUST use a good source of randomness when generating messages IDs in order to make the chance of duplicate IDs negligible.`
- The TOML quote starts with `While implementations cannot guarantee complete uniqueness,` which is missing from the annotation.
- Duvet may still match this since the MUST clause is present. Verify after fixing the higher-priority gaps.

## Potential Spec Gaps

### 1. MessageId type alias hides byte interpretation
- **Code location**: `header_types.rs` — `pub(crate) type MessageId = Vec<u8>;`
- **Behavior**: The type alias makes `MessageId` a `Vec<u8>`, which structurally enforces byte interpretation. There is no way to accidentally interpret it as anything else in Rust.
- **Why it matters**: Correctness — the spec says "MUST be interpreted as bytes" but in Rust this is enforced by the type system, not runtime code.
- **Suggested spec note**: No change needed; the `type=implication` annotation type handles this correctly.

### 2. No validation of message ID length on write path
- **Code location**: `write_message_id` in `shared_header_functions.rs`
- **Behavior**: `write_message_id` writes whatever `message_id` slice is passed, with no length validation. If a caller passes a 15-byte or 33-byte message ID, it would be written without error.
- **Why it matters**: Correctness / interop — a malformed message ID would produce a non-compliant message that other implementations may reject.
- **Suggested spec requirement**: "The serialized message ID MUST have the length specified for the header version." (This is arguably already covered by the existing length requirements, but the write path doesn't enforce it.)
