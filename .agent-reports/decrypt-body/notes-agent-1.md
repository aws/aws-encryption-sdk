# Agent 1 Notes — decrypt-the-message-body

## Spec Section Logical Flow

The `decrypt-the-message-body` section describes:

1. After header parsing, deserialize body bytes per the message body spec
2. Use content type to determine framed vs non-framed
3. For framed: use first 4 bytes to detect regular vs final frame
4. For regular frames: deserialize Seq Num, IV, Encrypted Content, Auth Tag
5. For final frames: deserialize Seq Num End, Seq Num, IV, Encrypted Content Length, Encrypted Content, Auth Tag
6. Decrypt each frame with AES-GCM using AAD (message ID, body AAD content, sequence number, content length), IV, derived key, ciphertext, tag
7. Fail immediately on decryption failure; never release unauthenticated plaintext
8. Streaming: release regular frame plaintext after tag verification; hold final frame/unframed until signature verification

## Coverage Analysis

The snapshot format `TEXT[!MUST,implementation,test]` means: MUST-level requirement, HAS implementation and test annotations. The `!` prefix on the level is a normative marker, not a failure indicator.

`TEXT[!MUST]` (without implementation or test) means: MUST-level requirement with NO annotations — these are the actual gaps.

### Gaps Found (8 total: 6 implementation + corresponding tests)

All gaps are in the final frame deserialization sub-section. The regular frame fields are fully annotated, but the parallel final frame fields and both parent summary requirements are missing:

1. `For a regular frame, each field MUST be deserialized according to its specification:` — parent summary, no annotation
2. `For a final frame, each field MUST be deserialized according to its specification:` — parent summary, no annotation
3. Final frame Sequence Number — no `client-apis/decrypt.md` annotation (has `data-format/message-body.md` annotation)
4. Final frame IV — no `client-apis/decrypt.md` annotation (has `data-format/message-body.md` annotation)
5. Final frame Encrypted Content — no annotation
6. Final frame Authentication Tag — no annotation

Plus corresponding test annotations for all 6.

## Traceability Answers

### Q1: Spec section logical flow
See above.

### Q2: Where each requirement is fulfilled in code
- Parent "For a regular frame..." → the block of `read_bytes` calls after `expected_frame += 1` in the loop body
- Parent "For a final frame..." → the `if seq_num == ENDFRAME_SEQUENCE_NUMBER` block
- Final frame Seq Num → `let seq_num: u32 = read_u32(ciphertext, sig_digest)?;` inside ENDFRAME block
- Final frame IV → `read_bytes(ciphertext, &mut iv, sig_digest)?;` inside ENDFRAME block
- Final frame Encrypted Content → `read_seq_u32_bounded(...)` result inside ENDFRAME block
- Final frame Auth Tag → `read_bytes(ciphertext, &mut auth_tag, sig_digest)?;` inside ENDFRAME block

### Q3: Sub-items under normative requirements
Yes — the "For a regular/final frame" requirements each have a list of field sub-items. Each sub-item maps to a distinct `read_*` call. All are quoted individually in the TOML.

### Q4: Most likely structural mistake
The implementer might be tempted to annotate the final frame field requirements at the same location as the existing `data-format/message-body.md` annotations. This is actually correct — the same code line fulfills both the data-format spec (field format) and the client-api spec (decrypt deserialization). Multiple annotation blocks can stack before the same code line.

## Potential Spec Gaps

No significant spec gaps identified. The code behavior closely matches the spec for this section. The only notable observation:

- **Code location**: `body.rs` line 176 — `if seq_num != expected_frame` check for final frame sequence number ordering. The spec says sequence numbers must increment, but doesn't explicitly say the final frame's sequence number must equal the expected next sequence number. The code enforces this, which is correct behavior implied by the spec's incrementing requirement.
  - **Why it matters**: Correctness — ensures no frames were dropped or reordered
  - **Suggested spec requirement**: Already covered by "Otherwise, this value MUST be 1 greater than the value of the sequence number of the previous frame."

## Self-Verification

1. ✅ TOML content was read from `.duvet/requirements/specification/client-apis/decrypt/decrypt-the-message-body.toml`
2. ✅ Source file `src/message/body.rs` exists (verified via read)
3. ✅ Test file `tests/test_decrypt_the_message_body.rs` exists (verified via read)
4. ✅ `duvet report` was executed and snapshot was read
5. ✅ `cargo test --test test_decrypt_the_message_body -- --list` was executed to discover tests
