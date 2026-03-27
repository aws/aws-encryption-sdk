# Agent 1 Notes — v2_header_body

## Spec-Aligned Structure Analysis

### 1. Spec Section Logical Flow

**`encrypt.md#v2-header`** describes 8 field serializations plus a serialization order requirement:
1. Gate condition: algorithm suite version is 2.0 → annotate at function entry
2. Version field → `write_msg_format_version`
3. Algorithm Suite ID → `write_esdk_suite_id`
4. Message ID → `write_message_id`
5. AAD → `write_aad_section`
6. Encrypted Data Keys → `write_edks`
7. Content Type → `write_content_type`
8. Frame Length → `write_u32`
9. Algorithm Suite Data → `write_bytes`
10. Serialization order → structural (sequential calls)

**`data-format/message-header.md#header-body-version-2-0`** has 2 requirements:
1. Version field MUST be `02`
2. Serialization order

### 2. Where Each Requirement Is Fulfilled

All implementation annotations are already placed correctly in `write_v2_header_body`.
The gap is entirely in **test annotations** — the test file only covers 2 of the 20 TOML requirements.

### 3. Sub-items

The `encrypt.md#v2-header` spec has 8 field-level sub-requirements, each split into:
- A "MUST be serialized according to" requirement
- A "value MUST be" requirement

These are separate `[[spec]]` entries in the TOML and need individual test annotations.

### 4. Most Likely Structural Mistake

The implementer may be tempted to write a single test that covers "serialization order" and assume all field-level requirements are covered. Each TOML `[[spec]]` entry needs its own `type=test` annotation at a test that exercises that specific field.

## Potential Spec Gaps

### read_v2_header_body commitment check
- **Code location**: `v2_header_body.rs` line ~97: `if !has_hkdf(&algorithm_suite.commitment)`
- **Why it matters**: Correctness — the deserialization rejects non-committing suites for V2 headers, but the spec doesn't explicitly state this constraint for the data format section.
- **Suggested spec requirement**: "When deserializing a Version 2.0 header body, the algorithm suite MUST support key commitment."
