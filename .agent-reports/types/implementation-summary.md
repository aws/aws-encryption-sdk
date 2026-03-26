## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs` — Added 3 duvet annotations: `type=implication` on `encryption_context` field, and 2 implementation annotations on `DecryptInput::validate()`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs` — Added 2 `type=test` annotations to `test_bad_decrypt_input`

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs
```

### Requirements Addressed
- ✅ `The Decrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.` — implemented + tested
- ✅ `If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.` — implemented + tested
- ✅ `- The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.` — annotated as implication (structural field existence)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_encrypt_decrypt.rs`
- **Number of `type=test` annotations added**: 2 for 2 requirements (the 3rd requirement uses `type=implication` which satisfies both implementation and test checks)
- **Test function names**: `test_bad_decrypt_input`

### Proposed Commit Message

```
feat(decrypt): add missing duvet annotations for DecryptInput validation and encryption context

Add duvet annotations for three decrypt.md#input requirements:
- DecryptInput::validate() annotated for keyring/CMM validation requirement
- DecryptInput::validate() annotated for failure requirement
- DecryptInput.encryption_context field annotated as implication for optional EC acceptance
- test_bad_decrypt_input annotated with type=test for validation/failure requirements

Spec section: specification/client-apis/decrypt.md#input
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 34ms
    Scanning sources
     Scanned 148 sources 3ms
     Parsing annotations
      Parsed 806 annotations 35ms
     Loading specifications
      Loaded 12 specifications 16ms
     Mapping sections
      Mapped 112 sections 13ms
    Matching references
     Matched 1486 references 4ms
     Sorting references
      Sorted 1486 references 10ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 17ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 969µs
```

### Test Results (actual command output)
```
$ cargo check
warning: `aws-esdk` (lib) generated 5 warnings (pre-existing)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
```

Duvet snapshot confirms all 3 requirements are covered:
```
TEXT[!MUST,implementation,test]:   The Decrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
TEXT[!MUST,implementation,test]:   If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.
TEXT[!MUST,implication]: - The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
```

### Notes
- The `cargo clippy -- -D warnings` fails due to pre-existing `missing_docs` warnings unrelated to this change.
- `test_bad_decrypt_input` is an integration test requiring KMS access; `cargo check` confirms compilation. The test annotations are correctly placed and duvet picks them up.
- The `encryption_context` cross-reference link `[Encryption Context](#encryption-context)` is an internal page anchor to `decrypt.md#encryption-context`. The TOML for that section only contains a MAY about output, not relevant to input field acceptance, so no cross-reference annotation was added.
