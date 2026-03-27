# Agent 1 Notes — v1_header_body

## Spec-Aligned Structure Analysis (Step 6.8)

### 1. Spec Section Logical Flow
The `encrypt.md#v1-header` spec describes the serialization of a V1 header body as an ordered sequence of field writes:
1. Version (value 0x01)
2. Type (value 0x80 = Customer Authenticated Encrypted Data)
3. Algorithm Suite ID (2 bytes matching the suite used)
4. Message ID (16 random bytes)
5. AAD (encryption context, excluding required EC keys)
6. Encrypted Data Keys (from encryption materials)
7. Content Type (value 0x02 = framed)
8. Reserved (4 zero bytes)
9. IV Length (matches algorithm suite IV length)
10. Frame Length (frame size as UInt32)

The `data-format/message-header.md#header-body-version-1-0` spec mirrors this with a normative serialization order requirement.

### 2. Where Each Requirement Is Fulfilled in Code
- Top-level "MUST be serialized" → the function signature/entry of `write_v1_header_body`
- Serialization order → structural: the sequential `write_*` calls enforce order
- Version → `write_msg_format_version(w, MessageFormatVersion::V1)?;`
- Type → `write_msg_type(w, body.message_type)?;`
- Algorithm Suite ID → `write_esdk_suite_id(w, &body.algorithm_suite)?;`
- Message ID → `write_message_id(w, &body.message_id)?;`
- AAD → `write_aad_section(w, &body.encryption_context)?;`
- EDKs → `write_edks(w, &body.encrypted_data_keys)?;`
- Content Type → `write_content_type(w, body.content_type)?;`
- Reserved → `write_bytes(w, &RESERVED_BYTES)?;`
- IV Length → `write_u8(w, get_iv_length(&body.algorithm_suite))?;`
- Frame Length → `write_u32(w, body.frame_length)`

### 3. Sub-items Under Normative Requirements
Each field in the spec has two sub-requirements:
- "MUST be serialized according to the [Field] specification" (format compliance)
- A value constraint (e.g., "The value MUST correspond to [1.0]")

These are separate `[[spec]]` entries in the TOML and should be annotated as separate annotation blocks (or combined in one block before the write call, matching the v2 pattern).

### 4. Most Likely Structural Mistake
- Putting all annotations at the function top instead of distributing them before each write call
- Forgetting the `type=implication` + `reason=` for the serialization order annotations
- Confusing the `specification/` symlink path with the `aws-encryption-sdk-specification/` path (the codebase uses `specification/` in annotations)

## Potential Spec Gaps

### Partial Quote for message-id
- **Code location**: `v1_header_body.rs` line 28-30
- **Behavior**: The existing annotation quotes only the second half of the TOML requirement for `data-format/message-header.md#message-id`. The TOML quote starts with "While implementations cannot guarantee complete uniqueness," but the annotation starts at "implementations MUST use a good source of randomness..."
- **Why it matters**: Correctness — duvet may not match this partial quote against the TOML entry. However, this is a pre-existing issue and not part of the current work item.
- **Suggested action**: Verify duvet accepts this partial quote. If not, fix the annotation to include the full TOML quote.

### V1 Type Field Value Not Validated on Read
- **Code location**: `v1_header_body.rs` `read_v1_header_body` function
- **Behavior**: The `read_msg_type` function is called but there's no visible validation that the type value is specifically `0x80` (Customer Authenticated Encrypted Data). The spec says "The value MUST correspond to [Customer Authenticated Encrypted Data]" for V1 headers.
- **Why it matters**: Correctness — a V1 header with an invalid type byte might be accepted.
- **Suggested spec requirement**: "When deserializing a V1 header body, the Type field MUST be validated to be Customer Authenticated Encrypted Data (0x80)."

### Content Type Value Not Validated as 0x02 on Read
- **Code location**: `v1_header_body.rs` `read_v1_header_body` function
- **Behavior**: `read_content_type` is called but there's no visible check that the value is specifically `0x02` (framed). The encrypt spec says "The value MUST be [02]".
- **Why it matters**: Interop — accepting non-framed content types in V1 headers could cause downstream issues.
- **Suggested spec requirement**: N/A — this is an encrypt-side constraint. The decrypt path may legitimately accept other content types for backward compatibility.

## Discovery Process Notes
- Shell access was unavailable, so `make validate-all-tests`, `make duvet_extract`, and `make duvet_report` could not be run.
- Gap analysis was performed by manual comparison of TOML requirements against source file annotations.
- The v2_header_body.rs file was used as the authoritative pattern reference — it has complete `encrypt.md#v2-header` annotations and the v1 file should mirror its structure exactly.
- All 21 `encrypt.md#v1-header` test annotations already exist in `tests/test_v1_header_body.rs`.
