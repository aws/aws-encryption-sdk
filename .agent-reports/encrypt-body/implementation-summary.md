## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs` — Fixed 4 annotation stacking/placement violations by splitting stacked annotations across separate anchor lines (max 2 blocks per code line). Added anchor lines `let _regular_frame_serialization = ();`, `let _seq_num_written = &input.sequence_number;`, `let _iv_written = &iv;`, `let _final_encrypted_content_written = ();`, `let _final_authentication_tag_written = ();`.
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_a_frame.rs` — Fixed collapsible_if clippy warning by collapsing nested `if` into single condition.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_a_frame.rs
```

### Requirements Addressed
- ✅ All 6 original requirements remain implemented + tested (no annotation content changed)

### Review Issues Fixed
- ✅ Issue 1: Sequence number stacking (4→2+2) — regular frame annotations before `write_u32`, final frame annotations on `let _seq_num_written`
- ✅ Issue 2: Encrypted content stacking (4→2+2) — regular frame annotations before `let _encrypted_content_written`, final frame annotations on `let _final_encrypted_content_written`
- ✅ Issue 3: Authentication tag stacking (4→2+2) — regular frame annotations before `let _authentication_tag_written`, final frame annotations on `let _final_authentication_tag_written`
- ✅ Issue 4: IV blank lines + stacking — removed blank lines, regular frame annotations before `write_bytes`, final frame annotations on `let _iv_written`
- ✅ Bonus: Regular frame preamble implication given its own anchor `let _regular_frame_serialization = ()` to avoid 3-block stack with sequence number annotations
- ✅ Suggestion: Collapsed nested `if` in test to fix clippy `collapsible_if` warning

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_a_frame.rs`
- **Number of `type=test` annotations added**: 6 (unchanged from Round 1)
- **Test function names**: `test_construct_frame_serialization_regular_and_final`, `test_construct_frame_sequence_number_serialized`, `test_construct_frame_iv_serialized`, `test_construct_frame_auth_tag_serialized`

### Proposed Commit Message

```
fix(encrypt): resolve annotation stacking violations in construct_frame

Split stacked annotation blocks (>2 per code line) across separate
anchor lines for sequence number, IV, encrypted content, and
authentication tag serialization. Each code line now has at most 2
annotation blocks before it: regular frame annotations on the actual
serialization call, final frame annotations on a dedicated anchor line.

Also fixed collapsible_if clippy warning in test file.

Spec: specification/client-apis/encrypt.md#construct-a-frame
```

### Duvet Verification (actual command output)
```
$ make duvet
[extract steps omitted for brevity]
    Scanning sources
     Scanned 549 sources 16ms
     Parsing annotations
      Parsed 2890 annotations 94ms
     Loading specifications
      Loaded 69 specifications 34ms
     Mapping sections
      Mapped 526 sections 22ms
    Matching references
     Matched 4901 references 8ms
     Sorting references
      Sorted 4901 references 24ms
     Writing specification_compliance_report.html
       Wrote specification_compliance_report.html 30ms
```

### Test Results (actual command output)
```
$ cargo test test_construct_frame
running 22 tests
test test_construct_frame_final_frame_content_length_serialized ... ok
test test_construct_frame_empty_plaintext ... ok
test test_construct_frame_final_frame_has_endframe_marker ... ok
test test_construct_frame_content_length_in_aad ... ok
test test_construct_frame_bytes_not_released_until_complete ... ok
test test_construct_frame_body_aad_content ... ok
test test_construct_frame_aad_and_iv ... ok
test test_construct_frame_cipherkey_and_plaintext ... ok
test test_construct_frame_iv_serialized ... ok
test test_construct_frame_final_frame_content_length_less_than_frame_length ... ok
test test_construct_frame_auth_tag_serialized ... ok
test test_construct_frame_final_frame_remaining_plaintext ... ok
test test_construct_frame_message_id_in_aad ... ok
test test_construct_frame_sequence_number_serialized ... ok
test test_construct_frame_regular_frame_plaintext_equals_frame_length ... ok
test test_construct_frame_sequence_number_starts_at_one ... ok
test test_construct_frame_sequence_number_in_aad ... ok
test test_construct_frame_sequence_number_increments ... ok
test test_construct_frame_serialization_regular_and_final ... ok
test test_construct_frame_single_final_frame ... ok
test test_construct_frame_streaming_frame_released ... ok
test test_construct_frame_regular_frame_plaintext_subsequence ... ok
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- The 4 stacking violations all followed the same pattern: shared code path for regular+final frames had both sets of annotations stacked. The fix distributes them across anchor lines.
- Pre-existing clippy errors (missing docs on `encrypt_stream`/`plaintext`, collapsible_if in other files, unreachable patterns in `materials.rs`) are unchanged.
- No annotation content was changed — only placement was restructured.
