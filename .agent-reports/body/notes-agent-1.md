# Agent 1 Notes — body.rs

## Discovery Summary

### Step 2: Coverage Validation
- `make validate-all-tests` target does not exist in this project. Skipped.

### Step 3: Infrastructure
- `design/requirements/infrastructure.md` does not exist. No infrastructure requirements to check.

### Step 4: Duvet Checks
- Shell access was restricted; duvet could not be run directly.
- Manual analysis performed by comparing TOML requirements against source annotations.

### Gap Analysis

#### Priority 2: Missing Test Annotations (HIGHEST PRIORITY — selected for work item)
The `decrypt-the-message-body` section has 15 implementation annotations in `body.rs` but ZERO test annotations anywhere in the project. This is the largest cluster of untested implementation annotations.

#### Priority 3: TODO Annotations
- `specification/client-apis/encrypt.md#construct-the-body` — "Plaintext Length Bound" (line 534-538)
  - `EncryptInput` struct has no `plaintext_length_bound` field yet
  - This TODO depends on adding the field to `EncryptInput` in `types.rs`, which is outside body.rs scope
  - Deferred until the input field is added

#### Priority 4: Missing Implementation Annotations
The following spec sections have ZERO annotations in body.rs:
1. `data-format/message-body.md#non-framed-data` — 2 MUST requirements (serialization/deserialization order)
2. `data-format/message-body.md#non-framed-data-iv` — 4 MUST requirements
3. `data-format/message-body.md#non-framed-data-encrypted-content` — 2 MUST requirements
4. `data-format/message-body.md#non-framed-data-encrypted-content-length` — 4 MUST requirements
5. `data-format/message-body.md#non-framed-data-authentication-tag` — 2 MUST requirements
6. `data-format/message-body.md#final-frame-sequence-number` — 3 MUST requirements
7. `data-format/message-body.md#regular-frame-encrypted-content` — "The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length."
8. `data-format/message-body.md#regular-frame-iv` — "Each frame in the [Framed Data] MUST include an IV that is unique within the message."
9. `data-format/message-body.md#final-frame-iv` — "The IV MUST be a unique IV within the message."
10. `data-format/message-body-aad.md#content-length` — 2 sub-item requirements (regular frames, final frame)

Also missing from decrypt-the-message-body (not annotated in body.rs):
- "Once the message header is successfully parsed, the next sequential bytes MUST be deserialized..."
- "The Decrypt operation MUST use the [content type] field parsed from the message header..."
- "If there could still be message body left to deserialize and decrypt..."
- All deserialization sub-items (Sequence Number End, Sequence Number, IV, Encrypted Content Length, Encrypted Content, Authentication Tag)
- All AAD sub-items for decrypt (message ID, Body AAD Content, sequence number, IV, cipherkey, ciphertext, tag)
- Streaming requirements (without signature, with signature, final frame release, signature input)

### Annotation Prefix Note
body.rs uses both `specification/` (symlink) and `aws-encryption-sdk-specification/` prefixes.
The TOML targets use `aws-encryption-sdk-specification/`. Both should work with duvet since
`specification/` is a symlink to `aws-encryption-sdk-specification/`.

## Spec-Aligned Structure Analysis

### Q1: Logical flow of decrypt-the-message-body
1. Parse content type from header → determine framed vs non-framed
2. For framed: read first 4 bytes of each frame
3. If 0xFFFFFFFF → final frame path
4. Otherwise → regular frame path
5. Validate sequence numbers (first=1, subsequent=prev+1)
6. For final frame: validate content length <= frame length
7. Construct AAD (message ID, body AAD content, sequence number, content length)
8. Decrypt and authenticate using AES-GCM
9. On failure → halt immediately, no unauthenticated plaintext released
10. Streaming: release regular frames after tag verification; hold final frame until signature verification

### Q2: Code constructs fulfilling requirements
- Frame type determination → `if seq_num == ENDFRAME_SEQUENCE_NUMBER` branch
- Sequence number validation → `if seq_num != expected_frame` error check
- Content length validation → `read_seq_u32_bounded()` call
- AAD construction → `body_aad()` call
- Decryption → `aes_decrypt()` call
- Error propagation → `?` operator on `aes_decrypt()`
- Unauthenticated plaintext prevention → `aes_decrypt()` returns error before any write

### Q3: Sub-items
The decrypt-the-message-body spec has extensive sub-items for deserialization fields and AAD construction.
These are listed in the work item.

### Q4: Most likely structural mistake
The implementer may be tempted to write a single "round-trip proves everything" test.
Instead, each requirement needs its own test annotation, even if the test body is similar.
The key is that each test annotation quotes a DIFFERENT requirement.

## Potential Spec Gaps

### 1. Sequence number out-of-order error on decrypt
- **Code location**: `body.rs` line 129 — `if seq_num != expected_frame { return Err("Final sequence number out of order.".into()); }`
  and line 226 — `if seq_num != expected_frame { return Err("Sequence number out of order.".into()); }`
- **Why it matters**: Security — prevents frame reordering attacks
- **Suggested spec requirement**: "If the sequence number of a frame does not match the expected sequence number, the Decrypt operation MUST fail."

### 2. Non-framed message frame length validation
- **Code location**: `body.rs` line 286 — `if header.body.frame_length() != 0 { return Err(...) }`
- **Why it matters**: Correctness — enforces that non-framed messages have frame_length=0
- **Note**: This is annotated against `message-header.md#frame-length`, not `message-body.md`. The annotation is correct.

### 3. MAX_DATA limit (2^36 - 32)
- **Code location**: `body.rs` line 330 — `const MAX_DATA: usize = (1usize << 36) - 32;`
- **Why it matters**: Security — enforces AES-GCM cryptographic safety limit
- **Note**: This is enforced but not annotated against a specific spec requirement. The non-framed-data-encrypted-content-length spec has this requirement: "The length MUST NOT be greater than `2^36 - 32`". The framed data path enforces this via `total_data_size > MAX_DATA` checks.
