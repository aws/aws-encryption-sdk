# Agent 1 Notes — header_auth.rs

## Spec-Aligned Structure Analysis

### 1. What is the spec section's logical flow?

The data-format spec defines two serialization formats:

- **V1 Header Authentication** (`header-authentication-version-1-0`): Serialized as IV followed by Authentication Tag (two fields, in order).
- **V2 Header Authentication** (`header-authentication-version-2-0`): Serialized as Authentication Tag only (single field).

The client-apis encrypt spec defines how these are populated:

- **V1 Authentication Tag** (`v1-authentication-tag`): Dispatch on message version 1.0, then serialize IV (padded to IV length with 0) and Authentication Tag.
- **V2 Authentication Tag** (`v2-authentication-tag`): Dispatch on message version 2.0, then serialize Authentication Tag only.

### 2. Where will each requirement be fulfilled in code?

- `data-format/message-header.md#header-authentication-version-1-0` → `write_header_auth_tag_v1()` function body (the match arm that writes IV then auth tag in order)
- `data-format/message-header.md#header-authentication-version-2-0` → `write_header_auth_tag_v2()` function body (the match arm that writes only auth tag)
- `client-apis/encrypt.md#v1-authentication-tag` (already annotated) → `write_header_auth_tag()` match arm for version 1
- `client-apis/encrypt.md#v2-authentication-tag` (already annotated) → `write_header_auth_tag()` match arm for version 2

### 3. Sub-items under normative requirements?

The V1 data-format requirement mentions "in order, IV, and Authentication Tag" — these map to the two `write_bytes` calls in `write_header_auth_tag_v1`.

The V2 data-format requirement says "Authentication Tag only" — maps to the single `write_bytes` call in `write_header_auth_tag_v2`.

### 4. Most likely structural mistake?

The implementer might be tempted to place the data-format annotation at the `match` dispatch in `write_header_auth_tag()` rather than inside the version-specific functions. The data-format requirements describe the serialization format itself (field order), so they belong inside `write_header_auth_tag_v1` and `write_header_auth_tag_v2` where the actual serialization happens.

## Potential Spec Gaps

### read_header_auth_tag_v2 synthesizes a zero IV

- **Code location**: `header_auth.rs`, `read_header_auth_tag_v2()`, line `let header_iv = vec![0u8; get_iv_length(suite) as usize];`
- **Behavior**: When deserializing V2 header auth, the code creates a zero-filled IV vector even though V2 doesn't serialize an IV. This synthetic IV is stored in the `HeaderAuth::AESMac` struct.
- **Why it matters (interop/correctness)**: The spec says V2 is "Authentication Tag only" but doesn't describe what the in-memory representation of the IV should be for V2. The code assumes zero IV for V2, which is consistent with the encrypt spec's "The IV MUST have a value of 0" requirement, but the data-format spec doesn't explicitly state this for deserialization.
- **Suggested spec requirement**: "When deserializing a V2 Header Authentication, the IV value SHOULD be treated as all zeros with length equal to the IV length of the algorithm suite."

## Coverage Gap Summary

| Spec Target | Requirement | Implementation | Test |
|---|---|---|---|
| `data-format/message-header.md#header-authentication-version-1-0` | V1 MUST be serialized as IV, then Auth Tag | MISSING | MISSING |
| `data-format/message-header.md#header-authentication-version-2-0` | V2 MUST be serialized as Auth Tag only | MISSING | MISSING |
| `client-apis/encrypt.md#v1-authentication-tag` | serialize v1 header auth | ✓ exists | MISSING |
| `client-apis/encrypt.md#v1-authentication-tag` | IV MUST have value of IV used, padded | ✓ exists | MISSING |
| `client-apis/encrypt.md#v1-authentication-tag` | Auth Tag MUST have value calculated | ✓ exists | MISSING |
| `client-apis/encrypt.md#v2-authentication-tag` | serialize v2 header auth | ✓ exists | MISSING |
| `client-apis/encrypt.md#v2-authentication-tag` | Auth Tag MUST have value calculated | ✓ exists | MISSING |

Priority: data-format implementation annotations are missing entirely — these are the highest priority gap for this file.
