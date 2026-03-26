# Agent 1 Notes — v1_header_body encrypt.md#v1-header Annotations

## Spec-Aligned Structure Analysis (Step 6.8)

### 1. Spec Section Logical Flow
The `encrypt.md#v1-header` section describes a single serialization operation with 10 field-level sub-requirements:
1. Parent: "MUST be serialized with the following specifics"
2. Version field (serialize + value = 1.0)
3. Type field (serialize + value = Customer AED)
4. Algorithm Suite ID field (serialize + value = suite used)
5. Message ID field (serialize + randomness)
6. AAD field (serialize + value = EC minus required EC keys)
7. Encrypted Data Keys field (serialize + value = EDKs from materials)
8. Content Type field (serialize + value = 02)
9. Reserved field (serialize per spec)
10. IV Length field (serialize + value = suite IV length)
11. Frame Length field (serialize + value = frame size)
12. Serialization order constraint

### 2. Where Each Requirement Is Fulfilled
Each requirement maps to a specific `write_*` function call in `write_v1_header_body`:
- Parent → function body entry point
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
- Serialization order → the sequential ordering of all the above calls

### 3. Sub-items Under Normative Requirements
Yes — each field has two sub-requirements:
1. "MUST be serialized according to the [X] specification" (format compliance)
2. A value constraint ("value MUST correspond to...", "value MUST be...", etc.)

These should be annotated as stacked annotation blocks before each `write_*` call, exactly as done in `v2_header_body.rs`.

### 4. Most Likely Structural Mistake
The implementer may be tempted to:
- Put all annotations at the top of the function instead of distributing them to each `write_*` call
- Forget to add the data-format serialization order annotation (`header-body-version-1-0` section)
- Confuse the `specification/` prefix (symlink) with `aws-encryption-sdk-specification/` (used in TOML/tests)
- Stack the existing `data-format/message-header.md` annotations incorrectly with the new `encrypt.md#v1-header` annotations

## Potential Spec Gaps

### Content Type Hardcoded to Framed
- **Code location**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/encrypt.rs` line ~506, `content_type: ContentType::Framed`
- **Behavior**: The V1HeaderBody is always constructed with `ContentType::Framed`, never `NonFramed`
- **Why it matters**: The spec says "The value MUST be [02]" (which is Framed), but the data-format spec defines both `01` (NonFramed) and `02` (Framed) as supported content types. The encrypt spec correctly constrains this to `02` only, but the code enforces this at construction time rather than at serialization time. This is correct behavior but worth noting.
- **Suggested spec requirement**: N/A — the spec already covers this with "The value MUST be [02]"

### Reserved Bytes Validation on Read
- **Code location**: `v1_header_body.rs` `read_v1_reserved_bytes` function
- **Behavior**: The read path validates that reserved bytes are exactly `00 00 00 00` and returns an error otherwise
- **Why it matters (interop)**: This is a correctness check for deserialization that ensures forward compatibility. The spec says reserved bytes "MUST have the value (hex) of `00 00 00 00`" but doesn't explicitly say implementations MUST reject non-zero reserved bytes on read.
- **Suggested spec requirement**: "Implementations MUST reject messages where the Reserved field does not have the value `00 00 00 00`."

### IV Length Validation on Read
- **Code location**: `v1_header_body.rs` `read_v1_header_iv_length` function
- **Behavior**: Validates that the deserialized IV length matches the algorithm suite's expected IV length
- **Why it matters (correctness)**: Prevents using a mismatched IV length during decryption, which would cause silent corruption
- **Suggested spec requirement**: "On deserialization, implementations MUST verify that the IV Length field matches the IV length of the algorithm suite and reject the message if they differ."

## Self-Verification Checklist
1. ✅ TOML content was actually read from `compliance/aws-encryption-sdk-specification/client-apis/encrypt/v1-header.toml`
2. ✅ Source file path verified: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs`
3. ✅ Pattern file verified: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs`
4. ⚠️ Could not run `cargo test -- --list` or `make duvet` due to shell command restrictions — test names inferred from reading test file contents directly
5. ✅ TOML for `header-body-version-1-0` was actually read and verified
