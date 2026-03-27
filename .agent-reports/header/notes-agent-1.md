# Agent 1 Notes — header.rs / data-format/message-header.md

## Spec-Aligned Structure (Step 6.8)

### 1. Spec Section Logical Flow
The `data-format/message-header.md` spec describes the binary format of the message header:
- Structure: big-endian, Header Body + Header Authentication
- Header Body V1: Version, Type, AlgSuiteID, MessageID, AAD, EDKs, ContentType, Reserved, IVLength, FrameLength
- Header Body V2: Version, AlgSuiteID, MessageID, AAD, EDKs, ContentType, FrameLength, AlgSuiteData
- Header Auth V1: IV + AuthTag
- Header Auth V2: AuthTag only
- Each field has length, format, and value constraints

### 2. Where Each Requirement is Fulfilled in Code
For requirements relevant to `header.rs`:
- `#structure` big-endian → `write_header_body` function signature (entry point)
- `#structure` serialization order → `serialize_header` function body (write raw_header then auth tag)
- `#message-id` randomness → `generate_message_id` function body (`generate_random_bytes` call)
- `#encrypted-data-key-count` > 0 → `validate_max_encrypted_data_keys` function body (`edks.is_empty()` check)
- `#algorithm-suite-data` length → `validate_suite_data` function body (length comparison)
- `#algorithm-suite-data` interpreted as bytes → `validate_suite_data` function body (byte slice comparison)

### 3. Sub-items Under Normative Requirements
No sub-items relevant to header.rs. The sub-item patterns (V1/V2 field lists) are in v1_header_body.rs and v2_header_body.rs.

### 4. Most Likely Structural Mistake
The main risk is annotating "interpreted as bytes" at a variable declaration rather than at the point where the byte interpretation matters (comparison, serialization). The annotation should go at the byte-level operation, not at a `let` binding.

## Potential Spec Gaps

### 1. Framed Content Must Have Positive Frame Length
- **Code location**: `header.rs` `read_header_body`, lines 76-80
- **Behavior**: If content_type is Framed and frame_length is 0, returns error "Frame length must be positive if content is framed"
- **Why it matters**: Correctness — a framed message with frame_length=0 would cause division by zero or infinite loops during body processing
- **Suggested spec requirement**: "When the [content type](#content-type) is framed, the value of the [Frame Length](#frame-length) field MUST be greater than 0."
- **Note**: The spec only says "When the content type is non-framed, the value of this field MUST be 0" — it doesn't explicitly require positive frame length for framed content.

### 2. Header Version Must Support Commitment
- **Code location**: `header.rs` `header_version_supports_commitment`, lines 87-95
- **Behavior**: V2 bodies must have suite_data length matching the HKDF output key length; V1 bodies always pass
- **Why it matters**: Security — ensures commitment key is present and correctly sized for committing algorithm suites
- **Suggested spec requirement**: Already partially covered by `#algorithm-suite-data` length requirement, but the V1/V2 version-commitment relationship is implicit.

## Duplicate Annotation Inventory

| Requirement | Locations | Correct Location |
|---|---|---|
| `#message-id` randomness | header.rs, shared_header_functions.rs, v1_header_body.rs | header.rs (generate_message_id) |
| `#structure` big-endian | header.rs, serialize_functions.rs | header.rs (write_header_body) |

## Coverage Summary for header.rs

| Requirement | Impl | Test | Status |
|---|---|---|---|
| `#structure` big-endian | ✅ header.rs | ✅ test_header_structure.rs | DUPLICATE (also in serialize_functions.rs) |
| `#structure` serialization order | ✅ header.rs | ✅ test_header_structure.rs | OK |
| `#message-id` randomness | ✅ header.rs | ❌ missing data-format test | DUPLICATE + NEEDS TEST |
| `#encrypted-data-key-count` > 0 | ✅ header.rs | ✅ test_header_structure.rs + test_encrypted_data_keys.rs | STYLE FIX (explicit type=implementation) |
| `#algorithm-suite-data` length | ✅ header.rs | ✅ test_header_structure.rs | OK |
| `#algorithm-suite-data` bytes | ❌ missing | ❌ missing | NEEDS IMPL + TEST |
