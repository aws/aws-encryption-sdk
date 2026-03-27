# Work Item: Fix Spec Path Prefix Mismatches and Add Missing Annotations for message-body.md

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-body.md`
- **Section**: `## Structure` (all sub-sections: Non-Framed Data, Framed Data, Regular Frame, Final Frame)
- **Duvet Target**: `specification/data-format/message-body.md#*` (multiple sections)

## Type of Work
FIX_ANNOTATION

## Problem Summary

The local duvet config (`esdk/.duvet/config.toml`) uses `specification/` as the spec source prefix.
However, 8 implementation annotations in `body.rs` and ALL 40 test annotations in
`test_message_body_format.rs` (33) and `test_construct_the_body.rs` (7) use the wrong prefix
`aws-encryption-sdk-specification/` instead of `specification/`.

This means the local duvet report sees implementation/implication annotations but does NOT see
test annotations for any `specification/` prefix requirements.

Additionally, 2 requirements have NO annotation at all (not even wrong-prefix).

## Requirements to Address

### Group A: Wrong-Prefix Implementation Annotations in body.rs (8 annotations)

These annotations exist but use `aws-encryption-sdk-specification/` instead of `specification/`.
Fix: change the prefix.

#### A1. Regular Frame Sequence Number — serialized as UInt32
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The sequence number MUST be serialized as a UInt32.
  ```
- **Current State**: wrong-prefix (line 516 of body.rs)
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

#### A2. Regular Frame IV — unique within message
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
  ```
- **Current State**: wrong-prefix (line 478 of body.rs)
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

#### A3. Regular Frame Encrypted Content — length equals frame length
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
  ```
- **Current State**: wrong-prefix (line 723 of body.rs)
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

#### A4. Final Frame Sequence Number — equals total frames
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
  ```
- **Current State**: wrong-prefix (line 787 of body.rs)
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

#### A5. Final Frame Sequence Number — serialized same as regular
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Final Frame Sequence Number MUST be serialized to a message the same way as the
  [Regular Frame Sequence Number](#regular-frame-sequence-number).
  ```
- **Current State**: wrong-prefix (line 521 of body.rs)
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

#### A6. Final Frame Sequence Number — interpreted same as regular
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The Final Frame Sequence Number MUST be interpreted from a message the same way as the
  [Regular Frame Sequence Number](#regular-frame-sequence-number).
  ```
- **Current State**: wrong-prefix (line 129 of body.rs)
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

#### A7. Final Frame Encrypted Content Length — serialized as UInt32
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The encrypted content length MUST be serialized as a UInt32.
  ```
- **Current State**: wrong-prefix (line 551 of body.rs)
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

#### A8. Sequence Number End — value encoded as FF FF FF FF
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
  ```
- **Current State**: wrong-prefix (line 505 of body.rs); also has correct-prefix annotation in header.rs line 12
- **Fix**: Change `aws-encryption-sdk-specification/` to `specification/`

### Group B: Missing Implementation Annotations (2 annotations)

These requirements have NO annotation at all in any file.

#### B1. Final Frame IV — unique within message
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The IV MUST be a unique IV within the message.
  ```
- **Current State**: missing
- **Placement**: In `construct_frame()` in body.rs, near the existing `#regular-frame-iv` uniqueness annotation (line 478). The `construct_frame` function is called for both regular and final frames, so the final frame IV uniqueness is fulfilled by the same `iv_seq` call. Add a `specification/data-format/message-body.md#final-frame-iv` implication annotation alongside the existing `#regular-frame-iv` one.

#### B2. Final Frame Encrypted Content — interpreted as bytes
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  The encrypted content MUST be interpreted as bytes.
  ```
- **Current State**: missing
- **Placement**: In `read_and_decrypt_framed_message_body()` in body.rs, near the `read_seq_u32_bounded` call for the final frame (around line 163). The encrypted content is read as bytes via `read_seq_u32_bounded` which returns `Vec<u8>`. Add a `specification/data-format/message-body.md#final-frame-encrypted-content` implication annotation.

### Group C: Wrong-Prefix Test Annotations (40 annotations)

ALL test annotations in `test_message_body_format.rs` (33 annotations) and `test_construct_the_body.rs` (7 annotations) use `aws-encryption-sdk-specification/` instead of `specification/`.

Fix: Change ALL occurrences of `aws-encryption-sdk-specification/data-format/message-body.md` to `specification/data-format/message-body.md` in both test files.

**test_message_body_format.rs** — 33 annotations covering:
- `#framed-data` (2 tests)
- `#regular-frame` (1 test)
- `#regular-frame-sequence-number` (5 tests)
- `#regular-frame-iv` (3 tests)
- `#regular-frame-encrypted-content` (2 tests)
- `#regular-frame-authentication-tag` (2 tests)
- `#final-frame` (2 tests)
- `#sequence-number-end` (3 tests)
- `#final-frame-sequence-number` (3 tests)
- `#final-frame-iv` (3 tests)
- `#final-frame-encrypted-content-length` (3 tests)
- `#final-frame-encrypted-content` (2 tests)
- `#final-frame-authentication-tag` (2 tests)

**test_construct_the_body.rs** — 7 annotations covering:
- `#final-frame` (7 tests: Framed data MUST contain exactly one final frame, The final frame MUST be the last frame, plaintext length constraints, SHOULD equal frame length, less than frame length)

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`

Wrong-prefix example (line 516):
```rust
    //= aws-encryption-sdk-specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=implication
    //= reason=write_u32 serializes as a 4-byte big-endian UInt32
    //# The sequence number MUST be serialized as a UInt32.
    write_u32(w, input.sequence_number)?;
```

Should be:
```rust
    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=implication
    //= reason=write_u32 serializes as a 4-byte big-endian UInt32
    //# The sequence number MUST be serialized as a UInt32.
    write_u32(w, input.sequence_number)?;
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_message_body_format.rs`

Wrong-prefix example (line 125):
```rust
    //= aws-encryption-sdk-specification/data-format/message-body.md#framed-data
    //= type=test
    //# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
```

Should be:
```rust
    //= specification/data-format/message-body.md#framed-data
    //= type=test
    //# - The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`.
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_the_body.rs`

Wrong-prefix example (line 150):
```rust
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame
    //= type=test
    //# Framed data MUST contain exactly one final frame.
```

Should be:
```rust
    //= specification/data-format/message-body.md#final-frame
    //= type=test
    //# Framed data MUST contain exactly one final frame.
```

## Implementation Guidance

- This is a **prefix replacement** task. No new test logic or implementation code is needed.
- For Group A: In `body.rs`, find-and-replace `//= aws-encryption-sdk-specification/data-format/message-body.md#` with `//= specification/data-format/message-body.md#` at the 8 specific lines listed above. Do NOT change annotations that reference `aws-encryption-sdk-specification/client-apis/encrypt.md` — those are a different spec and may use a different prefix intentionally.
- For Group B: Add 2 new implication annotations at the specified locations.
- For Group C: In `test_message_body_format.rs` and `test_construct_the_body.rs`, find-and-replace `//= aws-encryption-sdk-specification/data-format/message-body.md#` with `//= specification/data-format/message-body.md#` for ALL occurrences.
- Follow the existing annotation patterns in body.rs (see the `specification/` prefix annotations already present).

### Spec-Aligned Structure

The fix is purely mechanical — no structural changes needed:
1. Fix 8 wrong-prefix annotations in `body.rs` → changes `aws-encryption-sdk-specification/` to `specification/`
2. Add 2 missing implication annotations in `body.rs`
3. Fix 33 wrong-prefix test annotations in `test_message_body_format.rs`
4. Fix 7 wrong-prefix test annotations in `test_construct_the_body.rs`

### Missing Annotation Placement

For B1 (Final Frame IV unique), add near line 478 of body.rs:
```rust
    //= specification/data-format/message-body.md#final-frame-iv
    //= type=implication
    //= reason=Each frame's IV is derived from its unique sequence number via iv_seq
    //# The IV MUST be a unique IV within the message.
```

For B2 (Final Frame Encrypted Content bytes), add near line 163 of body.rs after the `read_seq_u32_bounded` call:
```rust
    //= specification/data-format/message-body.md#final-frame-encrypted-content
    //= type=implication
    //= reason=read_seq_u32_bounded returns Vec<u8>
    //# The encrypted content MUST be interpreted as bytes.
```

## Targeted Tests

No new tests needed. Existing tests already cover all requirements — they just use the wrong prefix.

Tests in `test_message_body_format.rs`:
- `test_framed_data_max_frame_size`
- `test_framed_data_max_frame_count`
- `test_regular_frame_serialization_order`
- `test_regular_frame_sequence_number_starts_at_one`
- `test_regular_frame_sequence_number_increments`
- `test_regular_frame_sequence_number_4_bytes`
- `test_regular_frame_sequence_number_uint32`
- `test_regular_frame_sequence_number_read_as_uint32`
- `test_regular_frame_iv_unique`
- `test_regular_frame_iv_length_matches_algorithm`
- `test_regular_frame_iv_interpreted_as_bytes`
- `test_regular_frame_encrypted_content_length_equals_frame_length`
- `test_regular_frame_encrypted_content_interpreted_as_bytes`
- `test_regular_frame_auth_tag_length_matches_algorithm`
- `test_regular_frame_auth_tag_interpreted_as_bytes`
- `test_final_frame_serialization_order`
- `test_final_frame_is_regular_frame_plus_additions`
- `test_sequence_number_end_value`
- `test_sequence_number_end_4_bytes`
- `test_sequence_number_end_interpreted_as_bytes`
- `test_final_frame_sequence_number_equals_total_frames`
- `test_final_frame_sequence_number_serialized_same_as_regular`
- `test_final_frame_sequence_number_interpreted_same_as_regular`
- `test_final_frame_iv_unique`
- `test_final_frame_iv_length_matches_algorithm`
- `test_final_frame_iv_interpreted_as_bytes`
- `test_final_frame_encrypted_content_length_4_bytes`
- `test_final_frame_encrypted_content_length_uint32`
- `test_final_frame_encrypted_content_length_read_as_uint32`
- `test_final_frame_encrypted_content_length_matches`
- `test_final_frame_encrypted_content_interpreted_as_bytes`
- `test_final_frame_auth_tag_length_matches_algorithm`
- `test_final_frame_auth_tag_interpreted_as_bytes`

Tests in `test_construct_the_body.rs`:
- `test_regular_frame_serialization_conforms_to_spec`
- `test_end_of_input_processing`
- `test_exact_frame_length_constructs_final_or_regular`
- `test_not_enough_bytes_constructs_final_frame`

## Success Criteria
```bash
cargo test --test test_message_body_format
cargo test --test test_construct_the_body
make duvet  # from esdk/ directory
```
- [ ] All tests pass (no test logic changes, only annotation prefix changes)
- [ ] duvet snapshot shows NO `TEXT[!MUST]` entries for `specification/data-format/message-body.md` sections
- [ ] All requirements have `implication`, `implementation`, or `exception` annotations
- [ ] All implementations have corresponding `type=test` annotations
- [ ] The 2 new implication annotations (B1, B2) appear in the snapshot
