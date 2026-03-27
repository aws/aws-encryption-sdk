# Review History — body-format Cycle 2

## Round 1

## Review: CHANGES REQUESTED

### Summary
All 24 annotation quotes are character-perfect against the TOML files. Annotation types (exception/implication) and reason lines are appropriate. However, there are multiple annotation stacking violations in the non-framed data function that Agent 2 created, and three cases where Agent 2 pushed pre-existing 2-stacks to 3-stacks in the framed data path. The non-framed stacks are blocking.

### Critical Issues (Must Fix)

1. **ANNOTATION_PLACEMENT / STACKING**: 6-annotation stack at `read_and_decrypt_non_framed_message_body` entry
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Lines ~289-313 (5 exception blocks + 1 pre-existing annotation before `if header.body.frame_length() != 0`)
   - **Problem**: 5 exception annotation blocks (A1, A3, A4, A8, A9) are stacked before the frame-length check. Exception annotations have NO semantic relationship to the `if` statement — they describe write-path requirements that the ESDK doesn't implement. Stacking them before a read-path validation line is misleading.
   - **Fix**: Move all 5 exception annotations to a dedicated block at the **top of the file** (after the imports, before any function definitions), separated from the read-path code. Exception annotations don't need to be "before" any specific code line — they document intentional non-implementation. Group them together with a single comment like `// Non-framed write-path requirements: ESDK only encrypts framed data`. Alternatively, place them just inside the function signature but BEFORE the pre-existing `frame-length` annotation, with a blank line separating the exception block from the read-path code. The key constraint: the exception block must NOT be stacked with the `frame-length` annotation.

2. **ANNOTATION_PLACEMENT / STACKING**: 3-annotation stack before `let iv = ...`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Lines ~319-335 (A2 + A5 + A6 before `let iv = serialize_functions::read_vec(...)`)
   - **Problem**: A2 (deserialization order) is a general-behavior annotation about the function's overall structure. A5 and A6 are specific to the IV read. Stacking all three before one line violates the hard limit.
   - **Fix**: Move A2 (deserialization order) to immediately after the frame-length check (before the first read operation), as a standalone annotation. It's a Pattern 3 (general behavior) annotation that describes the function's overall flow. Then A5 + A6 (2 annotations) remain before `let iv`, which is within the limit.

3. **ANNOTATION_PLACEMENT / STACKING**: 4-annotation stack before `let enc_content = ...`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Lines ~337-352 (A7 + A10 + A11 + A12 before `let enc_content = serialize_functions::read_seq_u64_bounded(...)`)
   - **Problem**: 4 annotations before a single function call. `read_seq_u64_bounded` is a single call that reads the length (as u64) and then reads that many bytes of content. The annotations describe different aspects: max length (A7), interpretation as Uint64 (A10), content length matching (A11), and content as bytes (A12).
   - **Fix**: Split into two groups by reformatting the call to multi-line with intermediate steps, OR distribute annotations to the call's parameters. Specifically:
     - Keep A7 (max content length) and A10 (Uint64 interpretation) before the `read_seq_u64_bounded` call — these relate to reading the length field.
     - Move A11 (content length matches) and A12 (content as bytes) to AFTER the call, on a `let _content_read = ();` sentinel line or similar. These describe properties of the content that was read, not the act of reading the length.
     This brings both groups to ≤2 annotations.

### Suggestions (Non-Blocking)

4. **ANNOTATION_PLACEMENT / STACKING**: B2, B3, B5 each push pre-existing 2-stacks to 3-stacks
   - **Files**: `body.rs` — before `iv_seq(...)`, before `plaintext: &plaintext_frame`, before `write_u32(w, input.sequence_number)?`
   - **Problem**: Agent 2 added one annotation to each of three pre-existing 2-stacks, creating 3-stacks. The pre-existing stacks were approved in prior cycles.
   - **Suggestion**: If you can find a way to place B2, B3, or B5 at a different code location that still makes semantic sense, do so. If not, leave them — the pre-existing stacks are the root cause and restructuring previously-approved code is out of scope for this work item. Document in your summary why each 3-stack is unavoidable.

5. **CODE_QUALITY**: Duplicate B1 annotation in `types.rs`
   - The same `framed-data` / `total bytes allowed` annotation appears at both the `FrameLength` struct definition (pre-existing, line ~52) and at `FrameLength::new` (Agent 2 added, line ~74). The pre-existing one is sufficient for duvet coverage. The new one at `FrameLength::new` is redundant but not harmful. No action needed unless you want to remove the duplicate.

### Checklist Summary

- ✅ All 24 annotation quotes match TOML files exactly
- ✅ Annotation targets are valid
- ✅ Annotation types are correct (exception for write-path, implication for read-path structural properties)
- ✅ All exception annotations have `reason=` lines
- ✅ All implication annotations have `reason=` lines
- ✅ All 14 non-framed data requirements annotated (Group A)
- ✅ All 6 framed data requirements annotated (Group B)
- ✅ All 4 misquoted annotations fixed (Group C)
- ❌ Annotation stacking violations (3 blocking stacks in non-framed function)
- ✅ Tests pass (33/33 body format tests)
- ✅ Clippy clean (0 warnings in modified files)
- ✅ Duvet report generates successfully

## Round 2

## Review: APPROVED AND COMMITTED ✅

### Summary
All 3 blocking annotation stacking violations from Round 1 are resolved. Both non-blocking suggestions (B2/B5 3-stacks reduced to 2-stacks, duplicate B1 removed) were addressed. No new issues introduced. All annotation quotes remain character-perfect against TOML files.

### What Was Verified
- ✅ Duvet annotations use exact quotes from TOML files
- ✅ Annotation placement follows correct patterns (stacking violations resolved)
- ✅ Implementation matches specification requirements
- ✅ Tests cover all implementation annotations (33/33 body format tests pass)
- ✅ Code quality is acceptable (sentinel lines follow existing codebase patterns)
- ✅ Commit message follows Conventional Commits format

### Test Results (manual validation)
- Check 1 (Tests): PASS — 33/33 body format tests pass; all other tests pass except 13 pre-existing credential failures
- Check 2 (Coverage): N/A — pre-spawn hook logs not available
- Check 3 (Duvet Report): PASS — report generates successfully (1265 annotations, 2256 references)
- Check 4 (Snapshot): N/A — pre-spawn hook logs not available
- Check 5 (Linter): PASS — 0 clippy warnings in modified files (8 pre-existing in other files)

### Commit
`e11d3d63 fix(message-body): resolve annotation stacking violations in body.rs`

### Test Handoff
**Spec**: `aws-encryption-sdk-specification/data-format/message-body.md#non-framed-data`, `#non-framed-data-iv`, `#non-framed-data-encrypted-content-length`, `#non-framed-data-encrypted-content`, `#non-framed-data-authentication-tag`, `#framed-data`, `#regular-frame-iv`, `#regular-frame-encrypted-content`, `#regular-frame-sequence-number`, `#final-frame-sequence-number`, `#final-frame-encrypted-content-length`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`

# Review History — body-format Cycle 3

## Round 1

## Review: CHANGES REQUESTED

### Summary
All 48 prefix fixes are correct — zero remaining `aws-encryption-sdk-specification/data-format/message-body.md` annotations in any of the 3 files. B1 (final-frame-iv uniqueness) is well-placed. However, B2 (final-frame-encrypted-content bytes) creates a 3-annotation stack before `read_bytes(r, &mut auth_tag, raw)?;`, which is a hard-limit violation.

### Critical Issues (Must Fix)

1. **ANNOTATION_PLACEMENT / STACKING**: B2 creates a 3-stack before `read_bytes(r, &mut auth_tag, raw)?;`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Lines ~168-178 (B2 + 2 auth_tag annotations before `read_bytes`)
   - **Problem**: B2 (`final-frame-encrypted-content` / "encrypted content MUST be interpreted as bytes") is placed after `read_seq_u32_bounded()?;` but before the 2 pre-existing `final-frame-authentication-tag` annotations and their `read_bytes` call. This creates 3 annotation blocks before a single executable line. Additionally, B2 describes a property of the encrypted content, but the next executable line reads the auth tag — a semantic mismatch that fails the context reset test.
   - **Fix**: Add a sentinel line immediately after B2 to separate it from the auth_tag annotations. For example:
     ```rust
            //= specification/data-format/message-body.md#final-frame-encrypted-content
            //= type=implication
            //= reason=read_seq_u32_bounded returns Vec<u8>
            //# The encrypted content MUST be interpreted as bytes.
            let _enc_content_is_bytes = &enc_content;
            //= specification/data-format/message-body.md#final-frame-authentication-tag
            ...
     ```
     This gives B2 its own fulfillment point (the sentinel references `enc_content`, matching the annotation's subject) and reduces the auth_tag stack to 2 annotations.

### Checklist Summary

- ✅ All 48 prefix fixes verified (0 remaining wrong-prefix annotations)
- ✅ B1 annotation quote matches TOML exactly
- ✅ B2 annotation quote matches TOML exactly
- ✅ B1 placement is correct (2-stack within limit)
- ❌ B2 placement creates 3-stack (blocking)
- ✅ All annotation types correct (implication with reason lines)
- ✅ `aws-encryption-sdk-specification/client-apis/encrypt.md` annotations intentionally untouched
- ✅ Tests: 33/33 body format + 7/7 construct body PASS
- ✅ Clippy: 0 warnings in modified files
- ✅ Duvet: 0 `!MUST` entries for `specification/data-format/message-body.md`

## Round 2

## Review: APPROVED AND COMMITTED ✅

### Summary
The Round 1 B2 stacking violation is resolved. The sentinel line `let _enc_content_is_bytes = &enc_content;` correctly separates the B2 annotation from the 2 auth_tag annotations, bringing both groups within the 2-annotation limit. All 48 prefix fixes remain correct. No new issues introduced.

### What Was Verified
- ✅ Duvet annotations use exact quotes from TOML files
- ✅ Annotation placement follows correct patterns (B2 stacking resolved with sentinel)
- ✅ Implementation matches specification requirements
- ✅ Tests cover all implementation annotations (33/33 body format + 7/7 construct body)
- ✅ Code quality is acceptable (sentinel follows existing `_iv_is_unique` pattern)
- ✅ Commit message follows Conventional Commits format

### Test Results
- Check 1 (Tests): PASS — 33/33 body format tests, 7/7 construct body tests
- Check 2 (Coverage): N/A — pre-spawn hook logs not available
- Check 3 (Duvet Report): PASS — 1267 annotations, 2258 references, 0 `!MUST` for message-body.md
- Check 4 (Snapshot): N/A — pre-spawn hook logs not available
- Check 5 (Linter): PASS — 0 clippy warnings in modified files

### Commit
`742471d7 fix(message-body): fix 48 annotation prefixes, add 2 annotations, fix B2 stacking`

### Test Handoff
**Spec**: `specification/data-format/message-body.md#regular-frame-sequence-number`, `#regular-frame-iv`, `#regular-frame-encrypted-content`, `#final-frame-sequence-number`, `#final-frame-iv`, `#final-frame-encrypted-content`, `#final-frame-encrypted-content-length`, `#sequence-number-end`, `#final-frame-authentication-tag`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_message_body_format.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs`

**Commit Message**: `fix(message-body): fix 48 annotation prefixes, add 2 annotations, fix B2 stacking`
