# Agent 3 Review Notes — types (DecryptInput annotations)

## Adversarial Pre-Review (Step 2)

### 1. Per-Annotation Challenge: Does the next line actually implement THAT requirement?

**Annotation A** (types.rs, on `encryption_context` field):
```
//= specification/client-apis/decrypt.md#input
//= type=implication
//# - The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.
pub encryption_context: EncryptionContext,
```
- The requirement says the input MUST accept an optional EC argument.
- The code line declares the `encryption_context` field on `DecryptInput`.
- This is a structural implication — the field's existence IS the acceptance.
- **Verdict**: PASS. The field declaration directly fulfills "MUST accept an optional EC argument."

**Annotation B** (types.rs, on `validate()`):
```
//= specification/client-apis/decrypt.md#input
//# The Decrypt operation MUST validate that exactly one keyring or CMM was provided by the caller.
//= specification/client-apis/decrypt.md#input
//# If the caller does not provide exactly one of a keyring or CMM, the Decrypt operation MUST fail.
pub(crate) fn validate(&self) -> Result<(), Error> {
```
- Requirement 1: "MUST validate that exactly one keyring or CMM was provided"
- Requirement 2: "MUST fail if not exactly one"
- The code: `validate()` checks `self.source.is_none()` and returns `Err` if so.
- **Concern**: There are 2 annotations stacked before `pub(crate) fn validate()`. This is within the 2-annotation limit (hard limit is 3+).
- **Concern**: This mirrors the EXACT pattern used by `EncryptInput::validate()` at lines 282-287. The existing codebase already has this same 2-annotation stack for the encrypt side.
- **Verdict**: PASS. Two annotations before one function is acceptable (under the 3-stack hard limit), and this matches the established pattern.

### 2. Annotation Stacking Check

- types.rs `validate()`: 2 annotations before `pub(crate) fn validate()` — ACCEPTABLE (under 3-stack limit)
- types.rs `encryption_context`: 1 annotation before `pub encryption_context` — FINE
- test file: 2 annotations before `assert!(bad_decrypt_output.is_err())` — ACCEPTABLE

**However**, I note that the `ciphertext` field already has a 2-annotation stack:
```
//= specification/client-apis/decrypt.md#input
//# - The input to the Decrypt operation MUST accept a required [Encrypted Message](#encrypted-message) argument.
//= specification/client-apis/decrypt.md#encrypted-message
//# The input encrypted message MUST be a sequence of bytes in the
//# [message format](../data-format/message.md) specified by the AWS Encryption SDK.
pub ciphertext: &'a [u8],
```
This is pre-existing, not introduced by Agent 2.

### 3. Context Reset Evaluation (per-block isolation)

**Block 1**: encryption_context annotation
- Annotation: "MUST accept an optional Encryption Context argument" (type=implication)
- Code: `pub encryption_context: EncryptionContext,`
- In isolation: Immediately obvious. The field IS the acceptance of the argument.
- **PASS**

**Block 2**: validate() annotations
- Annotation 1: "MUST validate that exactly one keyring or CMM was provided"
- Annotation 2: "MUST fail if not exactly one"
- Code: `pub(crate) fn validate(&self) -> Result<(), Error> { if self.source.is_none() { Err(...) } else { Ok(()) } }`
- In isolation: The function is named `validate`, returns `Result`, and checks `source.is_none()`. The connection is clear.
- **PASS**

**Block 3**: test annotations
- Annotation 1: "MUST validate that exactly one keyring or CMM was provided" (type=test)
- Annotation 2: "MUST fail if not exactly one" (type=test)
- Code: `assert!(bad_decrypt_output.is_err());`
- In isolation: The assertion checks that the output is an error. Looking up, `decrypt_input.source = None` was set and `decrypt()` was called. The test verifies the failure.
- **PASS**

### 4. Semantic Relationship Check

All annotations have clear semantic relationships to their code lines:
- EC field acceptance → field declaration
- Validation requirement → validate function
- Failure requirement → validate function (returns Err)
- Test annotations → assertion that decrypt fails

### 5. Spec Sub-Items

The spec's input section has a list of required and optional arguments. Each is annotated individually at the corresponding field or function. The sub-items that Agent 2 was asked to annotate are all covered.

### 6. Code Structure vs Spec Structure

The spec describes: (1) required args, (2) validation, (3) failure, (4) optional args.
The code has: struct fields for args, validate() method for validation/failure.
Structure matches.

### 7. Top-to-Bottom Readability

Reading types.rs top-to-bottom for DecryptInput:
1. `ciphertext` field — annotated with encrypted message requirement ✅
2. `encryption_context` field — annotated with optional EC requirement ✅ (NEW)
3. `source` field — annotated with CMM/keyring requirement ✅
4. `validate()` method — annotated with validation + failure requirements ✅ (NEW)

Linear and clear.

## Anti-Rationalization Check (Step 3)

Reviewing my notes for "but" patterns:
- I noted the 2-annotation stack on validate() "but" said it's under the limit. This is correct — the hard limit is 3+, and 2 is explicitly acceptable.
- I noted the test annotations are after `let bad_decrypt_output = decrypt(&decrypt_input).await;` rather than before it. Wait — let me re-examine this.

**Re-examination of test annotation placement:**
The test annotations are AFTER the `decrypt()` call and BEFORE the `assert!()`. The annotations are `type=test`. The assertion is the fulfillment point — it's where the test VERIFIES the requirement. The `decrypt()` call is the setup; the `assert!` is the verification. Placing test annotations before the assertion is correct per Pattern 2 (positive behavior — before the fulfilling code). The fulfilling code for a test is the assertion.

No rationalization issues found.

## Cross-Reference Check

Annotation quote: `- The input to the Decrypt operation MUST accept an optional [Encryption Context](#encryption-context) argument.`
- Contains link: `[Encryption Context](#encryption-context)` → `decrypt.md#encryption-context`
- The TOML for `decrypt.md#encryption-context` only has a MAY about output, not about input acceptance.
- No cross-reference annotation needed — the linked section doesn't have a requirement that applies to this code location.
- **PASS**

## Potential Spec Gaps

None identified. The implementation is minimal annotation-only work.

## Test Validation Results

- **Check 1 (Tests)**: KMS-dependent integration tests fail due to missing credentials. Non-KMS tests pass. All failures are pre-existing (in `test_authentication_tag.rs`), not in files modified by Agent 2.
- **Check 2 (Coverage)**: No check logs available. Duvet snapshot confirms all 3 requirements are covered.
- **Check 3 (Duvet Report)**: PASS — `make duvet` succeeds, snapshot shows all 3 requirements annotated.
- **Check 4 (Snapshot)**: Snapshot changed (expected — new annotations added).
- **Check 5 (Linter)**: `cargo clippy -- -D warnings` fails due to pre-existing `missing_docs` warning on `encrypt_stream` function. Not related to Agent 2's changes.
