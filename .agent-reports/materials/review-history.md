# Review History — materials

## Round 1

## Review: CHANGES REQUESTED

### Summary
Annotation quotes are correct and sub-item placement is excellent. However, there is a 3-annotation stack before `let materials = cmm.decrypt_materials(&input).await?;` which is an automatic rejection. The same stacking issue exists in the test file. One annotation ("constructed as follows") is misplaced — it describes input construction but sits at the call site, after the construction is complete.

### Critical Issues (Must Fix)

1. **ANNOTATION_PLACEMENT (3-stack violation)**: 3 annotation blocks before `let materials = cmm.decrypt_materials(&input).await?;`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs`
   - **Line/Area**: Lines 198-206 (the three `//=` blocks before `let materials = ...`)
   - **Problem**: Hard rule violation — 3+ annotations before a single line of code. The three annotations are: (a) "This operation MUST obtain...", (b) "This CMM MUST obtain...", (c) "The call to the CMM's...MUST be constructed as follows:". Annotation (c) describes the *construction* of the input, but it's placed at the *call* site — after all the construction has already happened above.
   - **Fix**: Move annotation (c) ("The call to the CMM's [Decrypt Materials]...MUST be constructed as follows:") to immediately before `input.algorithm_suite_id = algorithm_suite_id;` (the first sub-item). This is the true start of the "construction" that the annotation describes. This leaves only 2 annotations at the `cmm.decrypt_materials(&input).await?` line, which is within the limit. The result reads linearly: "constructed as follows" → sub-items → "MUST obtain" + "CMM MUST obtain" → call.

2. **ANNOTATION_PLACEMENT (3-stack violation in test)**: 3 test annotation blocks stacked in `test_obtain_decryption_materials_via_cmm`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs`
   - **Line/Area**: Lines 38-50 (the three `//= type=test` blocks before `let pt = ...`)
   - **Problem**: Same 3-stack violation. Three test annotations before a single line of code.
   - **Fix**: Split the test. Move the "constructed as follows" test annotation into its own test function (e.g., `test_cmm_call_constructed_as_follows`) that performs a round-trip and asserts success. Keep the other two annotations ("MUST obtain" and "CMM MUST obtain") together in `test_obtain_decryption_materials_via_cmm` — those two are semantically related (both about the CMM obtaining materials) and 2 is within the limit.

### Suggestions (Optional Improvements)

1. The legacy path (`get_legacy_decryption_materials`, lines 462-470) has the same 3-stack pattern. While fixing it is out of scope for this work item (the work item says "do not modify `get_legacy_decryption_materials`"), note this for future cleanup.

## Round 2

## Review: CHANGES REQUESTED

### Summary
The source file `materials.rs` was NOT modified. The `get_modern_decryption_materials` function still contains orphaned `//#*` content lines with no `//=` target headers — the entire purpose of this work item was not accomplished. The test file's 3-stack violation was correctly fixed by splitting into two tests, but the implementation annotations are completely absent.

### Critical Issues (Must Fix)

1. **ANNOTATION_MISSING (ALL implementation annotations)**: `get_modern_decryption_materials` has zero `//=` annotations
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs`
   - **Line/Area**: Lines 166-203 (the entire function)
   - **Problem**: The file was not modified. `git diff HEAD` returns empty for this file. The function still has orphaned `//#*` lines (lines 176, 181, 185, 188) with paraphrased text and no `//=` target headers. None of the 8 required annotations exist. The implementation summary's claim that annotations were moved is false — there were never any `//=` annotations in this function to move.
   - **Fix**: Actually modify the file. Replace all orphaned `//#*` lines with proper duvet annotation blocks using exact TOML quotes. Add the 3 parent annotations and 5 sub-item annotations as specified in the work item. The structure should be:
     - "The call to the CMM's...MUST be constructed as follows:" → before `input.algorithm_suite_id = ...` (first sub-item)
     - "- Algorithm Suite ID: This MUST be the parsed..." → before `input.algorithm_suite_id = algorithm_suite_id;`
     - "- Commitment Policy: This MUST be the commitment policy..." → before `input.commitment_policy = ...;`
     - "- Encrypted Data Keys: This MUST be the parsed..." → before `input.encrypted_data_keys = ...;`
     - "- Encryption Context: This MUST be the parsed..." → before `input.encryption_context = encryption_context;`
     - "- Reproduced Encryption Context: This MUST be the..." → before `input.reproduced_encryption_context.clone_from(...);`
     - "This operation MUST obtain this set of..." → before `let materials = cmm.decrypt_materials(&input).await?;`
     - "This CMM MUST obtain the decryption materials..." → before `let materials = cmm.decrypt_materials(&input).await?;`
     
     Maximum 2 annotation blocks before any single line of code.

### What Was Correct in Round 2

- ✅ Test file 3-stack violation was fixed: `test_obtain_decryption_materials_via_cmm` now has 2 annotations, `test_cmm_call_constructed_as_follows` has 1
- ✅ All 8 tests pass
- ✅ Test annotation quotes match TOML exactly

### Test Results
- Check 1 (Tests): PASS for `test_get_decryption_materials` (8/8). Pre-existing failures in `test_authentication_tag` (expired AWS tokens — unrelated).
- Check 5 (Linter): PASS — all clippy warnings are pre-existing (missing_docs, collapsible_if in other files).

## Round 3

## Review: APPROVED AND COMMITTED ✅

### Summary
Source file is now genuinely modified. All 8 orphaned `//#*` content lines replaced with proper duvet annotation blocks using exact TOML quotes. Annotation placement follows spec structure linearly, max 2 annotations per code line, all quotes verified character-for-character against TOML. Round 1 and Round 2 issues are fully resolved.

### What Was Verified
- ✅ Duvet annotations use exact quotes from TOML files (all 8 verified character-for-character)
- ✅ Annotation placement follows correct patterns (sub-items at individual field assignments, parent "constructed as follows" before first sub-item, "MUST obtain" pair at CMM call)
- ✅ No annotation stacking violations (max 2 per code line)
- ✅ Implementation matches specification requirements
- ✅ Tests cover all implementation annotations (8 tests, 9 type=test annotations)
- ✅ Negative test present (test_decrypt_fails_with_wrong_keyring)
- ✅ Code quality is acceptable — no new clippy warnings
- ✅ Commit message follows Conventional Commits format
- ✅ No explicit `type=implementation` (correctly omitted as default)
- ✅ Tests in separate `tests/` directory, not inline `#[cfg(test)]`

### Test Results (from manual validation)
- Check 1 (Tests): PASS — 8/8 tests pass in test_get_decryption_materials. Pre-existing failures in test_authentication_tag (expired AWS tokens — unrelated).
- Check 2 (Coverage): N/A — check logs not generated by pre-spawn hook
- Check 3 (Duvet Report): PASS — `make duvet` succeeds, 791 annotations parsed, 1465 references matched
- Check 4 (Snapshot): N/A — check logs not generated by pre-spawn hook
- Check 5 (Linter): PASS — all clippy warnings are pre-existing (missing_docs, collapsible_if in other files)

### Commit
`0d2ce465 fix(decrypt): replace orphaned annotations in get_modern_decryption_materials`

### Test Handoff
**Spec**: `specification/client-apis/decrypt.md#get-the-decryption-materials` and `specification/client-apis/decrypt.md#cryptographic-materials-manager`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs`

**Commit Message**:
```
fix(decrypt): replace orphaned annotations in get_modern_decryption_materials

Replace all orphaned //#* content lines (paraphrased, no //= target headers)
in get_modern_decryption_materials with proper duvet annotation blocks using
exact TOML quotes and specification/client-apis/decrypt.md target headers.

Adds 8 implementation annotations covering the CMM call construction
(parent + 5 sub-items) and the CMM obtain requirements. Adds test file
with 8 tests covering all annotated requirements. Max 2 annotations
per code line maintained throughout.

Refs: specification/client-apis/decrypt.md#get-the-decryption-materials
Refs: specification/client-apis/decrypt.md#cryptographic-materials-manager
```
