# Pre-Implementation Reasoning

## 1. Logical steps in the spec section

1. AAD = Key Value Pairs Length + Key Value Pairs (serialized in order)
2. Key Value Pairs Length is 2 bytes / UInt16
3. When EC is empty, Key Value Pairs Length = 0
4. Key Value Pairs serialized per structures.md#serialization
5. When EC is empty, Key Value Pairs field excluded from AAD

## 2. Point of fulfillment for each requirement

| Req | Quote | Point of fulfillment |
|-----|-------|---------------------|
| 1 | "The AAD MUST be serialized as, in order, Key Value Pairs Length, and Key Value Pairs." | `write_aad_section` function body — first writes length, then writes KVP |
| 2 | "The length of the serialized key value pairs length field MUST be 2 bytes." | `write_u16(w, bytes as u16)` in `write_aad_section` — `write_u16` always writes 2 bytes |
| 3 | "The key value pairs length MUST be serialized as a UInt16." | Same `write_u16` call — UInt16 is structural |
| 4 | "When the encryption context is empty, the value of this field MUST be 0." | `write_u16(w, 0)` in `write_aad_section`'s empty branch |
| 5 | "The encryption context key-value pairs MUST be serialized according to its specification for serialization." | Already annotated in `write_aad` loop |
| 6 | "When the encryption context is empty, this field MUST NOT be included in the AAD." | Already annotated at `write_empty_ec_or_write_aad` |

## 3. Sub-items

Req 1 has sub-items: "Key Value Pairs Length" and "Key Value Pairs" — annotate each at the specific code line in `write_aad_section`.

## 4. Reviewer readability

`write_aad_section` is short (~10 lines). Annotations will be close to their fulfilling code. No restructuring needed.

## 5. Existing similar code

`test_header_structure.rs` — uses round-trip encrypt/decrypt with raw AES keyring (no KMS). Same pattern for our tests.

## Misplaced annotation fix

The annotation at line 66 of `write_aad` for "When the encryption context is empty, the value of this field MUST be 0" is misplaced — `write_aad` is never called with empty data (the empty case returns early in `write_aad_section`). Move it to `write_aad_section` at the `write_u16(w, 0)` line.
