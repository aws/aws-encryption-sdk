## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs` — Rewrote tests to parse output ciphertext bytes and assert on frame structure (regular/final frame counts, content lengths), while keeping round-trip assertions and all existing duvet annotations.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs
```

### Requirements Addressed
- ✅ All 7 existing `type=test` annotations preserved
- ✅ Each test now asserts on frame structure by parsing output bytes
- ✅ `count_frames` helper walks frame boundaries to count regular vs final frames
- ✅ `final_frame_content_length` helper extracts content length from final frame
- ✅ Exact-match case (Req 4) verified: 0 regular frames, 1 final frame, content_length=10

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs`
- **Number of `type=test` annotations**: 7 (unchanged, all preserved)
- **Test function names**: `test_regular_frame_serialization_conforms_to_spec`, `test_process_consumable_bytes_as_regular_frames`, `test_end_of_input_processing`, `test_exact_frame_length_constructs_final_or_regular`, `test_enough_bytes_constructs_regular_frame`, `test_not_enough_bytes_constructs_final_frame`, `test_empty_plaintext_constructs_empty_final_frame`

### Structural Assertions Added
| Test | Regular Frames | Final Frames | Final Content Length |
|------|---------------|-------------|---------------------|
| regular_frame_serialization (30B, frame=10) | 2 | 1 | — |
| process_consumable_bytes (50B, frame=10) | 4 | 1 | — |
| end_of_input (15B, frame=10) | 1 | 1 | 5 |
| exact_frame_length (10B, frame=10) | 0 | 1 | 10 |
| enough_bytes (25B, frame=10) | 2 | 1 | 5 |
| not_enough_bytes (7B, frame=10) | 0 | 1 | 7 |
| empty_plaintext (0B, frame=4096) | 0 | 1 | 0 |

### Proposed Commit Message
```
test(encrypt): parse output bytes to verify frame structure in construct-the-body tests

Update test_construct_the_body.rs to parse ciphertext bytes and assert
on frame structure (regular/final frame counts, content lengths) instead
of relying solely on round-trip equality checks.

Added helpers: count_frames() walks frame boundaries to count regular vs
final frames; final_frame_content_length() extracts the content length
field from the final frame.

Key verification: exact-match case (plaintext == frame_length) produces
0 regular frames and 1 final frame with content_length == frame_length.

Spec: aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-body
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 45ms
    Scanning sources
     Scanned 134 sources 2ms
     Parsing annotations
      Parsed 679 annotations 20ms
     Loading specifications
      Loaded 9 specifications 8ms
     Mapping sections
      Mapped 104 sections 10ms
    Matching references
     Matched 1230 references 3ms
     Sorting references
      Sorted 1230 references 7ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 12ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 709µs
```

### Test Results (actual command output)
```
$ cargo test --test test_construct_the_body
running 7 tests
test test_empty_plaintext_constructs_empty_final_frame ... ok
test test_enough_bytes_constructs_regular_frame ... ok
test test_process_consumable_bytes_as_regular_frames ... ok
test test_not_enough_bytes_constructs_final_frame ... ok
test test_exact_frame_length_constructs_final_or_regular ... ok
test test_regular_frame_serialization_conforms_to_spec ... ok
test test_end_of_input_processing ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

### Notes
- The `count_frames` helper uses a two-phase approach: first `find_body_start` locates the first frame by scanning for SeqNum=1 and validating that walking regular frames leads to an ENDFRAME marker, then it walks through frames counting regular vs final.
- Round-trip assertions are preserved alongside the new structural assertions for defense-in-depth.
- All 5 pre-existing lib warnings are unrelated to this change (encrypt.rs doc comments, materials.rs unreachable patterns, missing docs).
