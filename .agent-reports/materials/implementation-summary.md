## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs` — replaced all orphaned `//#*` content lines in `get_modern_decryption_materials` with proper duvet annotation blocks using exact TOML quotes and `//=` target headers

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs
```

### Requirements Addressed
- ✅ "The call to the CMM's [Decrypt Materials]...MUST be constructed as follows:" — implemented (before `input.algorithm_suite_id = ...`)
- ✅ "- Algorithm Suite ID: This MUST be the parsed [algorithm suite ID]..." — implemented (at `input.algorithm_suite_id = algorithm_suite_id;`)
- ✅ "- Commitment Policy: This MUST be the commitment policy configured on the client." — implemented (at `input.commitment_policy = ...;`)
- ✅ "- Encrypted Data Keys: This MUST be the parsed [encrypted data keys]..." — implemented (at `input.encrypted_data_keys = ...;`)
- ✅ "- Encryption Context: This MUST be the parsed [encryption context]..." — implemented (at `input.encryption_context = encryption_context;`)
- ✅ "- Reproduced Encryption Context: This MUST be the [input](#input) encryption context." — implemented (at `input.reproduced_encryption_context.clone_from(...)`)
- ✅ "This operation MUST obtain this set of [decryption materials]..." — implemented (at `let materials = cmm.decrypt_materials(&input).await?;`)
- ✅ "This CMM MUST obtain the [decryption materials]...required for decryption." — implemented (at `let materials = cmm.decrypt_materials(&input).await?;`)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_get_decryption_materials.rs` (unchanged from Round 2 — already correct)
- **Number of `type=test` annotations added**: 9 (from Round 2, unchanged)
- **Test function names**: `test_obtain_decryption_materials_via_cmm`, `test_cmm_call_constructed_as_follows`, `test_cmm_call_algorithm_suite_id`, `test_cmm_call_commitment_policy`, `test_cmm_call_encrypted_data_keys`, `test_cmm_call_encryption_context`, `test_cmm_call_reproduced_encryption_context`, `test_decrypt_fails_with_wrong_keyring`

### Annotation Structure (max 2 per line — verified)
- 2 annotations before `input.algorithm_suite_id` ("constructed as follows" + "Algorithm Suite ID" sub-item)
- 1 annotation before `input.commitment_policy`
- 1 annotation before `input.encrypted_data_keys`
- 1 annotation before `input.encryption_context`
- 1 annotation before `input.reproduced_encryption_context.clone_from(...)`
- 2 annotations before `let materials = cmm.decrypt_materials(&input).await?` ("MUST obtain" + "CMM MUST obtain")

### Proposed Commit Message

```
fix(decrypt): replace orphaned annotations in get_modern_decryption_materials

Replace all orphaned //#* content lines (paraphrased, no //= target headers)
in get_modern_decryption_materials with proper duvet annotation blocks using
exact TOML quotes and specification/client-apis/decrypt.md target headers.

Adds 8 implementation annotations covering the CMM call construction
(parent + 5 sub-items) and the CMM obtain requirements. Max 2 annotations
per code line maintained throughout.

Refs: specification/client-apis/decrypt.md#get-the-decryption-materials
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 42ms
    Scanning sources
     Scanned 134 sources 4ms
     Parsing annotations
      Parsed 703 annotations 27ms
     Loading specifications
      Loaded 9 specifications 11ms
     Mapping sections
      Mapped 104 sections 11ms
    Matching references
     Matched 1285 references 4ms
     Sorting references
      Sorted 1285 references 8ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 12ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 788µs
```

### Test Results (actual command output)
```
$ cargo test --test test_get_decryption_materials
running 8 tests
test test_decrypt_fails_with_wrong_keyring ... ok
test test_cmm_call_commitment_policy ... ok
test test_cmm_call_reproduced_encryption_context ... ok
test test_cmm_call_constructed_as_follows ... ok
test test_cmm_call_encryption_context ... ok
test test_cmm_call_algorithm_suite_id ... ok
test test_cmm_call_encrypted_data_keys ... ok
test test_obtain_decryption_materials_via_cmm ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

### Notes
- The annotation prefix used is `specification/client-apis/decrypt.md` (not `aws-encryption-sdk-specification/`) — this matches the duvet config which uses `specification/` prefix and the legacy path in the same file.
- The test file uses `aws-encryption-sdk-specification/` prefix but tests live in `tests/` which is outside the duvet source scan pattern (`./src/**/*.rs`). This is consistent with the existing test file from Round 2.
- All pre-existing clippy warnings (missing_docs, collapsible_if) are unrelated to this change.
