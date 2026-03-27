# Agent 3 Notes — body-format Cycle 2, Review Round 2

## Focus: Verify Round 1 Fixes

This is Round 2. Focusing on the 3 blocking issues and 2 non-blocking suggestions from Round 1.

## Adversarial Pre-Review

### Fix 1: Exception annotations at function entry (was 6-stack)

**Before**: 5 exception blocks + 1 pre-existing frame-length annotation = 6 annotations before `if header.body.frame_length() != 0`.

**After**: Each of the 5 exception blocks is separated by blank lines. The frame-length annotation is separated from the exceptions by a blank line. Only 1 annotation before the `if` statement.

**Verdict**: FIXED ✅. Each exception is a standalone block. The frame-length annotation has exactly 1 annotation before its code line.

### Fix 2: A2+A5+A6 before `let iv` (was 3-stack)

**Before**: A2 (deserialization order) + A5 (deserialize IV length) + A6 (IV as bytes) = 3 annotations before `let iv`.

**After**: A2 is standalone after the frame-length check, separated by blank line. A5 + A6 (2 annotations) before `let iv`.

**Verdict**: FIXED ✅. A2 is standalone (1 annotation, no code line needed — it's an implication about the function's overall structure). A5+A6 = 2 annotations before `let iv`, within limit.

### Fix 3: A7+A10+A11+A12 before `let enc_content` (was 4-stack)

**Before**: A7 (max length) + A10 (Uint64) + A11 (content length matches) + A12 (content as bytes) = 4 annotations before `read_seq_u64_bounded`.

**After**: A7 + A10 (2 annotations) before `let enc_content = read_seq_u64_bounded(...)`. A11 + A12 (2 annotations) after the call, before `let _enc_content_read = &enc_content;`.

**Verdict**: FIXED ✅. Both groups are 2 annotations. The sentinel `let _enc_content_read = &enc_content;` is a reasonable fulfillment point — it references the content that was read, and the annotations describe properties of that content.

### Fix 4 (non-blocking): B2 moved after `iv_seq` (was 3-stack)

**Before**: 2 pre-existing annotations + B2 = 3 annotations before `iv_seq(...)`.

**After**: 2 pre-existing annotations before `iv_seq(...)`. B2 (1 annotation) after `iv_seq`, before `let _iv_is_unique = &iv;`.

**Verdict**: FIXED ✅. The annotation says "MUST include an IV that is unique within the message." The sentinel references the IV that was just computed. The reason line explains the uniqueness derivation. Semantically sound.

### Fix 5 (non-blocking): B5 moved after `write_u32` (was 3-stack)

**Before**: 2 pre-existing annotations + B5 = 3 annotations before `write_u32(w, input.sequence_number)?`.

**After**: 2 annotations before `write_u32(...)`. B5 (1 annotation) after `write_u32`, before `let _seq_num_written = &input.sequence_number;`.

**Verdict**: FIXED ✅. The annotation says "serialized to a message the same way as the Regular Frame Sequence Number." The sentinel references the sequence number that was just written. The reason line explains the shared code path. Semantically sound.

### B3 at `plaintext: &plaintext_frame` (still 3-stack)

This remains a 3-stack: 2 pre-existing annotations + B3. Round 1 review explicitly noted this as non-blocking and said restructuring previously-approved code is out of scope. Agent 2 documented why it's unavoidable. Accepted.

### Duplicate B1 removal

The `types.rs` diff is empty — the duplicate B1 annotation at `FrameLength::new` does not exist in the current working tree. The struct-level annotation at line ~52 provides sufficient coverage. Correct.

## Anti-Rationalization Check

I reviewed all 5 fixes. For each one, I asked: "Did the fix actually resolve the stacking violation, or did it just move the problem?" In all cases, the fix genuinely resolved the violation by either:
- Separating blocks with blank lines (Fix 1)
- Moving annotations to standalone positions (Fix 2)
- Splitting groups and using sentinel lines (Fixes 3, 4, 5)

I did not find any pattern of "this is wrong but acceptable because..." in my reasoning.

## New Issues Check

I scanned the entire diff for any new issues introduced by the fixes:
- `#[allow(clippy::no_effect_underscore_binding)]` on `read_and_decrypt_non_framed_message_body`: Necessary for the `_enc_content_read` sentinel. Acceptable.
- Sentinel lines (`_iv_is_unique`, `_seq_num_written`, `_enc_content_read`): These are no-op references that exist solely as annotation fulfillment points. They follow the existing pattern of `_endframe_written` and `_encrypted_content_written` already in the codebase. Consistent.
- No new stacking violations introduced.
- No quote changes — all quotes remain character-perfect against TOML files.

## Test Results

- Body format tests: 33/33 PASS
- All other tests: PASS (except 8 pre-existing credential failures in test_authentication_tag and 5 in test_encrypt_decrypt — all "security token included in the request is invalid")
- Clippy: 0 warnings in modified files (8 pre-existing in other files)
- Duvet: Report generates successfully

## Conclusion

All 3 blocking issues from Round 1 are resolved. Both non-blocking suggestions were addressed. No new issues introduced. Ready to approve.
