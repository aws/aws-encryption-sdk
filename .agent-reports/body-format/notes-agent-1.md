# Agent 1 Notes — body-format Cycle 2

## Spec-Aligned Structure Analysis

### Non-Framed Data (§non-framed-data)
Logical flow:
1. Serialization order: IV → Content Length → Content → Auth Tag
2. Deserialization order: IV → Content Length → Content → Auth Tag

Code fulfillment:
- Serialization: ESDK does NOT encrypt non-framed (exception needed)
- Deserialization: `read_and_decrypt_non_framed_message_body` reads IV, then content length via `read_seq_u64_bounded`, then auth tag via `read_vec`

### Non-Framed Data Sub-Sections
Each sub-section (IV, Content Length, Content, Auth Tag) has requirements about:
- Uniqueness (IV)
- Byte length constraints
- Serialization format (Uint64 for content length)
- Interpretation as bytes

Code fulfillment:
- IV: `read_vec(r, get_iv_length(...))` — reads IV length bytes
- Content Length: `read_seq_u64_bounded(r, SAFE_MAX_ENCRYPT, ...)` — reads 8 bytes as u64, enforces 2^36-32 limit
- Content: returned by `read_seq_u64_bounded` which reads content_len bytes
- Auth Tag: `read_vec(r, get_tag_length(...))` — reads tag length bytes

### Framed Data Sections
Most framed data requirements are covered by Cycle 1 annotations.
Remaining gaps are:
- Missing impl annotations for structural/implication requirements
- Misquoted annotations that don't match TOML exactly

## Misquoted Annotations (FIX_ANNOTATION)

1. body.rs line 114: `The length of the serialized sequence number MUST be 4 bytes.`
   Should be: `When serializing the sequence number to a message, the length of the serialized sequence number MUST be 4 bytes.`

2. body.rs line 117: `The sequence number MUST be interpreted as a UInt32.`
   Should be: `When reading the sequence number from a message, the sequence number MUST be interpreted as a UInt32.`

3. body.rs line 146: `The length of the serialized encrypted content length field MUST be 4 bytes.`
   Should be: `When serializing the encrypted content length to a message, the length of the serialized encrypted content length field MUST be 4 bytes.`

4. body.rs line 148: `The encrypted content length MUST be interpreted as a UInt32.`
   Should be: `When reading the encrypted content length from a message, the encrypted content length MUST be interpreted as a UInt32.`

## Most Likely Structural Mistake

The implementer may be tempted to annotate non-framed-data requirements
at the `read_and_decrypt_non_framed_message_body` function signature
rather than at the specific read calls that fulfill each sub-requirement.
Each `read_vec` / `read_seq_u64_bounded` call fulfills a specific sub-section requirement.

## Potential Spec Gaps

### 1. Non-framed content length upper bound enforcement on read
- **Code location**: `read_and_decrypt_non_framed_message_body` uses `read_seq_u64_bounded(r, SAFE_MAX_ENCRYPT, ...)`
- **Behavior**: Rejects content lengths > 2^36-32 during deserialization
- **Why it matters**: Security — prevents memory exhaustion attacks
- **Spec says**: The length MUST NOT be greater than 2^36-32 (covers this)
- **Status**: Spec covers this, just needs annotation

### 2. Non-framed sequence number is always 1
- **Code location**: `body_aad` call uses `NONFRAMED_SEQUENCE_NUMBER` (= 1)
- **Behavior**: Hardcoded to 1 for non-framed data
- **Why it matters**: Interoperability — other implementations expect this
- **Status**: Covered by message-body-aad.md spec, not message-body.md
