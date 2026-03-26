# Agent 1 Notes — types.rs Gap Analysis

## Spec-Aligned Structure Analysis

### 1. What is the spec section's logical flow?

`decrypt.md#input` describes:
1. Required arguments: encrypted message, CMM/keyring
2. Validation: exactly one keyring or CMM must be provided, fail otherwise
3. Optional arguments: encryption context

The code in `DecryptInput` follows this flow:
- Struct fields define the required/optional arguments
- `validate()` method enforces the exactly-one constraint

### 2. Where will each requirement be fulfilled in code?

- "MUST accept a required [Encrypted Message]" → `pub ciphertext: &'a [u8]` field (ALREADY ANNOTATED)
- "MUST accept a [CMM] and [keyring]" → `pub source: Option<MaterialSource>` field (ALREADY ANNOTATED)
- "MUST validate that exactly one keyring or CMM" → `DecryptInput::validate()` method (MISSING ANNOTATION)
- "MUST fail" if not exactly one → `Err(val_err(...))` in validate (MISSING ANNOTATION)
- "MUST accept an optional [Encryption Context]" → `pub encryption_context: EncryptionContext` field (MISSING ANNOTATION)

### 3. Sub-items under normative requirements?

The input section has a list of required and optional arguments. Each is a separate `[[spec]]` entry in the TOML.

### 4. Most likely structural mistake?

The implementer might annotate the `encryption_context` field on `DecryptInput` struct but forget that the annotation must be `type=implication` since it's a structural property (the field exists, making it accepted). Same pattern as `EncryptInput`.

## Potential Spec Gaps

### DecryptInput.validate() only checks for None, not "exactly one"

- **Code location**: `DecryptInput::validate()` in `types.rs` line ~488
- **Behavior**: The validate method only checks `self.source.is_none()`. It does not check for "exactly one" — the `MaterialSource` enum already ensures only one variant is active. However, the spec says "exactly one keyring or CMM" which implies mutual exclusivity. The `MaterialSource` enum enforces this by construction.
- **Why it matters**: Correctness — the spec requirement is satisfied by the type system (MaterialSource is an enum, not two separate Option fields), but this is non-obvious.
- **Suggested spec requirement**: No change needed — the current spec wording is fine, but the annotation should include a `reason=` explaining that MaterialSource enum enforces mutual exclusivity by construction.
