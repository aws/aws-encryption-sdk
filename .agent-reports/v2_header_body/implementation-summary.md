## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs` — Added 8 new test functions with 18 `type=test` annotations covering all V2 header body requirements. Added `round_trip_v2` helper for decrypt-based verification tests.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs
```

### Requirements Addressed
- ✅ Req 1: "If the message format version associated with the [algorithm suite]...is 2.0 then the [message header body]...MUST be serialized with the following specifics:" — tested in `test_v2_header_serialized`
- ✅ Req 2: "- [Version]...MUST be serialized according to the [Version]...specification." — tested in `test_v2_header_version`
- ✅ Req 3: "The value MUST correspond to [2.0]..." — tested in `test_v2_header_version`
- ✅ Req 4: "The value of the `Version` field MUST be `02` in the Version 2.0 header body." — tested in `test_v2_header_version`
- ✅ Req 5: "- [Algorithm Suite ID]...MUST be serialized according to the [Algorithm Suite ID]...specification." — tested in `test_v2_header_algorithm_suite_id`
- ✅ Req 6: "The value MUST correspond to the [algorithm suite]...used in this behavior." — tested in `test_v2_header_algorithm_suite_id`
- ✅ Req 7: "- [Message ID]...MUST be serialized according to the [Message ID]...specification." — tested in `test_v2_header_message_id`
- ✅ Req 8: "The process used to generate this identifier MUST use a good source of randomness..." — tested in `test_v2_header_message_id`
- ✅ Req 9: "- [AAD]...MUST be serialized according to the [AAD]...specification." — tested in `test_v2_header_aad`
- ✅ Req 10: "The value MUST be the serialization of the [encryption context]..." — tested in `test_v2_header_aad`
- ✅ Req 11: "- [Encrypted Data Keys]...MUST be serialized according to the [Encrypted Data Keys]...specification." — tested in `test_v2_header_encrypted_data_keys`
- ✅ Req 12: "The value MUST be the serialization of the [encrypted data keys]..." — tested in `test_v2_header_encrypted_data_keys`
- ✅ Req 13: "- [Content Type]...MUST be serialized according to the [Content Type]...specification." — tested in `test_v2_header_content_type`
- ✅ Req 14: "The value MUST be [02]..." — tested in `test_v2_header_content_type`
- ✅ Req 15: "- [Frame Length]...MUST be serialized according to the [Frame Length]...specification." — tested in `test_v2_header_frame_length`
- ✅ Req 16: "The value MUST be the value of the frame size determined above." — tested in `test_v2_header_frame_length`
- ✅ Req 17: "- [Algorithm Suite Data]...MUST be serialized according to the [Algorithm Suite Data]...specification." — tested in `test_v2_header_algorithm_suite_data`
- ✅ Req 18: "The value MUST be the value of the [commit key]..." — tested in `test_v2_header_algorithm_suite_data`

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v2_header_body.rs`
- **Number of `type=test` annotations added**: 18 new for 18 requirements (21 total including 3 pre-existing)
- **Test function names**:
  - `test_v2_header_serialized` (Req 1)
  - `test_v2_header_version` (Req 2, 3, 4)
  - `test_v2_header_algorithm_suite_id` (Req 5, 6)
  - `test_v2_header_message_id` (Req 7, 8 — updated from existing test)
  - `test_v2_header_aad` (Req 9, 10)
  - `test_v2_header_encrypted_data_keys` (Req 11, 12)
  - `test_v2_header_content_type` (Req 13, 14)
  - `test_v2_header_frame_length` (Req 15, 16)
  - `test_v2_header_algorithm_suite_data` (Req 17, 18)

### Proposed Commit Message

```
test(v2-header): add missing type=test annotations for V2 header body requirements

Add 8 new test functions with 18 type=test annotations covering all
V2 header body field serialization requirements from encrypt.md#v2-header
and message-header.md#header-body-version-2-0.

Tests verify: version byte (0x02), algorithm suite ID (0x0578),
message ID (32-byte length + randomness), AAD (round-trip with EC),
encrypted data keys (round-trip), content type (round-trip),
frame length (round-trip), and algorithm suite data/commit key (round-trip).

Spec: aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
Spec: aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-2-0
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 152ms
    Scanning sources
     Scanned 154 sources 17ms
     Parsing annotations
      Parsed 891 annotations 64ms
     Loading specifications
      Loaded 13 specifications 24ms
     Mapping sections
      Mapped 118 sections 15ms
    Matching references
     Matched 1661 references 4ms
     Sorting references
      Sorted 1661 references 14ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 160ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 1ms
```

### Test Results (actual command output)
```
$ cargo test test_v2_header
running 10 tests
test test_v2_header_version ... ok
test test_v2_header_body_serialization_order ... ok
test test_v2_header_serialized ... ok
test test_v2_header_content_type ... ok
test test_v2_header_encrypted_data_keys ... ok
test test_v2_header_frame_length ... ok
test test_v2_header_algorithm_suite_id ... ok
test test_v2_header_message_id ... ok
test test_v2_header_aad ... ok
test test_v2_header_algorithm_suite_data ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

### Notes
- No source file changes needed — all implementation annotations were already present in `v2_header_body.rs`.
- The existing `test_v2_header_message_id` was updated to include the `encrypt.md#v2-header` annotations (Req 7, 8) alongside the pre-existing `data-format/message-header.md#message-id` annotation.
- Pre-existing clippy warnings in `encrypt.rs` and `v2_header_body.rs` are unrelated to this change.
- All 18 requirements in the V2 Header section now show `implementation,test` in the duvet snapshot.
