## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_footer.rs` — Added 4 `type=test` annotations for `message.md#structure` requirements: (1) footer present with signing suite, (2) no footer without signing suite, (3) message begins with header, (4) body follows header. Added 3 new test functions.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_footer.rs
```

### Requirements Addressed
- ✅ `If the [message header]... the message MUST also contain a [message footer]...` — test annotation added to existing `test_footer_present_with_signing_suite`
- ✅ `If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer.` — new test `test_no_footer_without_signing_suite`
- ✅ `- The message MUST begin with [Message Header](message-header.md)` — new test `test_message_begins_with_header`
- ✅ `- The [Message Body](message-body.md) MUST follow the Message Header` — new test `test_message_body_follows_header`
- ⚠️ `If the algorithm suite contain an unrecognized signature algorithm, the operation MUST raise an error.` — DEFERRED: untestable from public API (Rust enum exhaustiveness makes the `_` arm unreachable). See `.agent-reports/footer/deferred-annotations.md`.

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_footer.rs`
- **Number of `type=test` annotations added**: 4 for 4 requirements
- **Test function names**:
  - `test_footer_present_with_signing_suite` (added second annotation block for `message.md#structure`)
  - `test_no_footer_without_signing_suite` (new)
  - `test_message_begins_with_header` (new)
  - `test_message_body_follows_header` (new)

### Proposed Commit Message

```
test(footer): add duvet test annotations for message.md#structure requirements

Add type=test annotations for 4 of 5 normative requirements in
data-format/message.md#structure:
- Message MUST begin with Message Header (version byte check)
- Message Body MUST follow the Message Header (round-trip decrypt)
- Signing suite → message MUST contain footer (existing test, new annotation)
- Non-signing suite → message MUST NOT contain footer (round-trip decrypt)

The 5th requirement (unrecognized signature algorithm MUST raise error)
is deferred: the Rust SignatureAlgorithm enum is exhaustive, making the
wildcard match arm unreachable from the public API.

Refs: specification/data-format/message.md#structure
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 39ms
    Scanning sources
     Scanned 157 sources 2ms
     Parsing annotations
      Parsed 965 annotations 36ms
     Loading specifications
      Loaded 14 specifications 16ms
     Mapping sections
      Mapped 123 sections 15ms
    Matching references
     Matched 1803 references 4ms
     Sorting references
      Sorted 1803 references 11ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 18ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 1ms
```

Duvet snapshot for `message.md#structure`:
```
  SECTION: [Structure](#structure)
    TEXT[!MUST,implementation,test]: - The message MUST begin with [Message Header](message-header.md)
    TEXT[!MUST,implementation,test]: - The [Message Body](message-body.md) MUST follow the Message Header
    TEXT[!MUST,implementation,test]: If the [message header]... the message MUST also contain a [message footer]...
    TEXT[!MUST,implementation,test]: If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer.
    TEXT[!MUST,implementation]: If the algorithm suite contain an unrecognized signature algorithm, the operation MUST raise an error.
```

4 of 5 requirements show `implementation,test`. The 5th has `implementation` only (deferred — untestable).

### Test Results (actual command output)
```
$ cargo test --test test_footer
running 7 tests
test test_footer_signature_length_is_uint16 ... ok
test test_no_footer_without_signing_suite ... ok
test test_footer_signature_length_is_two_bytes ... ok
test test_message_begins_with_header ... ok
test test_message_body_follows_header ... ok
test test_footer_signature_calculated_over_header_and_body ... ok
test test_footer_present_with_signing_suite ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

### Notes
- Requirement 5 ("unrecognized signature algorithm MUST raise an error") is deferred because `SignatureAlgorithm` is a Rust enum with only `Ecdsa(...)` and `None` variants. The `_ =>` wildcard arm in `step_construct_signature` is unreachable from the public API. No test can exercise this path without bypassing the type system. Details in `.agent-reports/footer/deferred-annotations.md`.
- All clippy warnings are pre-existing (in `encrypt.rs`, `materials.rs`, `v1_header_body.rs`, `v2_header_body.rs`) — none in `test_footer.rs`.
- Requirements 4 and 5 (header/body ordering) were placed in `test_footer.rs` rather than a separate file because the test helpers already exist there and the requirements are part of the same `message.md#structure` section.
