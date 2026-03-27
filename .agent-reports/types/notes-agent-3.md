# Agent 3 Notes — types (encrypt.md#input + decrypt.md#input)

## Round 2 — Adversarial Pre-Review

### 1. Per-Annotation Challenge

**types.rs — EncryptInput struct (pre-existing, 5 stacked implication blocks):**
These 5 annotations on `pub struct EncryptInput<'a>` are PRE-EXISTING, not added by Agent 2. They are stacked (5 blocks before one `pub struct` line). This is a pre-existing issue — not blocking for this review.

**types.rs — EncryptInput.source (NEW — Req 3):**
Quote: "The keyring and CMM inputs SHOULD be optional."
Code: `pub source: Option<MaterialSource>`
Verdict: PASS. The `Option<>` wrapper IS the optionality. The `reason=` line explains the connection. Self-evident.

**types.rs — DecryptInput.source (NEW — Req 11):**
Quote: "The keyring and CMM inputs SHOULD be optional."
Code: `pub source: Option<MaterialSource>`
Verdict: PASS. Same reasoning as above.

Note: On DecryptInput.source, there are now 2 annotation blocks before the field:
1. `decrypt.md#input` — "MUST accept a CMM and keyring argument" (pre-existing)
2. `decrypt.md#input` — "SHOULD be optional" (NEW)
This is 2 blocks, under the 3+ hard limit. PASS.

**test_create_esdk_client.rs — test_encrypt_input_accepts_plaintext:**
Quote: "The input to the Encrypt operation MUST accept a required [plaintext](#plaintext) argument."
Code: Creates `EncryptInput`, sets `plaintext`, asserts value.
Verdict: PASS. The test directly demonstrates the struct accepts plaintext.

**test_create_esdk_client.rs — test_encrypt_input_accepts_cmm_and_keyring:**
Quote: "The input to the Encrypt operation MUST accept a [cryptographic Materials Manager (CMM)]... and a [keyring]... argument."
Code: Creates `EncryptInput`, asserts `source.is_none()`.
Verdict: PASS. The test demonstrates the `source` field exists. The field type `Option<MaterialSource>` accepts both CMM and keyring variants.

**test_create_esdk_client.rs — test_encrypt_input_accepts_optional_algorithm_suite:**
Quote: "The input to the Encrypt operation MUST accept an optional [Algorithm Suite]... argument."
Code: Creates `EncryptInput`, asserts `algorithm_suite_id.is_none()`.
Verdict: PASS. Demonstrates the optional field exists and defaults to None.

**test_create_esdk_client.rs — test_encrypt_input_accepts_optional_encryption_context:**
Quote: "The input to the Encrypt operation MUST accept an optional [Encryption Context]... argument."
Code: Creates `EncryptInput`, asserts `encryption_context.is_empty()`.
Verdict: PASS. Demonstrates the field exists.

**test_create_esdk_client.rs — test_encrypt_input_accepts_optional_frame_length:**
Quote: "The input to the Encrypt operation MUST accept an optional [Frame Length]... argument."
Code: Creates `EncryptInput`, sets `frame_length`, asserts value.
Verdict: PASS. Demonstrates the field exists and can be set.

**test_create_esdk_client.rs — test_decrypt_input_accepts_encrypted_message:**
Quote: "The input to the Decrypt operation MUST accept a required [Encrypted Message]... argument."
Code: Creates `DecryptInput`, sets `ciphertext`, asserts value.
Verdict: PASS. Directly demonstrates the struct accepts ciphertext.

**test_create_esdk_client.rs — test_decrypt_input_accepts_cmm_and_keyring:**
Quote: "The input to the Decrypt operation MUST accept a [cryptographic Materials Manager (CMM)]... and a [keyring]... argument."
Code: Creates `DecryptInput`, asserts `source.is_none()`.
Verdict: PASS. Same reasoning as encrypt counterpart.

**test_create_esdk_client.rs — test_decrypt_input_accepts_optional_encryption_context:**
Quote: "The input to the Decrypt operation MUST accept an optional [Encryption Context]... argument."
Code: Creates `DecryptInput`, asserts `encryption_context.is_empty()`.
Verdict: PASS. Demonstrates the field exists.

**test_encrypt_decrypt.rs — test_bad_encrypt_input (2 NEW annotations):**
Quote 1: "The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller."
Quote 2: "If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail."
Code: `assert!(encrypt_output.is_err())`
Verdict: PASS. The test sets `source = None` and asserts error. Both annotations relate to the same assertion — the validation and the failure are tested by the same assertion. 2 blocks before one line — under the 3+ hard limit.

### 2. Annotation Stacking Check

- EncryptInput.source: 1 NEW block. PASS.
- DecryptInput.source: 2 blocks total (1 pre-existing + 1 new). Under 3+ limit. PASS.
- test_bad_encrypt_input: 2 NEW blocks before `assert!`. Under 3+ limit. PASS.
- All new test functions: 1 block each. PASS.

Pre-existing stacking issue: EncryptInput struct has 5 implication blocks stacked. NOT in scope for this review (pre-existing).

### 3. Per-Block Isolation (Context Reset)

Each new annotation block evaluated in complete isolation:

- EncryptInput.source SHOULD optional: "keyring and CMM inputs SHOULD be optional" → `pub source: Option<MaterialSource>` — immediately obvious with reason line.
- DecryptInput.source SHOULD optional: Same. PASS.
- Each test function: annotation quote describes what the struct must accept → test constructs struct and verifies field. Immediately obvious in every case.
- test_bad_encrypt_input validate: "MUST validate exactly one" → `assert!(encrypt_output.is_err())` after setting source=None. Clear.
- test_bad_encrypt_input fail: "MUST fail" → same assertion. Clear.

### 4. Semantic Relationship Check

All annotations semantically match their code lines. PASS.

### 5. Sub-Item Check

No sub-items in these requirements. Each is standalone. PASS.

### 6. Structure Mirror Check

The spec lists required and optional arguments → the code has struct fields for each → tests verify each field. PASS.

### 7. Linear Readability

Both source files read top-to-bottom with clear annotation-to-code mapping. PASS.

## Anti-Rationalization Check

Reviewed my reasoning above. No "but" patterns found. No instances of identifying a problem and then talking myself out of flagging it.

One observation: The 5-stack on EncryptInput struct is a pre-existing issue. I am NOT rationalizing this away — it genuinely is not Agent 2's change. Agent 2 only added annotations on the `source` field inside the struct, not on the struct declaration.

## Cross-Reference Analysis

Links found in annotation quotes:
- `[plaintext](#plaintext)` — same-document anchor. No cross-ref needed.
- `[cryptographic Materials Manager (CMM)](../framework/cmm-interface.md)` — cross-spec link. However, this links to the CMM interface definition, not a requirement that needs annotation at this code location. The annotation is about the INPUT accepting the argument, not about the CMM interface itself.
- `[keyring](../framework/keyring-interface.md)` — same reasoning as above.
- `[Algorithm Suite](#algorithm-suite)` — same-document anchor.
- `[Encryption Context](#encryption-context)` — same-document anchor.
- `[Frame Length](#frame-length)` — same-document anchor.
- `[Encrypted Message](#encrypted-message)` — same-document anchor.

Cross-ref ratio: 0 actionable cross-refs / 0 needed = N/A. The cross-spec links to cmm-interface.md and keyring-interface.md are definitional references, not requirements that need annotation at the struct field level.

## Quote Verification

Verified each annotation quote against the TOML files character-by-character:

### encrypt/input.toml:
- ✅ "- The input to the Encrypt operation MUST accept a required [plaintext](#plaintext) argument." — exact match
- ✅ "- The input to the Encrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument." — exact match
- ✅ "The keyring and CMM inputs SHOULD be optional." — exact match
- ✅ "The Encrypt operation MUST validate that exactly one keyring or CMM was provided by the caller." — exact match
- ✅ "If the caller does not provide exactly one of a keyring or CMM, the Encrypt operation MUST fail." — exact match
- ✅ "- The input to the Encrypt operation MUST accept an optional [Algorithm Suite](#algorithm-suite) argument." — exact match
- ✅ "- The input to the Encrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument." — exact match
- ✅ "- The input to the Encrypt operation MUST accept an optional [Frame Length](#frame-length) argument." — exact match

### decrypt/input.toml:
- ✅ "- The input to the Decrypt operation MUST accept a required [Encrypted Message](#encrypted-message) argument." — exact match
- ✅ "- The input to the Decrypt operation MUST accept a [cryptographic Materials Manager (CMM)](../framework/cmm-interface.md) and a [keyring](../framework/keyring-interface.md) argument." — exact match
- ✅ "The keyring and CMM inputs SHOULD be optional." — exact match
- ✅ "- The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument." — exact match

## Test Validation

- Check 1 (Tests): PASS — 15/15 tests pass in test_create_esdk_client, 1/1 test_bad_encrypt_input passes
- Check 2 (Coverage): N/A — no pre-spawn hook logs
- Check 3 (Duvet Report): PASS — duvet report generates, snapshot shows all requirements now have test coverage
- Check 4 (Snapshot): Snapshot changed — expected, as new annotations were added
- Check 5 (Linter): PASS — clippy passes (pre-existing warnings only in unmodified files)

Pre-existing failures: 8 tests in test_authentication_tag.rs (AWS credential issue), 5 tests in test_encrypt_decrypt.rs (same credential issue). None related to Agent 2's changes.

## Potential Spec Gaps

None identified.
