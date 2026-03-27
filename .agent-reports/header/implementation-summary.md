## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs` — Added 3 new test functions with `type=test` annotations, plus `encrypt_default()` and `parse_header_offsets()` helpers

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs
```

### Requirements Addressed
- ✅ "This value MUST be greater than 0." (`#encrypted-data-key-count`) — tested via byte inspection of EDK count field
- ✅ "The length of the suite data field MUST be equal to the Algorithm Suite Data Length value of the algorithm suite specified by the Algorithm Suite ID field." (`#algorithm-suite-data`) — tested via V2 round-trip (validate_suite_data runs during decrypt)
- ✅ "When the content type is non-framed, the value of this field MUST be 0." (`#frame-length`) — tested via ciphertext mutation (set NonFramed + non-zero frame length, assert decrypt fails)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs`
- **Number of `type=test` annotations added**: 3 for 3 requirements
- **Test function names**:
  - `test_encrypted_data_key_count_greater_than_zero`
  - `test_suite_data_length_matches_algorithm_suite`
  - `test_nonframed_frame_length_must_be_zero`

### Proposed Commit Message

```
test(message-header): add missing test annotations for EDK count, suite data length, and frame length

Add type=test annotations for three message-header requirements that had
implementation annotations but no corresponding test coverage:
- encrypted-data-key-count: "This value MUST be greater than 0"
- algorithm-suite-data: suite data length must match algorithm suite
- frame-length: "When content type is non-framed, value MUST be 0"

Spec: aws-encryption-sdk-specification/data-format/message-header.md
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 29ms
    Scanning sources
     Scanned 153 sources 2ms
     Parsing annotations
      Parsed 844 annotations 29ms
     Loading specifications
      Loaded 12 specifications 14ms
     Mapping sections
      Mapped 114 sections 13ms
    Matching references
     Matched 1567 references 4ms
     Sorting references
      Sorted 1567 references 10ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 14ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 855µs
```

### Test Results (actual command output)
```
$ cargo test --test test_header_structure
running 5 tests
test test_encrypted_data_key_count_greater_than_zero ... ok
test test_nonframed_frame_length_must_be_zero ... ok
test test_header_serialization_order ... ok
test test_header_big_endian_format ... ok
test test_suite_data_length_matches_algorithm_suite ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### Notes
- All three requirements now show `[!MUST,implementation,test]` in the duvet snapshot
- The `validate_max_encrypted_data_keys` function has a pre-existing explicit `//= type=implementation` that should be omitted per duvet patterns (non-blocking, noted in Round 1 review)
- Pre-existing clippy warnings (missing docs, unreachable patterns) are unrelated to this change
