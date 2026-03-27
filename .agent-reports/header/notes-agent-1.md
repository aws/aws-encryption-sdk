# Agent 1 Notes — header.rs

## Spec-Aligned Structure Analysis

### 1. What is the spec section's logical flow?

The `data-format/message-header.md` spec describes the message header as:
1. Structure: Header Body + Header Authentication (in order)
2. Header Body: Version-specific field sequences (V1 or V2)
3. Header Authentication: Version-specific auth fields (V1: IV + Tag, V2: Tag only)
4. Individual field definitions: Version, Type, Algorithm Suite ID, Message ID, AAD, EDKs, Content Type, Reserved, IV Length, Frame Length, Algorithm Suite Data

`header.rs` is the orchestration layer — it dispatches to version-specific readers/writers and performs cross-field validations.

### 2. Where will each requirement be fulfilled in code?

- `#structure` → `write_header_body` (dispatch) and `serialize_header` (body + auth)
- `#message-id` randomness → `generate_message_id` (random byte generation)
- `#encrypted-data-key-count` "greater than 0" → `validate_max_encrypted_data_keys` (empty check)
- `#algorithm-suite-data` → `validate_suite_data` (length and byte comparison)
- `#frame-length` "non-framed must be 0" → `read_header_body` (content_type vs frame_length validation)

### 3. Sub-items under normative requirements?

The `#message-id` requirement is a single TOML entry with a multi-line quote starting with "While implementations cannot guarantee complete uniqueness,". The current annotation in `header.rs` omits the first line.

### 4. Most likely structural mistake?

The `#frame-length` validation in `read_header_body` also checks the inverse (framed with zero frame_length), which has no spec requirement. An implementer might be tempted to annotate both checks, but only the non-framed check has a spec requirement.

## Potential Spec Gaps

### Framed content with zero frame_length

- **Code location**: `header.rs` lines 79-82 (`ContentType::Framed` branch)
- **Behavior**: Rejects framed content with frame_length == 0
- **Why it matters**: Correctness — a zero frame length with framed content would cause division-by-zero or infinite loops during frame processing
- **Suggested spec requirement**: "When the content type is framed, the value of the frame length field MUST be greater than 0."

### EDK count validation only when max_edks is set

- **Code location**: `header.rs` `validate_max_encrypted_data_keys` function
- **Behavior**: The `edks.is_empty()` check only runs when `max_encrypted_data_keys` is `Some`
- **Why it matters**: Correctness — the spec says "This value MUST be greater than 0" unconditionally, but the code only enforces this when max_edks is configured. During deserialization, the EDK count is read as u16 and if it's 0, `read_edks` would return an empty Vec without error.
- **Note**: The `read_edks` function in `encrypted_data_keys.rs` reads the count as u16 and loops `count` times — if count is 0, it returns an empty Vec. The "greater than 0" check in `validate_max_encrypted_data_keys` only runs when max_edks is set.

## Prefix Mismatch Issue (Advisory)

34 source annotations and 34 test annotations across `header_types.rs`, `header_auth.rs`, `shared_header_functions.rs`, `encrypted_data_keys.rs`, and `serializable_types.rs` use the `aws-encryption-sdk-specification/` prefix instead of the `specification/` prefix expected by the local duvet config. This causes the local duvet report to show many false gaps. This is a separate work item that affects multiple files.
