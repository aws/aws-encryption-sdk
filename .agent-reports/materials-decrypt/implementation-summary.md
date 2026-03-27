## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs` — Fixed 33 annotation path prefixes from `specification/` to `aws-encryption-sdk-specification/`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_post_cmm_validation.rs` — Fixed all annotation path prefixes from `specification/` to `aws-encryption-sdk-specification/`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_key_derivation.rs` — Fixed 3 annotation path prefixes from `specification/` to `aws-encryption-sdk-specification/`

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/materials.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_post_cmm_validation.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_key_derivation.rs
```

### Requirements Addressed
- ✅ Req 880: `If this algorithm suite is not [supported for the ESDK](...) encrypt MUST yield an error.` — `type=implication` in materials.rs now uses correct prefix, visible to duvet
- ✅ Req 881: `If the algorithm suite is not supported by the [commitment policy](...) configured in the [client](...) decrypt MUST yield an error.` — `type=test` annotations in test_post_cmm_validation.rs now use correct prefix, visible to duvet
- ✅ Req 885: `If the key derivation algorithm is the [identity KDF](...), then the derived data key MUST be the same as the plaintext data key.` — `type=test` annotation in test_post_cmm_validation.rs now uses correct prefix, visible to duvet

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_post_cmm_validation.rs`
- **Number of `type=test` annotations added**: 0 new annotations; 5 existing `type=test` annotations were fixed by correcting the path prefix so duvet can now match them
- **Test function names**: `test_post_cmm_commitment_policy_encrypt`, `test_post_cmm_commitment_policy_decrypt`, `test_encrypt_non_committing_with_require_policy_fails`, `test_decrypt_non_committing_with_require_policy_fails`, `test_identity_kdf_decrypt`

### Proposed Commit Message

```
fix(decrypt): correct annotation path prefix for get-the-decryption-materials

Fix duvet annotation path prefixes from `specification/` to
`aws-encryption-sdk-specification/` in materials.rs,
test_post_cmm_validation.rs, and test_key_derivation.rs.

The `specification/` symlink prefix did not match the TOML target
prefix `aws-encryption-sdk-specification/`, causing duvet to miss
existing annotations for 3 requirements in
decrypt.md#get-the-decryption-materials (Reqs 880, 881, 885).

Spec: aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
```

### Duvet Verification (actual command output)
```
$ make duvet
   Extracted 132 requirements across 17 sections 3ms
  Extracting requirements from aws-encryption-sdk-specification/client-apis/client.md
   Extracted 8 requirements across 4 sections 2ms
  ...
    Scanning sources
     Scanned 543 sources 17ms
     Parsing annotations
      Parsed 2664 annotations 96ms
     Loading specifications
      Loaded 68 specifications 33ms
     Mapping sections
      Mapped 497 sections 9ms
    Matching references
     Matched 4494 references 4ms
     Sorting references
      Sorted 4494 references 20ms
     Writing specification_compliance_report.html
       Wrote specification_compliance_report.html 14ms
```

### Test Results (actual command output)
```
$ cargo test --test test_post_cmm_validation -- --nocapture
test test_encrypt_non_committing_with_require_policy_fails ... ok
test test_decrypt_non_committing_with_require_policy_fails ... ok
test test_post_cmm_commitment_policy_decrypt ... ok
test test_identity_kdf_decrypt ... ok
test test_post_cmm_commitment_policy_encrypt ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --test test_get_decryption_materials -- --nocapture
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --test test_key_derivation -- --nocapture
(tests pass - file only has encrypt annotations, not decrypt)
```

### Notes
- The clippy errors (`missing_docs`, `collapsible_if`) are pre-existing and unrelated to this change.
- 163 occurrences of `//= specification/` remain in other test files (test_v1_header_body.rs, test_encrypt_behavior.rs, etc.) — these are out of scope for this task as they cover different spec sections.
- No code logic was changed; only annotation comment prefixes were corrected.
