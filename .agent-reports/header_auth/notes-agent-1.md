# Agent 1 Notes — header_auth.rs

## Potential Spec Gaps

### 1. V2 read creates a zero IV not mentioned in spec
- **Code location**: `read_header_auth_tag_v2()` line ~100 — `let header_iv = vec![0u8; get_iv_length(suite) as usize];`
- **Behavior**: When reading a V2 header auth, the code synthesizes a zero IV even though V2 doesn't serialize one. This is needed internally for the AES-GCM verification but the spec only says "The V2 Header Authentication MUST be serialized as the Authentication Tag only."
- **Why it matters**: Interoperability — other implementations must also use a zero IV when verifying V2 header auth tags. The spec doesn't explicitly state this for the deserialization path.
- **Suggested spec requirement**: "When deserializing a V2 Header Authentication, the IV used for verification MUST be a zero-valued byte sequence of length equal to the IV length of the algorithm suite."

### 2. No validation of IV/tag lengths during deserialization
- **Code location**: `read_header_auth_tag_v1()` and `read_header_auth_tag_v2()` — they use `get_iv_length(suite)` and `get_tag_length(suite)` to determine read lengths but don't validate the read data matches expected lengths.
- **Why it matters**: Correctness — if the stream is truncated, the error comes from the read layer, not from a length validation. This is probably fine since `read_vec` will fail on short reads, but the spec says "The length of the serialized IV MUST be equal to..." which implies validation.
- **Suggested spec requirement**: N/A — the existing spec requirement covers this implicitly through the serialization format.

## Spec Structure Analysis

### 1. Spec Section Logical Flow

The header-authentication section describes:
1. **V1 serialization order**: IV then Authentication Tag
2. **V2 serialization**: Authentication Tag only
3. **IV sub-section**: length constraint + byte interpretation
4. **Authentication Tag sub-section**: length constraint + byte interpretation

The encrypt.md v1/v2 authentication tag sections describe:
1. **V1**: Serialize header auth with IV (padded to IV length with 0) and Authentication Tag
2. **V2**: Serialize header auth with Authentication Tag only

### 2. Where Each Requirement Is Fulfilled in Code

| Requirement | Code Construct |
|---|---|
| V1 serialization order | `write_header_auth_tag_v1()` match arm writing IV then tag |
| V2 serialization | `write_header_auth_tag_v2()` match arm writing tag only |
| IV length constraint | `read_header_auth_tag_v1()` — `read_vec(r, get_iv_length(suite) as usize, raw)` |
| IV interpreted as bytes | `read_header_auth_tag_v1()` — `read_vec` returns `Vec<u8>` |
| Auth tag length constraint | `read_header_auth_tag_v1/v2()` — `read_vec(r, get_tag_length(suite) as usize, raw)` |
| Auth tag interpreted as bytes | Already annotated in `encrypt.rs` |

### 3. Sub-items

The IV and Authentication Tag sections don't have sub-items — they each have two flat MUST requirements.

### 4. Most Likely Structural Mistake

The implementer might be tempted to annotate the IV/tag length requirements at the `get_iv_length`/`get_tag_length` function definitions in `serializable_types.rs` rather than at the point of use in `header_auth.rs`. The annotation should be at the `read_vec` call where the length is actually enforced for this specific context (header auth), not at the generic helper.

Also: the "interpreted as bytes" annotations should use `type=implication` with a `reason=` since the byte interpretation is structural (Rust's `Vec<u8>` type), not runtime-testable. This matches the pattern used in `body.rs`.

## Quote Mismatch Found

The v1 authentication tag implementation annotation in `header_auth.rs` (line 17-22) has "is 1.0," (with comma) but the TOML quote has "is 1.0" (no comma). This needs to be fixed. The test file has the correct quote.
