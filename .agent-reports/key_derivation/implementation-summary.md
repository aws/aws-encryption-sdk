## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_key_derivation.rs` — NEW: test file with 3 round-trip tests covering key derivation requirements

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_key_derivation.rs
```

### Requirements Addressed
- ✅ "The algorithm used to derive a data key from the plaintext data key MUST be the [key derivation algorithm]... included in the [algorithm suite]... defined above." — tested via round-trip with HKDF suite
- ✅ "If the key derivation algorithm is the [identity KDF]..., then the derived data key MUST be the same as the plaintext data key." — tested via round-trip with `AlgAes256GcmIv12Tag16NoKdf`
- ✅ "If the key derivation algorithm is [HKDF]..., the derivation process used MUST be the process described in [HKDF Encryption Key]..." — tested via round-trip with `AlgAes256GcmIv12Tag16HkdfSha256`

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_key_derivation.rs`
- **Number of `type=test` annotations added**: 3 for 3 requirements
- **Test function names**: `test_key_derivation_uses_suite_kdf`, `test_identity_kdf_derived_key_equals_plaintext_key`, `test_hkdf_derivation_process`

### Proposed Commit Message

```
test(encrypt): add test annotations for key derivation requirements

Add type=test duvet annotations for three key derivation requirements
in encrypt.md#get-the-encryption-materials:
- Parent: algorithm used MUST be the KDF from the algorithm suite
- Identity KDF: derived key MUST equal plaintext data key
- HKDF: derivation MUST follow HKDF Encryption Key process

Each test uses encrypt-then-decrypt round-trip verification with
the appropriate algorithm suite and commitment policy.

Spec: specification/client-apis/encrypt.md#get-the-encryption-materials
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 29ms
    Scanning sources
     Scanned 152 sources 3ms
     Parsing annotations
      Parsed 827 annotations 32ms
     Loading specifications
      Loaded 12 specifications 15ms
     Mapping sections
      Mapped 113 sections 13ms
    Matching references
     Matched 1518 references 4ms
     Sorting references
      Sorted 1518 references 9ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 15ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 892µs
```

### Test Results (actual command output)
```
$ cargo test --test test_key_derivation
running 3 tests
test test_key_derivation_uses_suite_kdf ... ok
test test_identity_kdf_derived_key_equals_plaintext_key ... ok
test test_hkdf_derivation_process ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

### Notes
- Pre-existing clippy warnings in `encrypt.rs` and `materials.rs` are unrelated to this change
- Used `specification/` prefix (not `aws-encryption-sdk-specification/`) to match the source code annotations and duvet config
- All three requirements now show `implementation,test` in the duvet snapshot
