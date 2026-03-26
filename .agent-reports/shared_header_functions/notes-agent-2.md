# Agent 2 Notes — shared_header_functions

## Pre-Implementation Reasoning

1. **Logical steps**: Add 3 missing annotations to existing functions in shared_header_functions.rs, add test annotations to existing+new tests.

2. **Point of fulfillment**:
   - "16 bytes for v1" → `read_vec(r, MESSAGE_ID_LEN_V1 as usize, raw)` in `read_message_id_v1`
   - "32 bytes for v2" → `read_vec(r, MESSAGE_ID_LEN_V2 as usize, raw)` in `read_message_id_v2`
   - "interpreted as bytes" → `write_bytes(w, message_id)` in `write_message_id`

3. **Sub-items**: None — each is a standalone requirement.

4. **Reviewer readability**: Each annotation goes directly above the single line it describes. Trivial.

5. **Existing patterns**: `write_esdk_suite_id` in same file has stacked annotations before the function — follow that pattern.

## Cross-reference check
- "16 bytes for [version 1.0](#header-body-version-10)" — links to same file section, not a different spec. No cross-ref needed.
- "32 bytes for [version 2.0](#header-body-version-20)" — same, internal link. No cross-ref needed.
- "interpreted as bytes" — no links. No cross-ref needed.

## Annotation types
- V1 16-byte length: `type=implication` — structural, enforced by constant. Can't test "the length is enforced" separately from "it works".
  Actually — we CAN test that the output message ID is 16 bytes. That's `implementation` (default). The test asserts `msg_id.len() == 16`.
- V2 32-byte length: Same reasoning — testable by inspecting output. Default `implementation`.
- "interpreted as bytes": `type=implication` — structural, no test can assert "interpreted as bytes" vs some other interpretation. The type system (`Vec<u8>`) enforces this.
