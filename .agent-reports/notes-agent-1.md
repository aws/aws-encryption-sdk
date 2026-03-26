# Agent 1 Discovery Notes — body.rs Coverage Analysis

## Date
Auto-generated during discovery pass.

## Scope
Analyzed `body.rs` coverage of:
- `data-format/message-body.md`
- `data-format/message-body-aad.md`
- `client-apis/encrypt.md#construct-the-body`
- `client-apis/encrypt.md#construct-a-frame`
- `client-apis/decrypt.md#decrypt-the-message-body`

## Coverage Summary

### Well-Covered Sections
- `decrypt.md#decrypt-the-message-body` — Dense annotations in `read_and_decrypt_framed_message_body`
- `encrypt.md#construct-a-frame` — Good test coverage in `test_construct_a_frame.rs`
- `message-body-aad.md#body-aad-content` — All 3 MUST requirements annotated at constants
- `message-body-aad.md#sequence-number` — Non-framed case annotated; framed case covered by call sites
- `message-body-aad.md#content-length` — Non-framed case annotated; framed cases NOT annotated

### Major Gaps (by priority)

#### 1. `encrypt.md#construct-the-body` — 9 MUST requirements, only 2 annotated
**Selected as work item.** The `encrypt_and_serialize_body` function implements all these requirements but lacks annotations.

#### 2. `data-format/message-body.md` — Non-framed data sections (12+ MUST requirements, 0 annotated)
- `#non-framed-data` — serialization/deserialization ordering (2 MUST)
- `#non-framed-data-iv` — 4 MUST requirements
- `#non-framed-data-encrypted-content-length` — 4 MUST requirements
- `#non-framed-data-encrypted-content` — 2 MUST requirements
- `#non-framed-data-authentication-tag` — 2 MUST requirements

The `read_and_decrypt_non_framed_message_body` function has NO data-format annotations. It only has `message-body-aad.md` annotations.

#### 3. `data-format/message-body.md` — Framed data serialization ordering
- `#regular-frame` — "A regular frame MUST be serialized as, in order..." — NOT annotated
- `#final-frame` — "A final frame MUST be serialized as, in order..." — NOT annotated
- `#final-frame` — "This means a final frame MUST be a regular frame with the addition..." — NOT annotated

#### 4. `data-format/message-body.md` — Sequence number serialization
- `#regular-frame-sequence-number` — "When serializing... MUST be 4 bytes" — NOT annotated
- `#regular-frame-sequence-number` — "MUST be serialized as a UInt32" — NOT annotated
- `#regular-frame-sequence-number` — "MUST be interpreted as a UInt32" — annotated as implication in decrypt path only

#### 5. `data-format/message-body.md` — Final frame sequence number
- `#final-frame-sequence-number` — ALL 3 MUST requirements NOT annotated

#### 6. `data-format/message-body.md` — IV uniqueness
- `#regular-frame-iv` — "Each frame MUST include an IV that is unique within the message" — NOT annotated
- `#final-frame-iv` — "The IV MUST be a unique IV within the message" — NOT annotated

#### 7. `data-format/message-body-aad.md` — Framed data content-length sub-items
- `#content-length` — "For framed data, this value MUST equal the length..." — NOT annotated
- `#content-length` — "For regular frames, this value MUST equal the value of the frame length..." — NOT annotated
- `#content-length` — "For the final frame, this value MUST be >= 0 and <= frame length..." — NOT annotated
- `#sequence-number` — "For framed data, the value MUST be the frame sequence number" — NOT annotated

#### 8. `data-format/message-body.md` — Final frame plaintext length constraints
- `#final-frame` — "The length of the plaintext to be encrypted in the Final Frame MUST be >= 0 and <= Frame Length" — NOT annotated
- `#final-frame` — "When the length of the Plaintext is less than the Frame Length, the body MUST contain exactly one frame..." — NOT annotated

#### 9. `data-format/message-body.md#sequence-number-end`
- "The value MUST be encoded as the 4 bytes FF FF FF FF" — annotated in header.rs, NOT in body.rs encrypt path

#### 10. Missing test annotations for ALL data-format/message-body.md requirements
No `type=test` annotations exist for any `data-format/message-body.md` or `data-format/message-body-aad.md` requirements.

## Spec-Aligned Structure Analysis

### `encrypt.md#construct-the-body` Logical Flow
1. **Pre-loop setup** — frame length, sequence number initialization
2. **Regular frame loop** — process as much plaintext as possible into regular frames
3. **End-of-input detection** — three cases:
   a. Short read (`in_size != frame_length`) → not enough for regular frame
   b. Exact read + no more data (`next_char.is_none()`) → exact match
   c. Exact read + more data (`next_char.is_some()`) → continue loop
4. **Final frame construction** — construct final frame with remaining bytes (may be 0)

### Code Construct Mapping
- "MUST process as much of the consumable bytes as possible" → the `loop` statement
- "exactly enough consumable plaintext bytes" → `next_char.is_none()` break
- "enough input plaintext bytes consumable to create a new regular frame" → loop continuation after `next_char` check
- "not enough input consumable plaintext bytes" → `in_size != frame_length` break
- "MUST construct an empty final frame" → `construct_frame` with `plaintext_frame[0..in_size]` where `in_size == 0`

### Most Likely Structural Mistake
An implementer might annotate Req 4/5/6 (the three end-of-input cases) all at the final `construct_frame` call. But these requirements describe the *decision logic*, not the frame construction. They should be annotated at the branch/break points.

## Potential Spec Gaps

### 1. MAX_DATA size limit not in spec
- **Code location**: `body.rs` line 275: `const MAX_DATA: usize = (1usize << 36) - 32;`
- **Behavior**: The code enforces a maximum total plaintext size of 2^36 - 32 bytes across all frames, returning an error if exceeded.
- **Why it matters**: Security — this is the AES-GCM cryptographic safety limit for a single key. Exceeding this would compromise the encryption's security guarantees.
- **Suggested spec requirement**: "The total plaintext encrypted across all frames in a single message MUST NOT exceed 2^36 - 32 bytes, due to restrictions imposed by the implemented algorithms."

### 2. Peek-ahead logic for exact-match frame boundary
- **Code location**: `body.rs` `encrypt_and_serialize_body` — the `read_opt_u8` / `next_char` peek logic
- **Behavior**: When the plaintext is an exact multiple of frame length, the code peeks one byte ahead to decide whether to emit a regular frame or break to final frame. This is an implementation detail that determines whether the "exact match" case produces a single final frame or a regular frame + empty final frame.
- **Why it matters**: Interoperability — different implementations may make different choices here, and the spec says "SHOULD be equal to the frame length but MAY be 0" for the exact-match case. The Rust implementation always breaks to final frame on exact match (producing a single final frame with content = frame_length), which is the SHOULD behavior.
- **Suggested spec requirement**: No change needed — the spec already covers this with SHOULD/MAY.

### 3. `fail_if_multi_frame` parameter in decrypt path
- **Code location**: `body.rs` `read_and_decrypt_framed_message_body` — the `fail_if_multi_frame` parameter
- **Behavior**: When `fail_if_multi_frame` is true, the function errors if more than one frame is encountered. This is used by the streaming interface to prevent releasing unverified plaintext.
- **Why it matters**: Security — this is a streaming-specific safety mechanism.
- **Suggested spec requirement**: Already partially covered by the streaming spec's requirement about not releasing plaintext before signature verification.
