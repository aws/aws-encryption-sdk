# Discovery Notes — types.rs

## Spec-Aligned Structure Analysis (Step 6.8)

### 1. What is the spec section's logical flow?

**encrypt.md#input:**
1. Define required arguments (plaintext, CMM/keyring)
2. Define optional arguments (algorithm suite, encryption context, frame length)
3. Validate exactly one CMM or keyring provided
4. Handle Plaintext Length Bound (MAY/SHOULD)

**decrypt.md#input:**
1. Define required arguments (encrypted message, CMM/keyring)
2. Define optional arguments (encryption context)
3. Validate exactly one CMM or keyring provided

**client.md#initialization:**
1. Option to provide commitment policy
2. Option to provide max encrypted data keys
3. Default commitment policy = REQUIRE_ENCRYPT_REQUIRE_DECRYPT
4. Default max EDKs = no limit
5. Commitment policy SHOULD be immutable

### 2. Where will each requirement be fulfilled in code?

- "accept a required plaintext" → `pub plaintext: &'a [u8]` field on `EncryptInput`
- "accept CMM and keyring" → `pub source: Option<MaterialSource>` field + `MaterialSource` enum
- "SHOULD be optional" → `Option<MaterialSource>` type
- "validate exactly one" → `EncryptInput::validate()` method
- "MUST fail" → `Err(val_err(...))` return in validate
- "accept optional Algorithm Suite" → `pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>` field
- "accept optional Encryption Context" → `pub encryption_context: EncryptionContext` field
- "accept optional Frame Length" → `pub frame_length: FrameLength` field

### 3. Sub-items under normative requirements?

The encrypt.md#input section has a list of required and optional arguments.
Each list item is a separate `[[spec]]` entry in the TOML.
They are already annotated individually at the struct definition.

### 4. Most likely structural mistake?

The `implication` annotations on the struct fields are correct for "accept" requirements
(the struct field's existence IS the implementation).
The most likely mistake would be:
- Placing test annotations on the struct definition instead of on test code that exercises the struct
- Forgetting that `implication` type annotations satisfy both implementation and test checks in duvet,
  so these may already be passing

**Key insight**: Looking at the snapshot more carefully, the `!MUST` prefix on
`TEXT[!MUST,implication]` lines means the MUST-level check is NOT passing despite
having an `implication` annotation. This is unexpected because `implication` should
satisfy both implementation and test. This may indicate the duvet report was generated
with `--require-citations true --require-tests true` flags that treat `implication`
differently, OR the `!` simply indicates the requirement level.

After re-reading the snapshot format: the `!` prefix on the level means the requirement
is NOT fully satisfied. For `TEXT[!MUST,implication]`, the requirement has an implication
annotation but the MUST check is still failing. This likely means the duvet configuration
requires explicit `type=test` annotations even for `implication` types.

However, looking at the duvet-patterns.md: "Infrastructure requirements use `type=implication`,
which satisfies both the implementation and test checks (they are not runtime-testable)."

This contradicts the snapshot showing `!MUST` for implication-annotated requirements.
The most likely explanation is that the root Makefile's duvet_report does NOT use
`--require-citations true --require-tests true` (they're commented out as TODO),
so the `!MUST` in the snapshot may just be the requirement level indicator, not a failure flag.

**Resolution**: Since I cannot run duvet to verify, I'll focus on the clear gaps:
requirements that have `implementation` annotations but NO `test` annotations.

## Potential Spec Gaps

### 1. MaterialSource enum allows both CMM and Keyring but not simultaneously
- **Code location**: `MaterialSource` enum in types.rs
- **Behavior**: The enum design makes it impossible to provide both a CMM and a keyring simultaneously (they're variants, not separate fields)
- **Why it matters**: Correctness — the spec says "validate that exactly one keyring or CMM was provided" but the Rust type system already prevents providing both. The validate() method only checks for None (no source), not for "both provided".
- **Suggested spec requirement**: "If the implementation uses a sum type (enum/union) for the CMM/keyring input, the type system MAY enforce the 'exactly one' constraint, and the validation MUST at minimum ensure a value is provided."

### 2. EncryptInput has no Plaintext Length Bound field
- **Code location**: `EncryptInput` struct in types.rs
- **Behavior**: The `EncryptInput` struct has no `plaintext_length_bound` field. The spec says callers MAY input one.
- **Why it matters**: Interop — other implementations may support this feature
- **Note**: The `EncryptStreamInput` has a `data_size` field which may serve a similar purpose but is not the same as Plaintext Length Bound.
