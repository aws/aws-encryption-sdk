## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs` — Added `type=implication` annotation for framed data max frame size requirement before `let frame_length = header.body.frame_length() as usize;`

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs
```

### Requirements Addressed
- ✅ `- The total bytes allowed in a single frame MUST be less than or equal to 2^32 - 1.` — implemented (implication) + tested (existing test `test_framed_data_max_frame_size`)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: None (test already exists)
- **Number of `type=test` annotations added**: 0 new (1 already exists at `tests/test_message_body_format.rs` line 125-127)
- **Test function names**: `test_framed_data_max_frame_size` (pre-existing)

### Proposed Commit Message

```
feat(message-body): add implication annotation for framed data max frame size

Add type=implication annotation for the framed-data requirement that
total bytes in a single frame MUST be <= 2^32 - 1. The constraint is
enforced by the type system: frame_length() returns u32, bounded by
2^32 - 1.

Spec: specification/data-format/message-body.md#framed-data
```

### Duvet Verification (actual command output)
```
$ make duvet
    Scanning sources
     Scanned 543 sources 16ms
     Parsing annotations
      Parsed 2666 annotations 69ms
     Loading specifications
      Loaded 68 specifications 19ms
     Mapping sections
      Mapped 498 sections 10ms
    Matching references
     Matched 4496 references 4ms
     Sorting references
      Sorted 4496 references 21ms
     Writing specification_compliance_report.html
       Wrote specification_compliance_report.html 16ms
```

### Test Results (actual command output)
```
$ cargo test test_framed_data_max_frame_size --test test_message_body_format
running 1 test
test test_framed_data_max_frame_size ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.02s
```

### Notes
- This is a minimal change: a single 4-line annotation block added before an existing line of code.
- The annotation uses `type=implication` because the constraint is structural (enforced by the Rust type system via `u32`) and cannot be directly tested.
- The corresponding `type=test` annotation already exists in `test_message_body_format.rs` at line 125-127.
- No clippy warnings in `body.rs`.
