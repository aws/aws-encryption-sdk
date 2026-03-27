## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs` — Added 21 `encrypt.md#v1-header` implementation annotations and 1 `data-format/message-header.md#header-body-version-1-0` serialization order annotation to `write_v1_header_body`. Kept all pre-existing data-format annotations.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs
```

### Requirements Addressed
- ✅ "If the message format version associated with the [algorithm suite]... is 1.0 then the [message header body]... MUST be serialized with the following specifics:" — implemented + tested
- ✅ "The serialization order MUST follow the [Header Body Version 1.0]... specification." — implemented (implication) + tested
- ✅ "The V1 Header Body MUST be serialized as, in order, Version, Type, Algorithm Suite ID, Message ID, AAD, Encrypted Data Keys, Content Type, Reserved, IV Length, and Frame Length." — implemented (implication) + tested
- ✅ "- [Version]...: MUST be serialized according to the [Version]... specification." — implemented + tested
- ✅ "The value MUST correspond to [1.0]..." — implemented + tested
- ✅ "- [Type]...: MUST be serialized according to the [Type]... specification." — implemented + tested
- ✅ "The value MUST correspond to [Customer Authenticated Encrypted Data]..." — implemented + tested
- ✅ "- [Algorithm Suite ID]...: MUST be serialized according to the [Algorithm Suite ID]... specification." — implemented + tested
- ✅ "The value MUST correspond to the [algorithm suite]... used in this behavior." — implemented + tested
- ✅ "- [Message ID]...: MUST be serialized according to the [Message ID]... specification." — implemented + tested
- ✅ "The process used to generate this identifier MUST use a good source of randomness to make the chance of duplicate identifiers negligible." — implemented + tested
- ✅ "- [AAD]...: MUST be serialized according to the [AAD]... specification." — implemented + tested
- ✅ "The value MUST be the serialization of the [encryption context]... and this serialization MUST NOT contain any key value pairs listed in the [encryption material's]... [required encryption context keys]..." — implemented + tested
- ✅ "- [Encrypted Data Keys]...: MUST be serialized according to the [Encrypted Data Keys]... specification." — implemented + tested
- ✅ "The value MUST be the serialization of the [encrypted data keys]... in the [encryption materials]..." — implemented + tested
- ✅ "- [Content Type]...: MUST be serialized according to the [Content Type]... specification." — implemented + tested
- ✅ "The value MUST be [02]..." — implemented + tested
- ✅ "- [Reserved]...: MUST be serialized according to the [Reserved]... specification." — implemented + tested
- ✅ "- [IV Length]...: MUST be serialized according to the [IV Length]... specification." — implemented + tested
- ✅ "The value MUST match the [IV length]... specified by the [algorithm suite]..." — implemented + tested
- ✅ "- [Frame Length]...: MUST be serialized according to the [Frame Length]... specification." — implemented + tested
- ✅ "The value MUST be the value of the frame size determined above." — implemented + tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: No test file changes needed — `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs` already contains all 21 `type=test` annotations.
- **Number of `type=test` annotations added**: 0 (all 24 already existed)
- **Test function names**: `test_v1_header_serialized`, `test_v1_header_version`, `test_v1_header_type`, `test_v1_header_algorithm_suite_id`, `test_v1_header_message_id`, `test_v1_header_aad`, `test_v1_header_encrypted_data_keys`, `test_v1_header_content_type`, `test_v1_header_reserved`, `test_v1_header_iv_length`, `test_v1_header_frame_length`, `test_v1_header_serialization_order`

### Proposed Commit Message

```
feat(v1-header): add encrypt.md#v1-header implementation annotations

Add all 21 missing duvet implementation annotations for the
encrypt.md#v1-header specification section to write_v1_header_body
in v1_header_body.rs, mirroring the existing v2_header_body.rs pattern.

Also adds the data-format/message-header.md#header-body-version-1-0
serialization order annotation as type=implication.

All existing data-format annotations are preserved. Test annotations
already existed in tests/test_v1_header_body.rs.

Spec: aws-encryption-sdk-specification/client-apis/encrypt.md#v1-header
Spec: aws-encryption-sdk-specification/data-format/message-header.md#header-body-version-1-0
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 31ms
    Scanning sources
     Scanned 153 sources 2ms
     Parsing annotations
      Parsed 841 annotations 49ms
     Loading specifications
      Loaded 12 specifications 15ms
     Mapping sections
      Mapped 114 sections 12ms
    Matching references
     Matched 1563 references 4ms
     Sorting references
      Sorted 1563 references 10ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 18ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 1ms
```

### Test Results (actual command output)
```
$ cargo test --test test_v1_header_body
running 12 tests
test test_v1_header_type ... ok
test test_v1_header_message_id ... ok
test test_v1_header_version ... ok
test test_v1_header_serialized ... ok
test test_v1_header_iv_length ... ok
test test_v1_header_encrypted_data_keys ... ok
test test_v1_header_frame_length ... ok
test test_v1_header_algorithm_suite_id ... ok
test test_v1_header_content_type ... ok
test test_v1_header_reserved ... ok
test test_v1_header_serialization_order ... ok
test test_v1_header_aad ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- This is annotation-only work. No code logic was changed.
- The v2_header_body.rs pattern was followed exactly for annotation placement.
- Pre-existing data-format annotations were preserved: `message-header.md#reserved` (on const), `message-header.md#message-id` (on write_message_id), `message-header.md#iv-length` (on write_u8), and `message-header.md#header-body-version-1-0` (on write_msg_format_version).
- Pre-existing clippy warnings (collapsible if in read_v1_header_body, missing docs on encrypt_stream) are unrelated to this change.
