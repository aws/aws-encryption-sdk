# Review History — encrypt-body

## Round 1

## Review: CHANGES REQUESTED

### Summary
Annotation quotes are correct and all 6 requirements are addressed with corresponding test annotations. However, there are 3 hard stacking violations (4 annotation blocks before a single code line) and blank-line placement violations in the IV section. These must be restructured before approval.

### Critical Issues (Must Fix)

1. **ANNOTATION_PLACEMENT (stacking)**: 4 annotation blocks before `write_u32(frame_buf, input.sequence_number)?;`
   - **File**: `src/message/body.rs`
   - **Line/Area**: Lines ~711-726 (the sequence number serialization)
   - **Problem**: 4 annotation blocks (Regular SeqNum, Final SeqNum, Regular SeqNum data-format, Final SeqNum data-format) are stacked before a single `write_u32` call. Hard limit is 2.
   - **Fix**: Split the `write_u32` call into two conceptual steps or use intermediate variables so each annotation has its own fulfillment point. For example:
     ```rust
     //= specification/client-apis/encrypt.md#construct-a-frame
     //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be serialized according to the
     //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
     //# The value MUST be the sequence number of this frame.
     //= specification/data-format/message-body.md#regular-frame-sequence-number
     //# The sequence number MUST be interpreted as a UInt32.
     write_u32(frame_buf, input.sequence_number)?;
     //= specification/client-apis/encrypt.md#construct-a-frame
     //= reason=write_u32 above serializes the sequence number for both regular and final frames in this shared code path
     //# - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be serialized according to the
     //# [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
     //= specification/data-format/message-body.md#final-frame-sequence-number
     //= type=implication
     //= reason=write_u32 serializes the sequence number as a UInt32, same type as the regular frame sequence number
     //# The Final Frame Sequence Number MUST be interpreted as the same type as the
     //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
     let _seq_num_written = &input.sequence_number;
     ```
     This keeps the regular frame annotations on the actual `write_u32` call (2 blocks — within limit) and puts the final frame annotations on a separate anchor line (2 blocks — within limit). The `reason=` lines explain the shared code path.

2. **ANNOTATION_PLACEMENT (stacking)**: 4 annotation blocks before `let _encrypted_content_written = ();`
   - **File**: `src/message/body.rs`
   - **Line/Area**: Lines ~785-796 (encrypted content area)
   - **Problem**: 4 annotation blocks (Regular EncContent, Final EncContent, "value MUST be encrypted content", Final EncContent data-format implication) stacked before one dummy line.
   - **Fix**: Same approach — split into two anchor points. Keep the regular frame annotations + "value MUST be" on `let _encrypted_content_written = ();`, and put the final frame annotations on a separate anchor:
     ```rust
     //= specification/client-apis/encrypt.md#construct-a-frame
     //# - [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content): MUST be serialized according to the
     //# [Regular Frame Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content) specification.
     //= specification/client-apis/encrypt.md#construct-a-frame
     //# The value MUST be the encrypted content calculated for this frame.
     let _encrypted_content_written = ();
     //= specification/client-apis/encrypt.md#construct-a-frame
     //= reason=aes_encrypt writes encrypted content to frame_buf for both regular and final frames in this shared code path
     //# - [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content): MUST be serialized according to the
     //# [Final Frame Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content) specification.
     //= specification/data-format/message-body.md#final-frame-encrypted-content
     //= type=implication
     //= reason=aes_encrypt output bytes are written directly to frame_buf, interpreted as raw bytes
     //# The encrypted content MUST be interpreted as bytes.
     let _final_encrypted_content_written = ();
     ```

3. **ANNOTATION_PLACEMENT (stacking)**: 4 annotation blocks before `let _authentication_tag_written = ();`
   - **File**: `src/message/body.rs`
   - **Line/Area**: Lines ~798-811 (authentication tag area)
   - **Problem**: Same pattern as issue #2 — 4 annotation blocks stacked.
   - **Fix**: Same approach — split into two anchor points:
     ```rust
     //= specification/client-apis/encrypt.md#construct-a-frame
     //# - [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag): MUST be serialized according to the
     //# [Regular Frame Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag) specification.
     //= specification/client-apis/encrypt.md#construct-a-frame
     //# The value MUST be the authentication tag output when calculating the encrypted content for this frame.
     let _authentication_tag_written = ();
     //= specification/client-apis/encrypt.md#construct-a-frame
     //= reason=aes_encrypt writes the authentication tag to frame_buf for both regular and final frames in this shared code path
     //# - [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag): MUST be serialized according to the
     //# [Final Frame Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag) specification.
     //= specification/data-format/message-body.md#final-frame-authentication-tag
     //= type=implication
     //= reason=aes_encrypt output tag bytes are written directly to frame_buf, interpreted as raw bytes
     //# The authentication tag MUST be interpreted as bytes.
     let _final_authentication_tag_written = ();
     ```

4. **ANNOTATION_PLACEMENT (blank lines)**: IV annotations have blank lines before executable code
   - **File**: `src/message/body.rs`
   - **Line/Area**: Lines ~729-742 (IV serialization)
   - **Problem**: The Regular Frame IV annotation and Final Frame IV annotation each have a blank line after them before the next annotation block or `write_bytes`. Rule 4 requires no blank lines between annotation and annotated code.
   - **Fix**: Apply the same split pattern as above. Keep the regular frame IV annotations (2 blocks) immediately before `write_bytes(frame_buf, iv)?;` with no blank lines. Put the final frame IV annotations on a separate anchor after `write_bytes`:
     ```rust
     //= specification/client-apis/encrypt.md#construct-a-frame
     //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be serialized according to the
     //# [Regular Frame IV](../data-format/message-body.md#regular-frame-iv) specification.
     //= specification/client-apis/encrypt.md#construct-a-frame
     //# The value MUST be the IV used when calculating the encrypted content for this frame.
     write_bytes(frame_buf, iv)?;
     //= specification/client-apis/encrypt.md#construct-a-frame
     //= reason=write_bytes serializes the IV for both regular and final frames in this shared code path
     //# - [IV](../data-format/message-body.md#final-frame-iv): MUST be serialized according to the
     //# [Final Frame IV](../data-format/message-body.md#final-frame-iv) specification.
     //= specification/data-format/message-body.md#final-frame-iv
     //= type=implication
     //= reason=iv is &[u8], interpreted as raw bytes
     //# The IV MUST be interpreted as bytes.
     let _iv_written = &iv;
     ```

### Suggestions (Optional Improvements)

1. **Clippy warning**: The test file has a `collapsible_if` warning in `test_construct_frame_final_frame_content_length_less_than_frame_length`. Collapse the nested `if` into a single condition. Non-blocking.

### What Passed Review
- ✅ All 6 annotation quotes match TOML files exactly
- ✅ Annotation targets are correct
- ✅ `type=implication` with `reason=` used appropriately for preamble requirements (Req 1, 2) and data-format cross-references
- ✅ All 6 requirements have corresponding `type=test` annotations in the test file
- ✅ Tests pass (22/22)
- ✅ Duvet report generates successfully
- ✅ Cross-reference annotations added for data-format specs
- ✅ Commit message follows Conventional Commits format

### Pattern Summary
The fix for all 4 issues is the same: for each shared serialization line, keep the regular frame annotations (max 2 blocks) immediately before the actual code, then add a separate anchor line after it for the final frame annotations (max 2 blocks). This distributes the annotations so no single code line has more than 2 annotation blocks before it.
