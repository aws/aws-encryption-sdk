# Agent 3 Review Notes — v1_header_body

## Adversarial Pre-Review

### 1. Annotation-to-code semantic check
Each annotation block was verified against its corresponding `write_*` call. Every annotation's subject (Version, Type, Algorithm Suite ID, etc.) directly matches the function being called. No mismatches found.

### 2. Annotation stacking analysis
- Function top: 3 annotation blocks (parent, serialization order encrypt.md, serialization order data-format). These are Pattern 3 (general behavior at function start). They apply to the function as a whole, not to a specific line. This matches the v2 pattern exactly.
- Before `write_msg_format_version`: 2 blocks (encrypt.md Version + data-format Version). Within limit.
- Before `write_message_id`: 2 blocks (encrypt.md Message ID + data-format message-id). Within limit.
- Before `write_u8(get_iv_length)`: 2 blocks (encrypt.md IV Length + data-format iv-length). Within limit.
- All other write calls: 1 block each. No stacking issues.

### 3. Context reset evaluation (block-by-block)
Each annotation block + its code line was evaluated in isolation:
- "Version MUST be serialized" + `write_msg_format_version(w, MessageFormatVersion::V1)` → obvious match
- "Type MUST be serialized" + `write_msg_type(w, body.message_type)` → obvious match
- "Algorithm Suite ID MUST be serialized" + `write_esdk_suite_id(w, &body.algorithm_suite)` → obvious match
- "Message ID MUST be serialized" + `write_message_id(w, &body.message_id)` → obvious match
- "AAD MUST be serialized" + `write_aad_section(w, &body.encryption_context)` → obvious match
- "Encrypted Data Keys MUST be serialized" + `write_edks(w, &body.encrypted_data_keys)` → obvious match
- "Content Type MUST be serialized" + `write_content_type(w, body.content_type)` → obvious match
- "Reserved MUST be serialized" + `write_bytes(w, &RESERVED_BYTES)` → obvious match
- "IV Length MUST be serialized" + `write_u8(w, get_iv_length(&body.algorithm_suite))` → obvious match
- "Frame Length MUST be serialized" + `write_u32(w, body.frame_length)` → obvious match

All pass the context reset test.

### 4. Semantic relationship check
All executable lines semantically relate to their requirements. No mismatches.

### 5. Sub-items
Each field's "MUST be serialized" and value constraint are annotated at the corresponding write call. This follows Pattern 4 correctly.

### 6. Code structure mirrors spec
The code writes fields in the exact order specified: Version, Type, Algorithm Suite ID, Message ID, AAD, EDKs, Content Type, Reserved, IV Length, Frame Length.

### 7. Top-to-bottom readability
The file reads linearly. Each annotation is immediately before its fulfilling code line.

## Anti-Rationalization Check
No "but it's acceptable because..." patterns found in my reasoning. All findings are clean passes.

## Quote Verification
All 21 v1-header quotes and 2 header-body-version-1-0 quotes verified character-for-character against TOML files.

## Cross-Reference Check
Annotation quotes contain markdown links to data-format specs. The corresponding data-format annotations exist at the same code locations where applicable (Version, Message ID, IV Length). No missing cross-references for the annotations that have them.

## Test Validation
- 12/12 tests pass
- Tests use raw AES keyring (no KMS dependency)
- Tests verify observable output (byte offsets, round-trip correctness, uniqueness)
- All 22 requirements have type=test annotations
