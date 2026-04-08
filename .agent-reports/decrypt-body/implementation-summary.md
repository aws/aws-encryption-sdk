# Implementation Summary — decrypt-body (Cycle 2)

## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs` — Two fixes applied:
  1. Moved the "For a final frame, each field MUST be deserialized..." parent annotation from inside the `if seq_num == ENDFRAME_SEQUENCE_NUMBER` block to before the `if` statement (reduces annotation stack inside block from 3 to 2).
  2. Encrypt-side annotations: Could NOT revert — the old quotes ("The Sequence Number End MUST only be serialized for the final frame." and "The Encrypted Content Length MUST only be serialized for the final frame.") no longer exist in the spec or TOML. The spec was updated in a prior commit and the TOML only contains the new quotes. Reverting would cause duvet validation failures. Kept the current (spec-matching) quotes.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs
```

### Requirements Addressed
- ✅ ANNOTATION_PLACEMENT: Parent annotation moved before `if` statement
- ⚠️ SCOPE_CREEP: Cannot revert — old quotes don't exist in current spec/TOML. See Notes.

### Notes
- The SCOPE_CREEP issue (#2) cannot be addressed as requested. The old annotation quotes ("The Sequence Number End MUST only be serialized for the final frame." and "The Encrypted Content Length MUST only be serialized for the final frame.") were removed from the specification in a prior change. The `construct-a-frame.toml` file does not contain these quotes. Reverting the annotations causes `make duvet` to fail with "could not find text in section" errors. The current annotations use quotes that match the current spec and pass duvet validation.

### Duvet Verification (actual command output)
```
$ make duvet
     Loading specifications
      Loaded 69 specifications 34ms
     Mapping sections
      Mapped 526 sections 19ms
    Matching references
     Matched 4878 references 7ms
     Sorting references
      Sorted 4878 references 22ms
     Writing specification_compliance_report.html
       Wrote specification_compliance_report.html 27ms
```

### Test Results (actual command output)
```
$ cargo test --test test_decrypt_the_message_body
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --test test_construct_a_frame
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Proposed Commit Message
```
fix(decrypt): move final-frame parent annotation before if statement

Move the "For a final frame, each field MUST be deserialized..." parent
annotation from inside the ENDFRAME_SEQUENCE_NUMBER block to before the
if statement, reducing the annotation stack inside the block from 3 to 2.

Refs: specification/client-apis/decrypt.md#decrypt-the-message-body
```
