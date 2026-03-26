## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs` — Fixed review issues: (1) moved NonFramed `type=test` annotation to new dedicated `test_content_type_nonframed_value` test that actually exercises NonFramed=0x01, (2) changed invalid-value test annotation from `supported-content-types` NonFramed sub-item to `content-type` section quote, (3) removed all blank lines between annotation blocks and between annotations and code

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs
```

### Requirements Addressed
- ✅ `The supported content types MUST be:` — implemented (source, implication) + tested (test_content_type_framed_value)
- ✅ `- \`02\` for [Framed](message-body.md#framed-data)` — implemented (source, implication) + tested (test_content_type_framed_value)
- ✅ `- \`01\` for [Non-Framed](message-body.md#non-framed-data)` — implemented (source, implication) + tested (test_content_type_nonframed_value)
- ✅ `The value (hex) of this field MUST be a value that exists in the following table:` — implemented (source, read_content_type) + tested (test_content_type_invalid_value_rejected)

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_types.rs`
- **Number of `type=test` annotations added**: 4 for 4 requirements (3 supported-content-types + 1 content-type)
- **Test function names**:
  - `test_content_type_framed_value` — 2 annotations: parent MUST + Framed sub-item
  - `test_content_type_nonframed_value` — 1 annotation: NonFramed sub-item
  - `test_content_type_invalid_value_rejected` — 1 annotation: content-type MUST (invalid value rejection)

### Review Feedback Addressed
1. **NonFramed annotation misplacement (Critical)**: Created new `test_content_type_nonframed_value` test that sets content type byte to 0x01 and verifies it is NOT rejected as "Unsupported Content Type" — proving 0x01 is a supported content type. Removed the NonFramed annotation from the invalid-value test.
2. **Invalid-value test annotation (Critical)**: Changed annotation from `supported-content-types` NonFramed sub-item to `content-type` section quote (`The value (hex) of this field MUST be a value that exists in the following table:`), which correctly describes what the test verifies.
3. **Blank lines between annotations (Placement)**: Removed all blank lines between annotation blocks and between annotations and first executable line in all three test functions.

### Proposed Commit Message

```
fix(message-header): address review feedback for content-type test annotations

Move NonFramed type=test annotation to dedicated test that exercises
NonFramed=0x01 by verifying it's accepted as a valid content type.
Change invalid-value test annotation to content-type section quote.
Remove blank lines between annotation blocks per placement rules.

Spec: specification/data-format/message-header.md#supported-content-types
Spec: specification/data-format/message-header.md#content-type
```

### Duvet Verification (actual command output)
```
$ make duvet
[duvet extract and report runs successfully for all specs]
[2 pre-existing errors in compliance_exceptions/encrypt.txt — not related to this change]
```

### Test Results (actual command output)
```
$ cargo test --test test_header_types
running 3 tests
test test_content_type_nonframed_value ... ok
test test_content_type_framed_value ... ok
test test_content_type_invalid_value_rejected ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- The duvet report has 2 pre-existing errors in `compliance_exceptions/encrypt.txt` for the `construct-a-frame` section — these are unrelated to this work item.
- The `test_content_type_nonframed_value` test proves 0x01 is a supported content type by showing the error is NOT "Unsupported Content Type" when the byte is set to 0x01. The actual error is a frame-length mismatch ("Frame length must be zero if content is non-framed") because the message body was encrypted as framed.
- Pre-existing clippy warnings exist in encrypt.rs, materials.rs, v1_header_body.rs, v2_header_body.rs — none from our modified file.
