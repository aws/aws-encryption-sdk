# Work Item: Add Missing Duvet Annotations for Final Frame Field Deserialization in Decrypt Body

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/decrypt.md`
- **Section**: `decrypt-the-message-body`
- **Duvet Target**: `specification/client-apis/decrypt.md#decrypt-the-message-body`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  For a regular frame, each field MUST be deserialized according to its specification:
  ```
- **Current State**: missing
- **Placement**: `src/message/body.rs` — inside `read_and_decrypt_framed_message_body`, at the start of the regular frame deserialization block (after the `if expected_frame != START_SEQUENCE_NUMBER` write block, before `expected_frame += 1;`, around line 322). This is a parent/summary annotation for the regular frame field sub-items that follow.

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  For a final frame, each field MUST be deserialized according to its specification:
  ```
- **Current State**: missing
- **Placement**: `src/message/body.rs` — inside the `if seq_num == ENDFRAME_SEQUENCE_NUMBER` block, just after the ENDFRAME check (around line 170), before the final frame sequence number read. This is a parent/summary annotation for the final frame field sub-items that follow.

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be deserialized according to the
  [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
  ```
- **Current State**: missing
- **Placement**: `src/message/body.rs` — immediately before `let seq_num: u32 = read_u32(ciphertext, sig_digest)?;` inside the ENDFRAME block (around line 175). The existing `specification/data-format/message-body.md#final-frame-sequence-number` annotation is there but this is a different spec target (`client-apis/decrypt.md#decrypt-the-message-body`).

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [IV](../data-format/message-body.md#final-frame-iv): MUST be deserialized according to the
  [Final Frame IV](../data-format/message-body.md#final-frame-iv) specification.
  ```
- **Current State**: missing
- **Placement**: `src/message/body.rs` — immediately before `read_bytes(ciphertext, &mut iv, sig_digest)?;` inside the ENDFRAME block (around line 183). The existing `specification/data-format/message-body.md#final-frame-iv` annotations are there but this is a different spec target.

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content): MUST be deserialized according to the
  [Final Frame Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content) specification.
  ```
- **Current State**: missing
- **Placement**: `src/message/body.rs` — immediately before `let _enc_content_is_bytes = &enc_content;` inside the ENDFRAME block (around line 205). The encrypted content has already been read by `read_seq_u32_bounded` above; annotate at the point where the content is available.

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag): MUST be deserialized according to the
  [Final Frame Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag) specification.
  ```
- **Current State**: missing
- **Placement**: `src/message/body.rs` — immediately before `read_bytes(ciphertext, &mut auth_tag, sig_digest)?;` inside the ENDFRAME block (around line 210). The existing `specification/data-format/message-body.md#final-frame-authentication-tag` annotations are there but this is a different spec target.

### Requirement 7 (test for Requirement 1)
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  For a regular frame, each field MUST be deserialized according to its specification:
  ```
- **Current State**: needs-test
- **Placement**: `tests/test_decrypt_the_message_body.rs` — add a `type=test` annotation to an existing test (e.g., `test_decrypt_frame_fields_deserialized_correctly`) or create a new test.

### Requirement 8 (tests for Requirements 2–6)
- **Level**: MUST
- **Exact Quotes** (from TOML):
  ```toml
  For a final frame, each field MUST be deserialized according to its specification:
  ```
  ```toml
  - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be deserialized according to the
  [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
  ```
  ```toml
  - [IV](../data-format/message-body.md#final-frame-iv): MUST be deserialized according to the
  [Final Frame IV](../data-format/message-body.md#final-frame-iv) specification.
  ```
  ```toml
  - [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content): MUST be deserialized according to the
  [Final Frame Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content) specification.
  ```
  ```toml
  - [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag): MUST be deserialized according to the
  [Final Frame Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag) specification.
  ```
- **Current State**: needs-test
- **Placement**: `tests/test_decrypt_the_message_body.rs` — add `type=test` annotations to the existing `test_decrypt_frame_fields_deserialized_correctly` test or `test_decrypt_final_frame_deserialization` test.

## Existing Code Context

### Source File: `src/message/body.rs`

Final frame block where annotations are needed (around lines 170–210):
```rust
        if seq_num == ENDFRAME_SEQUENCE_NUMBER {
            //= specification/data-format/message-body.md#final-frame-sequence-number
            //# The Final Frame Sequence Number MUST be interpreted as the same type as the
            //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
            let seq_num: u32 = read_u32(ciphertext, sig_digest)?;
            if seq_num != expected_frame {
                return Err("Final sequence number out of order.".into());
            }
            //= specification/data-format/message-body.md#final-frame-iv
            //# The length of the IV field MUST be equal to the IV length of the [algorithm suite](../framework/algorithm-suites.md) that generated the message.
            //= specification/data-format/message-body.md#final-frame-iv
            //# The IV MUST be interpreted as bytes.
            read_bytes(ciphertext, &mut iv, sig_digest)?;
```

Regular frame block where parent annotation is needed (around lines 318–325):
```rust
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# Otherwise, this value MUST be 1 greater than the value of the sequence number
        //# of the previous frame.
        expected_frame += 1;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //= reason=read_bytes reads IV bytes from the regular frame
        //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be deserialized according to the
```

### Test File: `tests/test_decrypt_the_message_body.rs`

Existing test that covers frame field deserialization (around line 410):
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_decrypt_frame_fields_deserialized_correctly() {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=test
    //# - The [Sequence Number End](../data-format/message-body.md#sequence-number-end): MUST be deserialized according to the
    //# [Sequence Number End](../data-format/message-body.md#sequence-number-end) specification.
    // ... (more annotations)
    let pt = vec![0xBBu8; 25];
    let result = round_trip(&pt, 10).await;
    assert_eq!(result, pt, "multi-frame round-trip proves all frame fields deserialized correctly");
}
```

## Implementation Guidance

- All 6 implementation annotations go in `src/message/body.rs` in the `read_and_decrypt_framed_message_body` function.
- The 2 parent annotations ("For a regular frame..." and "For a final frame...") are summary annotations placed at the top of their respective deserialization blocks.
- The 4 final frame field annotations (Sequence Number, IV, Encrypted Content, Authentication Tag) are placed immediately before the code that reads each field, alongside the existing `data-format/message-body.md` annotations.
- All test annotations go in `tests/test_decrypt_the_message_body.rs`. The existing `test_decrypt_frame_fields_deserialized_correctly` test already covers regular frame fields and can be extended with the final frame field annotations. Alternatively, `test_decrypt_final_frame_deserialization` can be used for the final frame parent and field annotations.
- Do NOT include `//= type=implementation` — it is the default. Only specify `type=test` for test annotations.
- Follow the existing annotation pattern in `body.rs` where multiple annotations stack before a single code line.

### Spec-Aligned Structure
The spec describes this flow:
1. "For a regular frame, each field MUST be deserialized..." → annotate at the start of the regular frame deserialization block (before `expected_frame += 1`)
2. Regular frame sub-items (Seq Num, IV, Content, Tag) → already annotated at their respective `read_*` calls
3. "For a final frame, each field MUST be deserialized..." → annotate at the start of the ENDFRAME block
4. Final frame Sequence Number → annotate before `read_u32` in ENDFRAME block
5. Final frame IV → annotate before `read_bytes` for IV in ENDFRAME block
6. Final frame Encrypted Content → annotate before/at `read_seq_u32_bounded` result in ENDFRAME block
7. Final frame Authentication Tag → annotate before `read_bytes` for auth_tag in ENDFRAME block

Sub-items to annotate individually:
- "For a regular frame, each field MUST..." → at the start of the regular frame deserialization section, before `expected_frame += 1;`
- "For a final frame, each field MUST..." → at the top of the `if seq_num == ENDFRAME_SEQUENCE_NUMBER` block
- Final frame Sequence Number → at `let seq_num: u32 = read_u32(ciphertext, sig_digest)?;`
- Final frame IV → at `read_bytes(ciphertext, &mut iv, sig_digest)?;` in ENDFRAME block
- Final frame Encrypted Content → at `let _enc_content_is_bytes = &enc_content;` or `read_seq_u32_bounded(...)` in ENDFRAME block
- Final frame Authentication Tag → at `read_bytes(ciphertext, &mut auth_tag, sig_digest)?;` in ENDFRAME block

## Targeted Tests
- `test_decrypt_frame_fields_deserialized_correctly` — existing test that covers frame field deserialization; extend with final frame field test annotations
- `test_decrypt_final_frame_deserialization` — existing test for final frame; can host the parent "For a final frame..." test annotation

## Success Criteria
```bash
cargo test --test test_decrypt_the_message_body
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `decrypt-the-message-body` section (no `TEXT[!MUST]` lines without annotation types)
- [ ] All requirements have `type=implementation` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
