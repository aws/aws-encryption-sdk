# Agent 2 Notes — header_auth

## Step 1.5: Pre-Implementation Reasoning

### 1. Logical steps in this spec section

1. V1 Header Authentication serialization: write IV, then Authentication Tag (in order)
2. V2 Header Authentication serialization: write Authentication Tag only

### 2. Point of fulfillment for each requirement

- **Req 1** ("V1 Header Authentication MUST be serialized as, in order, IV, and Authentication Tag"):
  Fulfilled by the match arm body in `write_header_auth_tag_v1` — the two `write_bytes` calls in sequence (IV first, then auth tag).
  The annotation goes at the top of the match arm body as a "general behavior" annotation (Pattern 3).

- **Req 2** ("V2 Header Authentication MUST be serialized as the Authentication Tag only"):
  Fulfilled by the single `write_bytes(w, header_auth_tag)` call in `write_header_auth_tag_v2`.
  The annotation goes immediately before that call.

### 3. Sub-items?

No sub-items. The V1 requirement mentions "IV, and Authentication Tag" but these are part of the normative quote itself, not separate list items in the TOML.

### 4. Can a reviewer read this top-to-bottom?

Yes. Each function is small (~10 lines). The V1 annotation goes at the top of the match arm, before the existing client-apis annotations. The V2 annotation goes before the `write_bytes` call. Both are immediately obvious.

For V2, the current code uses an inline expression style for the match arm. Need to reformat to a block body so the annotation can be placed before the executable `write_bytes` line.

### 5. Existing similar code

- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v1_header_body.rs` — has data-format annotations alongside client-apis annotations
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs` — round-trip test pattern with raw AES keyring
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_v1_header_body.rs` — V1-specific test with non-committing algorithm suite, `encrypt_v1()` and `round_trip_v1()` helpers

### Cross-reference check

Neither requirement quote contains markdown links, so no cross-references needed.
