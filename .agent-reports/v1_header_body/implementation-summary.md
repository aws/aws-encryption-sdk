## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs` — Added 21 `encrypt.md#v1-header` annotations and 1 `message-header.md#header-body-version-1-0` serialization order annotation to `write_v1_header_body`. Preserved existing data-format annotations.
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs` — NEW FILE: 12 tests with 23 `type=test` annotations covering all 22 requirements.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs
```

### Requirements Addressed
- ✅ Req 1 (parent): "If the message format version...MUST be serialized with the following specifics" — implemented + tested
- ✅ Req 2: "Version MUST be serialized" — implemented + tested
- ✅ Req 3: "value MUST correspond to 1.0" — implemented + tested
- ✅ Req 4: "Type MUST be serialized" — implemented + tested
- ✅ Req 5: "value MUST correspond to Customer Authenticated Encrypted Data" — implemented + tested
- ✅ Req 6: "Algorithm Suite ID MUST be serialized" — implemented + tested
- ✅ Req 7: "value MUST correspond to the algorithm suite" — implemented + tested
- ✅ Req 8: "Message ID MUST be serialized" — implemented + tested
- ✅ Req 9: "MUST use a good source of randomness" — implemented + tested
- ✅ Req 10: "AAD MUST be serialized" — implemented + tested
- ✅ Req 11: "value MUST be the serialization of the encryption context...MUST NOT contain required EC keys" — implemented + tested
- ✅ Req 12: "Encrypted Data Keys MUST be serialized" — implemented + tested
- ✅ Req 13: "value MUST be the serialization of the encrypted data keys" — implemented + tested
- ✅ Req 14: "Content Type MUST be serialized" — implemented + tested
- ✅ Req 15: "value MUST be 02" — implemented + tested
- ✅ Req 16: "Reserved MUST be serialized" — implemented + tested
- ✅ Req 17: "IV Length MUST be serialized" — implemented + tested
- ✅ Req 18: "value MUST match the IV length specified by the algorithm suite" — implemented + tested
- ✅ Req 19: "Frame Length MUST be serialized" — implemented + tested
- ✅ Req 20: "value MUST be the value of the frame size determined above" — implemented + tested
- ✅ Req 21: "serialization order MUST follow Header Body Version 1.0" — implemented + tested
- ✅ Req 22: "V1 Header Body MUST be serialized as, in order..." — implemented + tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs`
- **Number of `type=test` annotations added**: 23 for 22 requirements
- **Test function names**:
  - `test_v1_header_serialized` — parent requirement
  - `test_v1_header_version` — Version field + value + data-format version
  - `test_v1_header_type` — Type field + value
  - `test_v1_header_algorithm_suite_id` — Algorithm Suite ID + value
  - `test_v1_header_message_id` — Message ID + randomness
  - `test_v1_header_aad` — AAD + EC value
  - `test_v1_header_encrypted_data_keys` — EDKs + value
  - `test_v1_header_content_type` — Content Type + value
  - `test_v1_header_reserved` — Reserved field
  - `test_v1_header_iv_length` — IV Length + value
  - `test_v1_header_frame_length` — Frame Length + value
  - `test_v1_header_serialization_order` — serialization order (encrypt.md + data-format)

### Proposed Commit Message

```
feat(v1-header): add duvet annotations for encrypt.md#v1-header

Add 21 specification annotations from encrypt.md#v1-header and 1 from
message-header.md#header-body-version-1-0 to write_v1_header_body,
matching the pattern already established in v2_header_body.rs.

Create test_v1_header_body.rs with 12 tests and 23 type=test annotations
covering all requirements. Tests use a non-committing algorithm suite
(AlgAes256GcmIv12Tag16HkdfSha256) to exercise the V1 header path.

Spec: specification/client-apis/encrypt.md#v1-header
Spec: specification/data-format/message-header.md#header-body-version-1-0
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 40ms
    Scanning sources
     Scanned 146 sources 4ms
     Parsing annotations
      Parsed 803 annotations 28ms
     Loading specifications
      Loaded 13 specifications 16ms
     Mapping sections
      Mapped 112 sections 11ms
    Matching references
     Matched 1511 references 4ms
     Sorting references
      Sorted 1511 references 10ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 16ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 756µs
```

All v1-header requirements show `[!MUST,implementation,test]` in the snapshot.
All header-body-version-1-0 requirements show `[!MUST,implementation,test]` in the snapshot.

### Test Results (actual command output)
```
$ cargo test --test test_v1_header_body
running 12 tests
test test_v1_header_version ... ok
test test_v1_header_type ... ok
test test_v1_header_reserved ... ok
test test_v1_header_iv_length ... ok
test test_v1_header_algorithm_suite_id ... ok
test test_v1_header_serialization_order ... ok
test test_v1_header_content_type ... ok
test test_v1_header_message_id ... ok
test test_v1_header_frame_length ... ok
test test_v1_header_serialized ... ok
test test_v1_header_aad ... ok
test test_v1_header_encrypted_data_keys ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### Notes
- No logic changes were made — this is annotation-only work on the source file.
- Tests use `AlgAes256GcmIv12Tag16HkdfSha256` with `ForbidEncryptAllowDecrypt` commitment policy to force V1 header generation.
- Tests use a raw AES keyring (no KMS needed) for fast, offline execution.
- Pre-existing clippy warnings exist in the codebase (missing docs, collapsible if, etc.) — none introduced by this change.
- Pre-existing test failures in `test_authentication_tag` are due to expired AWS tokens, unrelated to this change.
