# Pre-Implementation Reasoning — types (Plaintext Length Bound)

## 1. Logical steps in this spec section

1. `EncryptInput` takes `plaintext: &'a [u8]` — always known length. No `plaintext_length_bound` field exists.
2. `EncryptStreamInput` takes `data_size: Option<usize>` — serves as plaintext length bound for unknown-length streams.
3. Three missing annotations need to be added as `type=implication`.

## 2. Point of fulfillment for each requirement

- **Req 1 (MUST NOT use/MUST be ignored)**: Fulfilled at `pub struct EncryptInput<'a>` — by construction, `EncryptInput` has no `plaintext_length_bound` field, so it's impossible to specify both known-length plaintext and a length bound.
- **Req 2 (SHOULD ensure by construction)**: Fulfilled at `pub struct EncryptInput<'a>` — same reasoning, the type system prevents it.
- **Req 3 (MAY input Plaintext Length Bound)**: Fulfilled at `pub data_size: Option<usize>` on `EncryptStreamInput` — this is the field that accepts the optional length bound for streaming (unknown-length) input.

## 3. Sub-items?

No sub-items. Each requirement is a single statement.

## 4. Reviewer readability

- Reqs 1 and 2 go on the `EncryptInput` struct definition, grouped with existing `encrypt.md#input` annotations.
- Req 3 goes on the `data_size` field of `EncryptStreamInput`.
- All are `type=implication` with `reason=` lines. No code restructuring needed.

## 5. Existing similar code

- `types.rs` already has `type=implication` + `reason=` annotations on `EncryptInput.source` and `EncryptInput.max_encrypted_data_keys`. Follow that exact pattern.
