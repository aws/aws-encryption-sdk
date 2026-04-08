# Review History — decrypt-body

## Round 1

## Review: CHANGES REQUESTED

### Summary
Annotation quotes are correct and match the TOML. Semantic placement is good. Tests pass and duvet coverage is complete. Two issues require fixes: (1) a 3-annotation stack at the final frame Sequence Number that can be reduced by moving the parent annotation, and (2) out-of-scope encrypt-side annotation changes that must be reverted from this commit.

### Critical Issues (Must Fix)

1. **ANNOTATION_PLACEMENT**: Final frame parent annotation creates a 3-stack at the Sequence Number line
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Lines 175-184 (inside the `if seq_num == ENDFRAME_SEQUENCE_NUMBER` block)
   - **Problem**: Three annotation blocks before `let seq_num: u32 = read_u32(ciphertext, sig_digest)?;`:
     1. `For a final frame, each field MUST be deserialized according to its specification:` (new parent)
     2. `- [Sequence Number](...): MUST be deserialized...` (new sub-item)
     3. `The Final Frame Sequence Number MUST be interpreted...` (pre-existing data-format)
   - **Fix**: Move the parent annotation (`For a final frame, each field MUST be deserialized...`) to BEFORE the `if seq_num == ENDFRAME_SEQUENCE_NUMBER {` line. This is Pattern 3 (general behavior at block start) — the `if` statement is the entry point to the final frame deserialization block. The executable line after the annotation becomes the `if` statement itself. This reduces the stack inside the block from 3 to 2.

     Before:
     ```rust
     if seq_num == ENDFRAME_SEQUENCE_NUMBER {
         //= specification/client-apis/decrypt.md#decrypt-the-message-body
         //# For a final frame, each field MUST be deserialized according to its specification:
         //= specification/client-apis/decrypt.md#decrypt-the-message-body
         ...
     ```
     After:
     ```rust
     //= specification/client-apis/decrypt.md#decrypt-the-message-body
     //# For a final frame, each field MUST be deserialized according to its specification:
     if seq_num == ENDFRAME_SEQUENCE_NUMBER {
         //= specification/client-apis/decrypt.md#decrypt-the-message-body
         ...
     ```

2. **SCOPE_CREEP**: Out-of-scope encrypt-side annotation changes in `construct_frame`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
   - **Line/Area**: Lines 665-668 and 706-709 (in `construct_frame` function)
   - **Problem**: Agent 2 modified two encrypt-side annotations (`encrypt.md#construct-a-frame`) that are not part of the decrypt-body work item. Changed:
     - `"The Sequence Number End MUST only be serialized for the final frame."` → `"- [Sequence Number End](...): MUST be serialized according to the [Sequence Number End](...) specification."`
     - `"The Encrypted Content Length MUST only be serialized for the final frame."` → `"- [Encrypted Content Length](...): MUST be serialized according to the [Final Frame Encrypted Content Length](...) specification."`
   - **Fix**: Revert these two changes. They may be correct fixes, but they belong in a separate work item for `encrypt.md#construct-a-frame`. This commit should only contain decrypt-body changes.

### Non-Blocking Observations (Pre-existing, not caused by Agent 2)

The following 3-annotation stacks exist but are caused by pre-existing annotations that Agent 2's new annotations stack with. These are NOT blocking because fixing them would require restructuring pre-existing code beyond the scope of this FIX_ANNOTATION work item:

- **IV location** (line ~188): New `- [IV](...): MUST be deserialized...` + 2 pre-existing data-format annotations = 3-stack
- **Auth Tag location** (line ~228): New `- [Authentication Tag](...): MUST be deserialized...` + 2 pre-existing data-format annotations = 3-stack
- **Encrypted Content Length location** (line ~197): New + modified annotations added to pre-existing 5-stack = 7-stack

These should be addressed in a future refactoring work item that restructures the pre-existing annotation stacks in the ENDFRAME block.

### What Passed Review
- ✅ All annotation quotes match the TOML character-for-character
- ✅ Annotation targets are correct (`specification/client-apis/decrypt.md#decrypt-the-message-body`)
- ✅ No `type=implementation` explicitly specified (default used correctly)
- ✅ Test annotations use `type=test` correctly
- ✅ All 6 requirements have implementation annotations
- ✅ All 6 requirements have corresponding test annotations
- ✅ Duvet snapshot shows full coverage (`[!MUST,implementation,test]` for all requirements)
- ✅ All 36 tests pass
- ✅ Clippy passes (only pre-existing warnings in encrypt.rs and types.rs)
- ✅ Reason lines are factually correct and helpful
- ✅ Content length validation quote fixed to match TOML (was previously misquoted)
