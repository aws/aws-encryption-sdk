## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs` — Added two missing serialization order annotations to `write_v2_header_body`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs` — Fixed 2 clippy warnings (`pos + 1 <= ct.len()` → `pos < ct.len()`)
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/.duvet/config.toml` — Added `./tests/**/*.rs` source pattern so duvet picks up test annotations

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/.duvet/config.toml AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs
```

### Requirements Addressed
- ✅ `The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.` — implemented (implication) + tested
- ✅ `The V2 Header Body MUST be serialized as, in order, Version, Algorithm Suite ID, Message ID, AAD, Encrypted Data Keys, Content Type, Frame Length, and Algorithm Suite Data.` — implemented (implication, cross-reference) + tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs`
- **Number of `type=test` annotations added**: 2 for 2 requirements (already present from Round 1, now picked up by duvet via config change)
- **Test function names**: `test_v2_header_body_serialization_order`

### Proposed Commit Message

```
feat(message-header): add serialization order annotations for V2 header body

Add two missing duvet annotations to write_v2_header_body that cover
the overall serialization order constraint from both the encrypt.md
and message-header.md specifications.

- encrypt.md#v2-header: "The serialization order MUST follow..."
- data-format/message-header.md#header-body-version-2-0: "The V2 Header
  Body MUST be serialized as, in order, ..."

Also adds ./tests/**/*.rs to duvet config so test annotations are
scanned, and fixes 2 clippy warnings in the test file.

Refs: specification/client-apis/encrypt.md#v2-header,
      specification/data-format/message-header.md#header-body-version-2-0
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 34ms
    Scanning sources
     Scanned 147 sources 2ms
     Parsing annotations
      Parsed 791 annotations 28ms
     Loading specifications
      Loaded 11 specifications 15ms
     Mapping sections
      Mapped 109 sections 12ms
    Matching references
     Matched 1465 references 4ms
     Sorting references
      Sorted 1465 references 9ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 14ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 788µs
```

Snapshot confirms both requirements have `implication,test` coverage:
- `TEXT[!MUST,implication,test]: The serialization order MUST follow the [Header Body Version 2.0]...`
- `TEXT[!MUST,implication,test]: The V2 Header Body MUST be serialized as, in order,...`

### Test Results (actual command output)
```
$ cargo test test_v2_header_body
running 1 test
test test_v2_header_body_serialization_order ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

### Notes
- Pre-existing clippy errors in other files (encrypt.rs: missing docs, unreachable patterns, collapsible if-statements) are unrelated to this work item.
- The nested if-let clippy warning in `read_v2_header_body` (line 119) is pre-existing code that was not modified — only the line number shifted due to added annotations above it.
- The `./tests/**/*.rs` duvet config addition is needed for ALL test files with `type=test` annotations, not just this work item's test file.
