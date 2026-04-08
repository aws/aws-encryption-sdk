# Agent 1 Notes — encrypt-body

## Spec Section Logical Flow

### `## Construct the body`
1. Regular frames conform to Regular Frame spec
2. Final frames conform to Final Frame spec
3. Output body must equal calculated body
4. Plaintext Length Bound enforcement (runtime check)
5. Before end-of-input: process as many regular frames as possible
6. At end-of-input: three-way branch (exact fit, more than one frame, less than one frame)
7. Empty final frame if no bytes remain and no final frame yet

### `### Construct a frame`
1. Calculate encrypted content + auth tag via AES-GCM with:
   - AAD (message body AAD with message_id, body_aad_content, sequence_number, content_length)
   - IV (sequence number padded to IV length)
   - Cipherkey (derived data key)
   - Plaintext (next unconsumed bytes)
2. Serialize regular frame fields: SeqNum, IV, EncContent, AuthTag
3. Serialize final frame fields: SeqNumEnd, SeqNum, IV, EncContentLen, EncContent, AuthTag
4. Frame bytes not released until fully serialized
5. Streaming: release frame once complete; feed to signature digest

### `### Un-Framed Message Body Encryption`
Single requirement: MUST NOT encrypt using Non-Framed content type.

## Where Each Requirement Is Fulfilled in Code

### Construct the body — `encrypt_and_serialize_body()` in `body.rs`
- Regular/final frame conformance → delegated to `construct_frame()` calls
- Body equality → structural (body written directly to output)
- Plaintext Length Bound → `if total_data_size > max_plaintext_len` checks
- Process regular frames → the `loop` with `read_up_to_peek`
- End-of-input branching → `if in_size != frame_length` + `if next_char.is_none()`
- Empty final frame → final `construct_frame()` call with `plaintext_frame[0..in_size]` where `in_size=0`

### Construct a frame — `construct_frame()` in `body.rs`
- AAD → `body_aad()` call
- IV → `iv_seq()` call
- Cipherkey → `input.key` passed to `aes_encrypt`
- Plaintext → `input.plaintext` passed to `aes_encrypt`
- Regular frame serialization → `write_u32(frame_buf, input.sequence_number)`, `write_bytes(frame_buf, iv)`, `aes_encrypt` appends enc_content+tag
- Final frame serialization → same as regular but with `write_u32(frame_buf, ENDFRAME_SEQUENCE_NUMBER)` and `write_u32(frame_buf, input.plaintext.len() as u32)` for content length
- Frame release → `write_bytes(ciphertext, frame_buf)` after full serialization
- Signature feed → `write_bytes(sig_digest, frame_buf)`

### Un-Framed — `build_header_body()` in `encrypt.rs`
- `content_type: ContentType::Framed` — structurally prevents non-framed encryption

## Spec Structure Traceability

The `construct_frame()` function handles BOTH regular and final frames in a single code path.
The spec describes them separately with distinct field lists.
The existing annotations cover regular frame fields but NOT the final frame-specific field requirements.

The code uses `if input.is_final` branches for:
- Sequence Number End (before sequence number)
- Encrypted Content Length (before encrypted content)

But the IV, Encrypted Content, and Authentication Tag are shared code paths.
The final frame spec requirements for these fields need annotations at the shared code
with `reason=` explaining the code handles both regular and final frames.

## Most Likely Structural Mistake

Agent 2 may be tempted to add final frame annotations only inside `if input.is_final` blocks.
But IV serialization (`write_bytes(frame_buf, iv)`), encrypted content, and auth tag
are OUTSIDE the `if input.is_final` block — they're shared between regular and final frames.
The annotations for final frame IV, encrypted content, and auth tag MUST go at these shared locations.

## Potential Spec Gaps

### 1. MAX_DATA overflow check
- **Code**: `body.rs` line 868: `if total_data_size > MAX_DATA { return Err("Plain text too large") }`
- **Behavior**: Enforces a 2^36 - 32 byte limit on total plaintext across all frames
- **Why it matters**: Security — prevents AES-GCM nonce reuse / cryptographic safety violation
- **Suggested spec text**: "The total plaintext encrypted across all frames MUST NOT exceed 2^36 - 32 bytes."

### 2. Sequence number overflow check
- **Code**: `body.rs` line 863: `if sequence_number == ENDFRAME_SEQUENCE_NUMBER { return Err("too many frames") }`
- **Behavior**: Prevents sequence number from reaching 0xFFFFFFFF (reserved for ENDFRAME marker)
- **Why it matters**: Correctness — 0xFFFFFFFF is the final frame marker, not a valid sequence number
- **Note**: The data-format spec covers this (`number of frames <= 2^32 - 1`) but the encrypt.md spec doesn't explicitly mention this check.
