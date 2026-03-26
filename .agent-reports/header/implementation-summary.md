## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs` — added missing implementation annotation inside `serialize_header()` before `let mut w = Vec::new();`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs` — fixed both test annotation target paths from `aws-encryption-sdk-specification/...` to `specification/...`

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs
```

### Requirements Addressed
- ✅ "The header MUST be serialized as, in order, Header Body, and Header Authentication." — implemented (annotation added in `serialize_header`) + tested
- ✅ "The message header is a sequence of bytes that MUST be in big-endian format." — implemented (annotation already existed) + tested (annotation path fixed)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs`
- **Number of `type=test` annotations added**: 2 (paths corrected, annotations were already present)
- **Test function names**: `test_header_big_endian_format`, `test_header_serialization_order`

### Proposed Commit Message

```
fix(message-header): add missing serialization order annotation and fix test annotation paths

Add the missing implementation annotation for the header serialization
order requirement inside serialize_header() in header.rs. Fix both test
annotation target paths in test_header_structure.rs to use the correct
`specification/...` prefix matching the TOML target.

Spec: aws-encryption-sdk-specification/data-format/message-header.md#structure
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 32ms
    Scanning sources
     Scanned 134 sources 2ms
     Parsing annotations
      Parsed 701 annotations 19ms
     Loading specifications
      Loaded 9 specifications 9ms
     Mapping sections
      Mapped 104 sections 10ms
    Matching references
     Matched 1275 references 4ms
     Sorting references
      Sorted 1275 references 8ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 16ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 762µs
```

### Test Results (actual command output)
```
$ cargo test test_header
running 2 tests
test test_header_serialization_order ... ok
test test_header_big_endian_format ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

### Notes
- Clippy errors are pre-existing (missing docs, unreachable patterns, etc.) and unrelated to this work item
- The duvet config only scans `./src/**/*.rs` so test annotations in `tests/` are not counted by duvet — this is a systemic issue noted by the reviewer as non-blocking
