## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs` — Fixed 8 wrong-prefix annotations (`aws-encryption-sdk-specification/` → `specification/`), added 2 new implication annotations (B1: final-frame-iv uniqueness, B2: final-frame-encrypted-content bytes), and added sentinel line `let _enc_content_is_bytes = &enc_content;` after B2 to break the 3-stack before `read_bytes`
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_message_body_format.rs` — Fixed 33 wrong-prefix test annotations (`aws-encryption-sdk-specification/` → `specification/`)
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs` — Fixed 7 wrong-prefix test annotations (`aws-encryption-sdk-specification/` → `specification/`)

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_message_body_format.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs
```

### Requirements Addressed

#### Group A: Wrong-Prefix Implementation Annotations in body.rs (8 fixed)
- ✅ A1: `The sequence number MUST be serialized as a UInt32.` — prefix fixed
- ✅ A2: `Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.` — prefix fixed
- ✅ A3: `The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.` — prefix fixed
- ✅ A4: `The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.` — prefix fixed
- ✅ A5: `The Final Frame Sequence Number MUST be serialized to a message the same way as the [Regular Frame Sequence Number](#regular-frame-sequence-number).` — prefix fixed
- ✅ A6: `The Final Frame Sequence Number MUST be interpreted from a message the same way as the [Regular Frame Sequence Number](#regular-frame-sequence-number).` — prefix fixed
- ✅ A7: `The encrypted content length MUST be serialized as a UInt32.` — prefix fixed
- ✅ A8: `The value MUST be encoded as the 4 bytes FF FF FF FF in hexadecimal notation.` — prefix fixed

#### Group B: Missing Implementation Annotations (2 added)
- ✅ B1: `The IV MUST be a unique IV within the message.` — added at `_iv_is_unique` sentinel in `construct_frame` (final-frame-iv)
- ✅ B2: `The encrypted content MUST be interpreted as bytes.` — added after `read_seq_u32_bounded` in final-frame branch of `read_and_decrypt_framed_message_body` (final-frame-encrypted-content), with sentinel line `let _enc_content_is_bytes = &enc_content;` to separate from auth_tag annotations

#### Group C: Wrong-Prefix Test Annotations (40 fixed)
- ✅ 33 annotations fixed in `test_message_body_format.rs`
- ✅ 7 annotations fixed in `test_construct_the_body.rs`

### Review Feedback Addressed (Cycle 3, Round 1)
- ✅ **B2 3-stack fix**: The B2 annotation (`final-frame-encrypted-content` / "encrypted content MUST be interpreted as bytes") is followed by sentinel line `let _enc_content_is_bytes = &enc_content;` which separates it from the 2 auth_tag annotations before `read_bytes(r, &mut auth_tag, raw)?;`. The auth_tag stack is now 2 annotations (within limit). The sentinel references `enc_content`, matching B2's subject semantically.

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `tests/test_message_body_format.rs`, `tests/test_construct_the_body.rs`
- **Number of `type=test` annotations fixed**: 40 (33 + 7) prefix corrections
- **No new test annotations needed** — existing tests already cover all requirements; they just had the wrong prefix

### Proposed Commit Message

```
fix(message-body): fix 48 annotation prefixes, add 2 annotations, fix B2 stacking

Fix 48 duvet annotations across body.rs, test_message_body_format.rs,
and test_construct_the_body.rs that used the wrong prefix
`aws-encryption-sdk-specification/` instead of `specification/` for
message-body.md targets.

Add 2 new implication annotations:
- final-frame-iv: "The IV MUST be a unique IV within the message"
- final-frame-encrypted-content: "The encrypted content MUST be interpreted as bytes"

Add sentinel line after B2 annotation to prevent 3-stack with
auth_tag annotations before read_bytes call.

Refs: specification/data-format/message-body.md
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 29ms
    Scanning sources
     Scanned 164 sources 1ms
     Parsing annotations
      Parsed 1267 annotations 25ms
     Loading specifications
      Loaded 13 specifications 17ms
     Mapping sections
      Mapped 141 sections 16ms
    Matching references
     Matched 2258 references 2ms
     Sorting references
      Sorted 2258 references 16ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 21ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 990µs
```

### Test Results (actual command output)
```
$ cargo test --test test_message_body_format
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --test test_construct_the_body
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- All 8 pre-existing clippy errors are in other files (encrypt.rs, v1_header_body.rs, v2_header_body.rs, decrypt.rs) — none in body.rs or the test files
- The `aws-encryption-sdk-specification/` prefix remains on `client-apis/encrypt.md` annotations in body.rs — those are intentionally different (different spec, different duvet config prefix)
- No `!MUST` entries in the duvet snapshot for the `specification/data-format/message-body.md` specification
