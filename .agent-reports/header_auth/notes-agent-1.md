# Agent 1 Notes — header_auth

## Spec-Aligned Structure Analysis

### Q1: What is the spec section's logical flow?

The `data-format/message-header.md#authentication-tag` section describes:
1. The authentication tag is the authentication value for the header
2. Its length MUST equal the algorithm suite's authentication tag length
3. It MUST be interpreted as bytes
4. The algorithm suite determines how it's calculated (links to encrypt.md)
5. It's used to authenticate header contents during decryption (links to decrypt.md)

### Q2: Where will each requirement be fulfilled in code?

- Length requirement → `read_vec(r, get_tag_length(suite) as usize, raw)?` — the `get_tag_length(suite)` enforces the length constraint
- "Interpreted as bytes" → `Vec<u8>` type of `header_auth_tag` field in `HeaderAuth::AESMac` — structural property

### Q3: Does the spec contain sub-items?

No sub-items for the `#authentication-tag` section. It's two independent MUST requirements.

### Q4: What is the most likely structural mistake?

The implementer might be tempted to add the "interpreted as bytes" annotation at the `read_vec` call.
But the correct placement is at the same `read_vec` line (or the `let header_auth_tag = ...` line)
since that's where the bytes are materialized as `Vec<u8>`.
The existing IV "interpreted as bytes" annotation is placed at the `let header_iv = read_vec(...)` line,
so the auth tag one should follow the same pattern.

## Potential Spec Gaps

No meaningful behaviors found in `header_auth.rs` that lack spec coverage.
The code is straightforward serialization/deserialization with no extra constraints beyond what the spec describes.

## Annotation Prefix Inconsistency (Advisory)

The `header_auth.rs` file uses TWO different spec path prefixes:
- `specification/` for `client-apis/encrypt.md` references
- `aws-encryption-sdk-specification/` for `data-format/message-header.md` references

The local duvet config (`.duvet/config.toml`) uses `specification/` as the spec path prefix.
The root `make duvet` compliance TOML uses `aws-encryption-sdk-specification/`.

This means:
- `specification/` annotations match the LOCAL duvet report
- `aws-encryption-sdk-specification/` annotations match the ROOT `make duvet` report

The existing annotations in `header_auth.rs` for `data-format/message-header.md` use `aws-encryption-sdk-specification/` prefix,
so the new annotation should use the same prefix for consistency within this file.
