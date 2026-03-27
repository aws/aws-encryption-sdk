# Agent 2 Notes — client.md#initialization annotations

## Pre-Implementation Reasoning

### 1. Logical steps in this spec section

1. Caller MUST have option to provide commitment policy → field exists on input structs
2. Caller MUST have option to provide max EDKs → field exists on input structs
3. Default commitment policy MUST be REQUIRE_ENCRYPT_REQUIRE_DECRYPT → `#[default]` on enum variant
4. Default max EDKs MUST be no limit → `Option<NonZeroUsize>` defaults to `None`
5. Commitment policy SHOULD be immutable → Rust ownership model (structural)

### 2. Point of fulfillment for each requirement

- Req 1 ("option to provide commitment policy"): `pub commitment_policy: EsdkCommitmentPolicy` field on `EncryptInput`
- Req 2 ("option to provide max EDKs"): `pub max_encrypted_data_keys: Option<NonZeroUsize>` field on `EncryptInput`
- Req 3 ("default no limit"): Same field — `Option<NonZeroUsize>` defaults to `None` via `#[derive(Default)]`
- Req 4 ("default REQUIRE_ENCRYPT_REQUIRE_DECRYPT"): Already annotated in `decrypt.rs` at the `use` statement. Needs `type=test` only.

### 3. Sub-items?

No sub-items. Each requirement is a standalone statement.

### 4. Reviewer readability

Annotations go directly on the struct fields in `types.rs`. Tests go in `test_create_esdk_client.rs`. Straightforward.

### 5. Existing similar code

- `decrypt.rs` lines 15-21 already have `client.md#initialization` annotations with `type=implication` for Req 4 and the SHOULD.
- `types.rs` has `encrypt.md#input` annotations with `type=implication` on `EncryptInput` struct fields — same pattern.

### Cross-reference analysis

- Req 1 quote contains `[commitment policy](#commitment-policy)` — self-referential anchor within client.md, no cross-ref needed.
- Req 2 quote contains `[maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys)` — self-referential, no cross-ref needed.
- Req 3 quote contains `[maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys)` and `[message format](../format/message-header.md)` — the message-header link could warrant a cross-ref, but there's no specific requirement in message-header about "no limit" that maps to this code. The annotation is about the *default* being no limit, not about the message format limit itself. No cross-ref needed.
- Req 4 quote contains `[commitment policy](#commitment-policy)` and `[REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt)` — the algorithm-suites link is to the MPL spec, not tracked by this project's duvet config. No cross-ref needed.

### Decision: annotate on EncryptInput only

The work item says to annotate on `EncryptInput`. The spec says "client initialization" which maps to constructing the input struct. Annotating on `EncryptInput` is sufficient — the same fields exist on all input structs, and `EncryptInput` is the primary one.
