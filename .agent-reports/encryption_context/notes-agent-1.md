# Agent 1 Notes — encryption_context

## Spec-Aligned Structure Analysis

### 1. Logical flow of the AAD / Key Value Pairs spec sections

The spec describes three nested sections:

1. **AAD** (`#aad`): The top-level AAD field is composed of two sub-fields serialized in order:
   - Key Value Pairs Length (2 bytes, UInt16)
   - Key Value Pairs (variable)

2. **Key Value Pairs Length** (`#key-value-pairs-length`): The byte-length of the Key Value Pairs field.
   - MUST be 2 bytes
   - MUST be serialized as UInt16
   - When encryption context is empty, value MUST be 0

3. **Key Value Pairs** (`#key-value-pairs`): The actual encoding of the encryption context.
   - MUST be serialized per `structures.md#serialization`
   - When encryption context is empty, this field MUST NOT be included

### 2. Code construct mapping

| Requirement | Code construct |
|---|---|
| AAD ordering (327) | `write_aad_section` function body — sequential calls to `write_u16` then `write_aad` |
| KVP Length is 2 bytes (328) | `write_u16(w, bytes as u16)` in `write_aad_section` — `write_u16` writes exactly 2 bytes |
| KVP Length as UInt16 (329) | Same `write_u16` call — `write_u16` serializes as UInt16 by definition |
| Empty EC → length 0 (330) | `write_u16(w, 0)` in `write_aad_section` when `data.is_empty()` |
| KVP serialization (331) | The `for` loop in `write_aad` — already annotated |
| Empty EC → no KVP field (332) | `write_empty_ec_or_write_aad` returning `Ok(())` when empty — already annotated |

### 3. Sub-items

The AAD requirement (327) has sub-items:
- "Key Value Pairs Length" → `write_u16(w, bytes as u16)` in `write_aad_section`
- "Key Value Pairs" → `write_aad(w, data)` call in `write_aad_section`

### 4. Most likely structural mistake

The implementer may be tempted to annotate the `write_aad` function for the AAD ordering requirement, but `write_aad` only writes the Key Value Pairs content (count + entries). The AAD ordering (length + content) is fulfilled by `write_aad_section`.

Also, the existing annotation for requirement 330 ("empty → 0") is placed in `write_aad`, but `write_aad` is only called when data is NOT empty (from `write_empty_ec_or_write_aad`). The correct placement for the empty-case annotation is in `write_aad_section` where `write_u16(w, 0)` is called. However, `write_aad` also writes `data.len()` as the count, which is 0 when empty — but this function is never called with empty data in the normal flow.

## Potential Spec Gaps

### 1. `write_aad` writes count, not just key-value pairs
- **Code location**: `write_aad` function, line `write_u16(w, data.len() as u16)?`
- **Behavior**: The function writes the count of key-value pairs before the entries. This count field is part of the `structures.md#serialization` spec, not the `message-header.md#key-value-pairs` spec.
- **Why it matters**: Interoperability — the count field is essential for deserialization but is only described in the structures spec, not in the message-header AAD section.
- **Suggested spec requirement**: The AAD section could explicitly mention that Key Value Pairs includes a count prefix per the serialization spec.

### 2. `read_canonical_ec` has no duvet annotations
- **Code location**: `read_canonical_ec` function (lines 9-24)
- **Behavior**: Deserializes the AAD section from a reader, reading length, count, then key-value pairs.
- **Why it matters**: Correctness — the deserialization path has no spec traceability.
- **Suggested spec requirement**: The deserialization of AAD should be annotated with the same spec requirements as serialization.
