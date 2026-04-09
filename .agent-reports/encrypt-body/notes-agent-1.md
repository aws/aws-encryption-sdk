# Agent 1 Notes — encrypt-body

## Discovery Summary

All three spec sections are fully covered with both implementation and test annotations.

### Spec-Aligned Structure Analysis

1. **Construct the body** — logical flow:
   - Check plaintext length bound → annotated at `if total_data_size > max_plaintext_len` in `body.rs`
   - Loop: process consumable bytes as regular frames → annotated at the main `loop` in `encrypt_and_serialize_body`
   - End-of-input: exact-frame-length → final-or-regular → annotated at `next_char.is_none()` check
   - End-of-input: more-than-frame → regular frame → annotated at the regular frame `construct_frame` call
   - End-of-input: less-than-frame → final frame → annotated at `in_size != frame_length` break
   - Empty final frame → annotated at the final `construct_frame` call

2. **Construct a frame** — logical flow:
   - Build AAD (message ID, body AAD content, sequence number, content length) → annotated at `body_aad()` call
   - Build IV from sequence number → annotated at `iv_seq()` call
   - Encrypt with AES-GCM → annotated at `aes_encrypt()` call
   - Serialize frame fields (seq num, IV, encrypted content, auth tag) → annotated at individual `write_*` calls
   - Release frame → annotated at `write_bytes(ciphertext, frame_buf)`
   - Feed to signature digest → annotated at `write_bytes(sig_digest, frame_buf)`

3. **Un-Framed Message Body Encryption** — single requirement:
   - MUST NOT encrypt using Non-Framed → annotated as implication at `content_type: ContentType::Framed`

## Potential Spec Gaps

### 1. Maximum frame count enforcement
- **Code location**: `body.rs`, `encrypt_and_serialize_body` — `if sequence_number == ENDFRAME_SEQUENCE_NUMBER`
- **Why it matters**: Correctness — prevents overflow of the 4-byte sequence number field
- **Spec coverage**: This is covered by `data-format/message-body.md#regular-frame-sequence-number` ("The number of frames in a single message MUST be less than or equal to `2^32 - 1`"), which is annotated. No gap.

### 2. Maximum data size enforcement
- **Code location**: `body.rs`, `encrypt_and_serialize_body` — `if total_data_size > MAX_DATA`
- **Why it matters**: Correctness/security — prevents encrypting more data than the format supports
- **Spec coverage**: The `MAX_DATA` check exists but is not annotated to a specific spec requirement. The encrypt spec's `plaintext-length-bound` covers the user-specified bound, but the absolute maximum (from the message format) may not have a direct spec requirement in the encrypt section. This is advisory — the data-format spec likely covers it.

### 3. Debug assertions for final frame invariants
- **Code location**: `body.rs` — `debug_assert!(in_size <= frame_length)` and `debug_assert!(in_size > 0 || ...)`
- **Why it matters**: Correctness — validates internal invariants
- **Spec coverage**: These are internal assertions, not spec requirements. No gap.
