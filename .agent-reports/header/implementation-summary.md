## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs` — Added missing first line of the `#message-id` randomness annotation quote ("While implementations cannot guarantee complete uniqueness,")

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs
```

### Requirements Addressed
- ✅ "While implementations cannot guarantee complete uniqueness, implementations MUST use a good source of randomness when generating messages IDs in order to make the chance of duplicate IDs negligible." — implementation annotation now uses exact full TOML quote; tests already existed

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: None — `type=test` annotations already exist in both test files
- **Number of `type=test` annotations added**: 0 (2 already present: 1 in `test_v1_header_body.rs`, 1 in `test_v2_header_body.rs`)
- **Test function names**: `test_v1_header_message_id`, `test_v2_header_message_id` (pre-existing, already contain the full 3-line quote with `type=test`)

### Proposed Commit Message

```
fix(message-header): complete message-id randomness annotation quote

Add missing first line "While implementations cannot guarantee complete
uniqueness," to the implementation annotation on generate_message_id()
in header.rs. The annotation now matches the exact TOML quote from
specification/data-format/message-header.md#message-id.

Test annotations in test_v1_header_body.rs and test_v2_header_body.rs
already included the full quote.

Spec: specification/data-format/message-header.md#message-id
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 24ms
    Scanning sources
     Scanned 159 sources 2ms
     Parsing annotations
      Parsed 1058 annotations 28ms
     Loading specifications
      Loaded 14 specifications 19ms
     Mapping sections
      Mapped 140 sections 18ms
    Matching references
     Matched 1937 references 3ms
     Sorting references
      Sorted 1937 references 13ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 16ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 981µs
```

Snapshot confirms full coverage:
```
  SECTION: [Message ID](#message-id)
    TEXT[!MUST,implementation,test]: While implementations cannot guarantee complete uniqueness,
    TEXT[!MUST,implementation,test]: implementations MUST use a good source of randomness when generating messages IDs in order to make
    TEXT[!MUST,implementation,test]: the chance of duplicate IDs negligible.
```

### Test Results (actual command output)
```
$ cargo test message_id
running 1 test
test test_v1_header_message_id ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.03s

running 1 test
test test_v2_header_message_id ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.03s
```

### Notes
- This is an annotation-only fix — no logic changes. The `generate_message_id` function was already correct; only the annotation quote was incomplete.
- Pre-existing clippy warnings exist in unrelated files (`encrypt.rs` missing docs, `v1_header_body.rs` collapsible if) — not introduced by this change.
