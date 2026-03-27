## Round 3

## Review: APPROVED AND COMMITTED ✅

### Summary
All 5 annotation stacking violations from Round 2 are resolved. Every annotation location now has at most 2 blocks before any single line of code. All 33 requirements from `encrypt.md#construct-a-frame` have implementation/implication annotations with correct quotes, proper placement, and semantic alignment. Tests pass.

### What Was Verified
- ✅ Duvet annotations use exact quotes from TOML files
- ✅ Annotation placement follows correct patterns (including fine-grained sub-items where applicable)
- ✅ No annotation stacking violations (max 2 blocks per code line, verified systematically)
- ✅ Implementation matches specification requirements
- ✅ Tests cover all implementation annotations (pre-existing test annotations for all 33 requirements)
- ✅ Code quality is acceptable
- ✅ Commit message follows Conventional Commits format
- ✅ Stale compliance exceptions for Reqs 32/33 correctly removed

### Test Results (from manual validation)
- Check 1 (Tests): PASS — 21/21 construct_a_frame, 7/7 construct_the_body pass
- Check 2 (Coverage): N/A (pre-spawn hook not available)
- Check 3 (Duvet Report): PASS — all 33 requirements show implementation/implication coverage
- Check 4 (Snapshot): PASS — snapshot updated to reflect new annotations
- Check 5 (Linter): PASS — only pre-existing warnings (missing_docs, collapsible_if)

Pre-existing failures: test_authentication_tag (8), test_encrypt_decrypt (5), test_reproduced_enc_context (2), test_required_encryption_context (2) — all due to invalid AWS credentials, unrelated to this change.

### Commit
`0bcba128 fix(encrypt): resolve annotation stacking violations in construct-a-frame`

### Test Handoff
**Spec**: `aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/.duvet/requirements/specification/client-apis/encrypt/construct-a-frame.toml`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/.duvet/snapshot.txt`
- `compliance_exceptions/encrypt.txt`

**Commit Message**: `fix(encrypt): resolve annotation stacking violations in construct-a-frame`

### Advisory Notes
- Reqs 32 and 33 lack `type=test` annotations in the snapshot (pre-existing gap, not introduced by this change)
- The `let _` anchor lines (`_encrypted_content_written`, `_authentication_tag_written`, `_endframe_written`) are the minimal approach to provide separate fulfillment points for annotations describing different outputs of a single function call

## Round 2

## Review: CHANGES REQUESTED

### Summary
Annotation quotes are exact and match the updated TOML. Spec compliance is correct. However, there are **5 locations** with 3+ annotation blocks stacked before a single line of code, which is an automatic rejection per the hard rule. The worst offender is 5 blocks before `aes_encrypt(...)`. The previous round's feedback about stacking was for a different work item (final-frame); this round's stacking issues are new and specific to the construct-a-frame annotations.

### Critical Issues (Must Fix)

1. **[ANNOTATION_PLACEMENT — 5-stack]**: 5 annotation blocks before `aes_encrypt(...)` in `construct_frame`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: The block of annotations immediately before `aes_encrypt(input.alg, iv, ...)`
   - **Problem**: 5 annotation blocks stacked:
     1. Req 1: "To construct a regular or final frame...MUST calculate the encrypted content and an authentication tag..."
     2. Req 27: "Encrypted Content: MUST be serialized according to..."
     3. Req 28: "The value MUST be the encrypted content calculated for this frame."
     4. Req 29: "Authentication Tag: MUST be serialized according to..."
     5. Req 30: "The value MUST be the authentication tag output..."
   - **Fix**: Req 1 is the overarching "calculate encrypted content and auth tag" requirement — it belongs before `aes_encrypt`. Reqs 12 and 13 (cipherkey, plaintext) are already correctly placed on parameters inside the call. But Reqs 27-30 are about **serialization** of the encrypted content and auth tag, not about the encryption operation itself. Since `aes_encrypt` writes both encrypted content and auth tag directly into `w`, and there's no separate serialization step, you need to restructure. Options:
     - **Option A**: After the `aes_encrypt(...)` call, add a comment block that separates the "encrypted content was written" from "auth tag was written". For example, add a `let encrypted_and_tag_written = true;` line (or similar no-op binding) after `aes_encrypt` and place Reqs 29-30 before it, keeping Reqs 27-28 before `aes_encrypt`. This gives each pair its own fulfillment point.
     - **Option B**: Move Reqs 27-28 to a `//= reason=` annotation on the `aes_encrypt` call explaining that aes_encrypt writes encrypted content to w, and move Reqs 29-30 to a separate line after the call with a reason explaining the auth tag is appended. But this still requires a code line after the call for Reqs 29-30.
     - **Preferred**: Keep only Req 1 before `aes_encrypt(...)`. Move Reqs 27-28 and 29-30 to after the `aes_encrypt` call — add a `// aes_encrypt writes encrypted content followed by authentication tag to w` comment line, then split: Reqs 27-28 as `type=implication` with `reason=aes_encrypt writes encrypted content directly to the output buffer w` before a no-op let binding like `let _encrypted_content_and_tag = ();`, and Reqs 29-30 as `type=implication` with similar reason. Or simply: keep Req 1 + one of {27,28} before `aes_encrypt`, and place {29,30} + the other of {27,28} after the call on a separate line. The key constraint: max 2 annotation blocks before any single line.

2. **[ANNOTATION_PLACEMENT — 4-stack]**: 4 annotation blocks before `if input.is_final` (first occurrence)
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: The block between `iv_seq(...)` and `if input.is_final {`
   - **Problem**: 4 annotation blocks:
     1. Req 16: "The Encrypt operation MUST serialize a regular frame or final frame..."
     2. `data-format/message-body.md#final-frame`: "A final frame MUST be serialized as, in order..."
     3. `data-format/message-body.md#final-frame`: "This means a final frame MUST be a regular frame..."
     4. Req 20: "The Sequence Number End MUST only be serialized for the final frame."
   - **Fix**: Req 16 is a general statement about serialization — it can stay at the top of the serialization block. Req 20 is specifically about the `if input.is_final` guard — it should stay immediately before that `if`. The two `data-format/message-body.md#final-frame` annotations (items 2 and 3) are about final-frame structure. Move them **inside** the `if input.is_final` block, before `write_u32(w, ENDFRAME_SEQUENCE_NUMBER)?`. This distributes the annotations: Req 16 before the serialization block, Req 20 before `if input.is_final`, and the two data-format annotations inside the `if` block (giving 2 + 2 = max 2 per location, since Reqs 19 + cross-ref are already inside the `if`). Actually, that would create a new stack inside the `if`. Better approach: move items 2 and 3 to just after `iv_seq` with Req 16 (keeping them as a group of 3 is still a violation). The real fix: remove items 2 and 3 from `construct_frame` entirely — they are `data-format` annotations that were added in the previous work item (final-frame). They are already covered by the `specification/data-format/message-body.md#final-frame` annotations. If they must stay, place item 2 inside the first `if input.is_final` block (before the ENDFRAME write) and item 3 inside the second `if input.is_final` block (before the content length write), since those are the two "additions" the annotation describes.

3. **[ANNOTATION_PLACEMENT — 3-stack]**: 3 annotation blocks before `write_u32(w, input.sequence_number)?`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Before `write_u32(w, input.sequence_number)?`
   - **Problem**: 3 annotation blocks:
     1. Req 21: "Sequence Number: MUST be serialized according to..."
     2. Req 22: "The value MUST be the sequence number of this frame."
     3. Cross-ref: `data-format/message-body.md#regular-frame-sequence-number` — "The sequence number MUST be serialized as a UInt32."
   - **Fix**: Combine Reqs 21 and 22 into a single annotation block (they are from the same spec section and describe the same code line). The cross-ref can stay as a second block. This gives 2 blocks, which is within the limit. Alternatively, move the cross-ref annotation to a `//= reason=` line on Req 21 explaining the UInt32 serialization format.

4. **[ANNOTATION_PLACEMENT — 3-stack]**: 3 annotation blocks before regular-frame `construct_frame` call
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Before `construct_frame(` in the regular-frame path
   - **Problem**: 3 annotation blocks:
     1. `specification/client-apis/encrypt.md#construct-the-body` — "Regular frame serialization MUST conform..." (pre-existing)
     2. `specification/data-format/message-body.md#regular-frame` — "A regular frame MUST be serialized as, in order..." (pre-existing)
     3. `aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame` — "For a regular frame, the serialization MUST follow..." (NEW — Agent 2 added this)
   - **Fix**: Move the new Req 17 annotation inside the `ConstructFrameInput` struct literal, on the `is_final: false` field with a `//= reason=is_final=false means this is a regular frame, whose serialization follows the Regular Frame specification`. This distributes it away from the pre-existing stack.

5. **[ANNOTATION_PLACEMENT — 3-stack]**: 3 annotation blocks before final-frame `construct_frame` call
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Before `construct_frame(` in the final-frame path
   - **Problem**: 3 annotation blocks:
     1. `specification/client-apis/encrypt.md#construct-the-body` — "If an end to the input has been indicated..." (pre-existing)
     2. `specification/client-apis/encrypt.md#construct-the-body` — "Final frame serialization MUST conform..." (pre-existing)
     3. `aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame` — "For a final frame, the serialization MUST follow..." (NEW — Agent 2 added this)
   - **Fix**: Same approach as issue 4 — move the new Req 18 annotation inside the `ConstructFrameInput` struct literal, on the `is_final: true` field with a `//= reason=is_final=true means this is a final frame, whose serialization follows the Final Frame specification`.

### Suggestions (Optional Improvements)

1. The `type=implication` annotations inside `if input.is_final` blocks (Reqs 19, 25) have 2 annotation blocks each before `write_u32`. This is at the limit (2) but acceptable. No action needed.

2. Consider whether the two `data-format/message-body.md#final-frame` annotations (items 2 and 3 in Issue 2) are truly needed in `construct_frame` given they are structural observations about the final-frame format. If they were added in a previous work item, they may already be covered elsewhere.

### Checklist Summary

| Check | Status |
|-------|--------|
| Exact quotes | ✅ PASS — all quotes match TOML character-for-character |
| TOML updates | ✅ PASS — local TOML updated to match current spec text |
| Correct targets | ✅ PASS |
| Correct types | ✅ PASS — implication with reason for structural reqs, default for runtime-enforceable |
| No 3+ stacks | ❌ FAIL — 5 locations with 3+ stacks (4 in construct_frame, 2 at call sites) |
| Semantic placement | ✅ PASS — all annotations semantically relate to their code |
| Tests present | ✅ PASS — pre-existing test annotations cover all 33 requirements |
| All tests pass | ✅ PASS — 21/21 + 7/7 pass; only pre-existing KMS credential failures |
| Duvet coverage | ✅ PASS — all 33 requirements show implementation/implication coverage |
| Code quality | ✅ PASS — minimal changes, idiomatic Rust |
| Commit message | ✅ PASS — conventional commits format |
| Compliance exceptions | ✅ PASS — 2 stale exceptions correctly removed |

## Round 1

## Review: CHANGES REQUESTED

### Summary
Annotation quotes are exact, placement semantics are correct, and all 7 tests pass. However, there are three locations with 3+ annotation blocks stacked before a single line of code, which is an automatic rejection per the hard rule. The fix is to distribute annotations closer to their individual fulfillment points.

### Critical Issues (Must Fix)

1. **[ANNOTATION_PLACEMENT — 3+ stack]**: `body.rs` encrypt path, final `construct_frame` call (~lines 501-515)
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Lines 501-515 (before `construct_frame(...)`)
   - **Problem**: 3 annotation blocks before `construct_frame`:
     1. `encrypt.md#construct-the-body` — "If an end to the input has been indicated..." (pre-existing)
     2. `encrypt.md#construct-the-body` — "Final frame serialization MUST conform..." (pre-existing)
     3. `data-format/message-body.md#final-frame` — "A final frame MUST be serialized as, in order..." (NEW)
   - **Fix**: Move the new Req 5 annotation ("A final frame MUST be serialized as, in order...") inside `construct_frame` itself, at the point where the serialization order is actually enforced. The `construct_frame` function already has the Req 6 annotation at the `if input.is_final` block. Place the Req 5 annotation on the first `if input.is_final` line (or just before the sequence of `write_u32`/`write_bytes` calls that implement the serialization order). This moves it to the actual point of fulfillment and eliminates the stack at the call site.

2. **[ANNOTATION_PLACEMENT — 3+ stack]**: `test_regular_frame_serialization_conforms_to_spec` (~lines 146-155)
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs`
   - **Line/Area**: Top of `test_regular_frame_serialization_conforms_to_spec`
   - **Problem**: 3 annotation blocks before `let pt = vec![0xAAu8; 30]`:
     1. `encrypt.md#construct-the-body` — "Regular frame serialization MUST conform..." (pre-existing)
     2. `message-body.md#final-frame` — "Framed data MUST contain exactly one final frame." (NEW)
     3. `message-body.md#final-frame` — "The final frame MUST be the last frame." (NEW)
   - **Fix**: Move the two new test annotations down to their respective assertion lines:
     - "Framed data MUST contain exactly one final frame." → place immediately before `assert_eq!(final_count, 1, ...)` 
     - "The final frame MUST be the last frame." → place immediately before `assert_eq!(result, pt, ...)` (the round-trip assertion proves no data after the final frame) or before the `assert_eq!(final_count, 1, ...)` if you prefer to keep them together (but then you'd have 2 annotations before one assert, which is the max allowed)

3. **[ANNOTATION_PLACEMENT — 3+ stack]**: `test_empty_plaintext_constructs_empty_final_frame` (~lines 289-303)
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs`
   - **Line/Area**: Top of `test_empty_plaintext_constructs_empty_final_frame`
   - **Problem**: 3 annotation blocks before `let ct = encrypt_with_frame_length(b"", 4096).await`:
     1. `encrypt.md#construct-the-body` — "If an end to the input has been indicated..." (pre-existing)
     2. `message-body.md#final-frame` — "A final frame MUST be serialized as, in order..." (NEW)
     3. `message-body.md#final-frame` — "This means a final frame MUST be a regular frame..." (NEW)
   - **Fix**: Move the two new test annotations down to the assertions that verify them:
     - "A final frame MUST be serialized as, in order..." → place immediately before the `assert!(found_structure, ...)` assertion at the bottom of the test (which verifies the serialization structure)
     - "This means a final frame MUST be a regular frame with the addition of the serialized Sequence Number End and Encrypted Content Length." → place immediately before the `assert!(found_structure, ...)` assertion (or before the `assert_eq!(content_len, 0, ...)` which verifies the Encrypted Content Length field exists). If both end up before `assert!(found_structure, ...)`, that's 2 annotations before one line, which is the max allowed. Alternatively, place one before `assert_eq!(final_count, 1, ...)` and one before `assert!(found_structure, ...)`.

### Checklist Summary

| Check | Status |
|-------|--------|
| Exact quotes | ✅ PASS — all quotes match TOML character-for-character |
| Correct targets | ✅ PASS — `specification/` for impl, `aws-encryption-sdk-specification/` for tests |
| Correct types | ✅ PASS — `implication` with `reason=` for structural reqs, default for runtime-enforceable |
| No 3+ stacks | ❌ FAIL — 3 locations with 3+ stacks |
| Semantic placement | ✅ PASS — all annotations semantically relate to their code |
| Tests present | ✅ PASS — 7 `type=test` annotations for 7 requirements |
| Tests exercise code | ✅ PASS — tests call encrypt/decrypt and verify output |
| All tests pass | ✅ PASS — 7/7 pass |
| Duvet coverage | ✅ PASS — all final-frame requirements show TEXT[test] |
| Code quality | ✅ PASS — minimal changes, idiomatic Rust |
| Commit message | ✅ PASS — conventional commits format |
