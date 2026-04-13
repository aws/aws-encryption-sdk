## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs` — removed 2 blank lines between annotation blocks; re-wrapped v2-authentication-tag quote to match TOML line breaks and removed trailing space
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs` — removed 3 blank lines between annotation blocks

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header_auth.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs
```

### Requirements Addressed
- ✅ Issue 1: Blank line between v1 "in order" annotation and v1-auth-tag IV annotation — removed
- ✅ Issue 2: Blank line between v2 "auth tag only" annotation and v2-auth-tag annotation — removed
- ✅ Issue 3: v2-authentication-tag quote line-wrapping mismatch + trailing space — re-wrapped to match TOML
- ✅ Issue 4: Blank line between v1-auth-tag IV and auth-tag test annotations — removed
- ✅ Issue 5: Blank line between v2-auth-tag main quote and auth-tag sub-item test annotations — removed
- ✅ Issue 6: Blank line between v2-auth-tag auth-tag sub-item and `let pt` — removed

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_auth.rs`
- **Number of `type=test` annotations added**: 0 new (existing annotations reformatted only — blank line removal)
- **Test function names**: `test_v1_encrypt_header_auth_tag_serialization`, `test_v2_encrypt_header_auth_tag_serialization` (annotation formatting fixed in both)

### Proposed Commit Message

```
fix(header-auth): fix annotation placement and quote formatting

Remove blank lines between adjacent annotation blocks in both
header_auth.rs and test_header_auth.rs to comply with Core Rule 4
(executable line must immediately follow annotation). Re-wrap the
v2-authentication-tag quote in the source file to match TOML line
breaks exactly and remove trailing whitespace.

Spec: specification/data-format/message-header.md#header-authentication-version-1-0
Spec: specification/data-format/message-header.md#header-authentication-version-2-0
Spec: specification/client-apis/encrypt.md#v1-authentication-tag
Spec: specification/client-apis/encrypt.md#v2-authentication-tag
```

### Duvet Verification (actual command output)
```
$ make duvet
  Extracting requirements from aws-encryption-sdk-specification/data-format/message-body.md
   Extracted 48 requirements across 18 sections 4ms
  Extracting requirements from aws-encryption-sdk-specification/data-format/message-body-aad.md
   Extracted 16 requirements across 5 sections 2ms
duvet \
		report \
		--spec-pattern "compliance/**/*.toml" \
		--source-pattern "AwsCryptographicMaterialProviders/dafny/**/src/**/*.dfy" \
		--source-pattern "AwsCryptographicMaterialProviders/dafny/**/Model/**/*.smithy" \
		--source-pattern "AwsCryptographicMaterialProviders/compliance_exceptions/**/*.txt" \
		--source-pattern "(# //=,# //#).github/workflows/duvet.yaml" \
		--source-pattern "AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/**/*.rs" \
		--source-pattern "AwsEncryptionSDK/runtimes/rust/esdk_rust/aws-esdk-cxx/**/*.rs" \
		--source-pattern "AwsEncryptionSDK/runtimes/rust/esdk_rust/prim/**/*.rs" \
		--source-pattern "compliance_exceptions/**/*.txt" \
		--html specification_compliance_report.html
    Scanning sources
     Scanned 552 sources 30ms
     Parsing annotations
      Parsed 2996 annotations 100ms
     Loading specifications
      Loaded 69 specifications 31ms
     Mapping sections
      Mapped 528 sections 20ms
    Matching references
     Matched 5088 references 7ms
     Sorting references
      Sorted 5088 references 24ms
     Writing specification_compliance_report.html
       Wrote specification_compliance_report.html 27ms
```

### Test Results (actual command output)
```
$ cargo test --test test_header_auth
running 7 tests
test test_v1_header_auth_tag_length_and_bytes ... ok
test test_v1_header_auth_serialization_order ... ok
test test_v1_encrypt_header_auth_tag_serialization ... ok
test test_v1_header_auth_iv_length_and_bytes ... ok
test test_v2_encrypt_header_auth_tag_serialization ... ok
test test_v2_header_auth_serialization ... ok
test test_v2_header_auth_tag_length_and_bytes ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### Notes
- All 6 issues from the work item are resolved. No executable code was changed — only annotation formatting (blank line removal and quote re-wrapping).
- No trailing whitespace remains in the modified source file.
