# Pre-Implementation Reasoning

## 1. Logical steps in this spec section

1. `DecryptInput` struct accepts an optional `encryption_context` field — needs annotation
2. `DecryptInput::validate()` validates exactly one keyring/CMM — needs annotation
3. `DecryptInput::validate()` fails if validation fails — needs annotation

## 2. Point of fulfillment for each requirement

- "MUST validate that exactly one keyring or CMM" → fulfilled at `pub(crate) fn validate()` method signature (the method IS the validation)
- "MUST fail" → fulfilled at `Err(val_err(...))` inside validate
- "MUST accept an optional Encryption Context" → fulfilled at `pub encryption_context: EncryptionContext` field

## 3. Sub-items?

No sub-items requiring individual annotation. Each requirement is standalone.

## 4. Reviewer readability

- `encryption_context` annotation goes directly above the field — obvious
- validate annotations go before the method — matches EncryptInput pattern exactly
- Test annotations go inside test_bad_decrypt_input — close to assertions

## 5. Existing similar code

- `EncryptInput::validate()` at types.rs ~line 282 — exact same pattern with annotations
- `EncryptInput` struct at types.rs ~line 185 — shows `type=implication` for "MUST accept" field annotations
- `DecryptInput` struct already has field-level annotations (not struct-level like EncryptInput)

## Cross-reference analysis

- Req 3 quote: `- The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.`
  - Contains link `[Encryption Context](#encryption-context)` → `decrypt.md#encryption-context`
  - TOML at `compliance/.../decrypt/encryption-context.toml` has only a MAY quote about output, not relevant to the input field
  - The `#encryption-context` link is an internal anchor to the same page's section, not a cross-spec reference requiring a separate annotation. The encryption-context TOML describes output behavior, not input acceptance. No cross-reference annotation needed.

## Annotation type decisions

- Req 1 (validate): `implementation` (default) — testable by calling validate with None source
- Req 2 (fail): `implementation` (default) — testable by asserting error
- Req 3 (encryption_context field): `type=implication` — structural field existence, not runtime-testable. Matches EncryptInput pattern.
