# Discovery Notes — body-format (Cycle 3)

## Spec-Aligned Structure Analysis

### Spec Section Logical Flow

**Non-Framed Data**: Serialize/deserialize order → IV → Content Length → Content → Auth Tag.
All non-framed write-path requirements are `type=exception` (ESDK only encrypts framed).
All non-framed read-path requirements are `type=implication`.
Non-framed data is fully annotated with correct `specification/` prefix.

**Framed Data**: Two constraints (max frame size, max frame count) → Regular Frame → Final Frame.

**Regular Frame**: Serialization order → Sequence Number → IV → Encrypted Content → Auth Tag.
Each sub-field has its own section with serialization/deserialization requirements.

**Final Frame**: Exactly one, must be last → plaintext length constraints → serialization order →
Sequence Number End → Sequence Number → IV → Content Length → Content → Auth Tag.

### Root Cause of Remaining Gaps

The primary issue is a **spec path prefix mismatch**:
- The local duvet config (`esdk/.duvet/config.toml`) uses `specification/` as the spec source prefix
- 8 implementation annotations in `body.rs` use `aws-encryption-sdk-specification/` prefix (wrong)
- ALL 40 test annotations in `test_message_body_format.rs` and `test_construct_the_body.rs` use `aws-encryption-sdk-specification/` prefix (wrong)

This means the local duvet report sees implementation/implication annotations (with correct prefix)
but does NOT see test annotations (wrong prefix) for any `specification/` prefix requirements.

Additionally, 2 requirements have NO annotation at all (not even wrong-prefix):
1. `#final-frame-iv`: "The IV MUST be a unique IV within the message."
2. `#final-frame-encrypted-content`: "The encrypted content MUST be interpreted as bytes."

### Where Each Requirement Is Fulfilled in Code

| Requirement | Code Construct |
|---|---|
| Regular frame seq num serialized as UInt32 | `write_u32(w, input.sequence_number)` in `construct_frame` |
| Regular frame IV unique | `iv_seq(input.sequence_number, iv)` in `construct_frame` |
| Regular frame encrypted content = frame length | `plaintext: &plaintext_frame` (exactly frame_length bytes) |
| Final frame seq num = total frames | `sequence_number` param in final `construct_frame` call |
| Final frame seq num serialized same as regular | Same `write_u32` call in `construct_frame` |
| Final frame seq num interpreted same as regular | `read_u32(r, raw)` in `read_and_decrypt_framed_message_body` |
| Final frame IV unique | `iv_seq(input.sequence_number, iv)` in `construct_frame` (same as regular) |
| Final frame encrypted content length as UInt32 | `write_u32(w, input.plaintext.len() as u32)` in `construct_frame` |
| Final frame encrypted content as bytes | `read_seq_u32_bounded` returns `Vec<u8>` in decrypt path |

### Most Likely Structural Mistake

The implementer might be tempted to add new annotations instead of fixing the prefix.
The correct fix is to change `aws-encryption-sdk-specification/` to `specification/` in existing annotations.
No new annotation logic is needed for the 7 wrong-prefix cases — only the path prefix changes.

For the 2 genuinely missing annotations, the implementer should add them at the same code locations
where the related requirements are already annotated (construct_frame for IV uniqueness,
read_and_decrypt_framed_message_body for encrypted content bytes).

## Potential Spec Gaps

No significant spec gaps identified. The code behavior aligns well with the spec requirements.
The `iv_seq` function derives IVs from sequence numbers, which guarantees uniqueness within a message
as long as sequence numbers are unique — this is enforced by the incrementing counter.
