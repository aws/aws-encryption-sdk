# Discovery Notes — encrypt.rs

## Spec Structure Analysis (Step 6.8)

### 1. Spec Section Logical Flow: `#authentication-tag`

The spec describes this flow for the authentication tag:
1. After serializing the header body, calculate an authentication tag over it
2. Use the authenticated encryption algorithm from the algorithm suite
3. Construct the AAD as concatenation of serialized header body + serialized required EC
4. Use IV of 0
5. Use derived data key as cipherkey
6. Use empty plaintext
7. Do not release serialized bytes until entire header is serialized
8. If streaming and header is complete, release it
9. Output encrypted message must have header equal to calculated header
10. If headers not equal, fail
11. If signature algorithm present and streaming, input header to signature algorithm

### 2. Code Constructs Fulfilling Each Requirement

- "MUST calculate an authentication tag" → `build_header_auth_tag` function call
- "MUST be the output of the authenticated encryption algorithm" → `aes_encrypt(...)` call
- "AAD MUST be the concatenation" → `&[raw_header, serialized_req_encryption_context].concat()`
- "encryption context to only authenticate MUST be..." → `required_encryption_context_map` construction in `build_header_for_encrypt`
- "IV MUST have a value of 0" → `let iv = vec![0; ...]`
- "cipherkey MUST be the derived data key" → `data_key` parameter
- "plaintext MUST be an empty byte array" → `&[]` (empty slice passed to aes_encrypt)
- "serialized bytes MUST NOT be released until entire header serialized" → structural: header is fully built before writing to ciphertext
- "serialized message header MUST be released" → `header::serialize_header` writes to ciphertext
- "encrypted message MUST have header equal" → structural: same header object used
- "headers not equal, MUST fail" → structural: single code path
- "MUST input serialized header to signature algorithm" → `DigestWriter` (dw) receives header bytes during serialization

### 3. Sub-items Under the Normative Requirement

The main requirement "MUST calculate an authentication tag" has sub-items for:
- AAD construction (with nested sub-item about required EC filtering)
- IV value
- Cipherkey
- Plaintext

### 4. Most Likely Structural Mistake

The implementer may be tempted to annotate all AAD-related requirements at the `aes_encrypt` call.
Instead, the AAD construction (concatenation) should be annotated at the `.concat()` line,
and the required EC filtering should be annotated at the `required_encryption_context_map` construction
in `build_header_for_encrypt`.

## Potential Spec Gaps

### 1. Data Key Length Validation
- **Code**: `build_header_auth_tag` checks `data_key.len() != key_length` before encryption
- **Why it matters**: Security — prevents using wrong-length keys for AES
- **Suggested spec**: "The operation MUST verify that the derived data key length matches the key length specified by the algorithm suite before computing the header authentication tag."

## Coverage Summary

Sections with ZERO annotations anywhere in codebase for `encrypt.md`:
- `#authentication-tag` — 11 MUST requirements (HIGHEST PRIORITY)
- `#construct-a-frame` — many MUST requirements (only in test files, not source)

Sections annotated in other files (not encrypt.rs):
- `#v1-header` — in v2_header_body.rs (partial)
- `#v2-header` — in v2_header_body.rs (9 annotations)
- `#v1-authentication-tag` — in header_auth.rs (3 annotations)
- `#v2-authentication-tag` — in header_auth.rs (2 annotations)
- `#frame-length` — in types.rs (2 annotations, implication)
- `#plaintext` — in types.rs (1 annotation, implication)
- `#input` — in types.rs (7 annotations, implication)
