# Agent 1 Notes — decrypt.rs Gap Analysis

## Full Gap Inventory for `decrypt.md` in `decrypt.rs`

### `#behavior` section (5 missing MUST annotations in decrypt.rs)

| Requirement | Status | Location |
|---|---|---|
| Step 1 MUST be Parse the header | ✅ annotated | line 133 |
| Step 2 MUST be Get the decryption materials | ✅ annotated | line 137 |
| Step 3 MUST be Verify the header | ✅ annotated | line 149 |
| Step 4 MUST be Decrypt the message body | ✅ annotated | line 156 |
| Step 5 MUST be Verify the signature | ✅ annotated | line 161 |
| MUST perform this step (signature present) | ✅ annotated | line 163 |
| **MUST NOT perform this step (no signature)** | ❌ MISSING | step_verify_signature |
| **all output MUST NOT be released until after these steps complete** | ❌ MISSING | decrypt() fn structure |
| **Output MUST NOT be released until otherwise indicated** | ❌ MISSING | streaming path |
| **MUST halt and indicate a failure** | ❌ MISSING | error returns |
| consumable bytes MUST fail | ✅ annotated | line 88 |
| **MUST provide a configuration option** (fail on signed header) | ❌ MISSING | ProtectionNeeded / i_accept_the_danger |

### `#get-the-decryption-materials` section (3 missing in decrypt.rs)

| Requirement | Status | Location |
|---|---|---|
| commitment policy not supported MUST yield error | ✅ annotated | line 212 |
| MUST obtain decryption materials | ✅ annotated (materials.rs) | materials.rs line 465 |
| CMM used MUST be input CMM | ✅ annotated | line 206 |
| MUST construct default CMM from keyring | ✅ annotated | line 206 |
| Call MUST be constructed as follows | ✅ annotated (materials.rs) | materials.rs line 470 |
| Encryption Context MUST be parsed | ✅ annotated (materials.rs) | materials.rs line 487 |
| Algorithm Suite ID MUST be parsed | ✅ annotated (materials.rs) | materials.rs line 475 |
| Encrypted Data Keys MUST be parsed | ✅ annotated (materials.rs) | materials.rs line 483 |
| Reproduced EC MUST be input EC | ✅ annotated (materials.rs) | materials.rs line 491 |
| Commitment Policy MUST be configured | ✅ annotated (materials.rs) | materials.rs line 480 |
| Data key MUST be derived from plaintext data key | ✅ annotated | line 252 |
| Algorithm suite MUST be from decryption materials | ✅ annotated | line 232 |
| **Algorithm suite not supported for ESDK MUST yield error** | ❌ MISSING | no code in decrypt.rs |
| **Algorithm suite not supported by commitment policy MUST yield error** | ❌ MISSING | validate_commitment_policy_on_decrypt call exists but no annotation |
| Commit key MUST be derived | ✅ annotated | line 265 |
| Derived commit key MUST equal stored | ✅ annotated | line 270 |
| Key derivation algorithm MUST be used | ✅ annotated | line 235 |
| **Identity KDF derived key MUST be same as plaintext** | ❌ MISSING | handled in key_derivation.rs but not annotated for decrypt |

### `#input` section (3 missing, but in types.rs not decrypt.rs)

| Requirement | Status | Location |
|---|---|---|
| MUST accept Encrypted Message | ✅ annotated | types.rs line 376 |
| MUST accept CMM and keyring | ✅ annotated | types.rs line 385 |
| **MUST validate exactly one keyring or CMM** | ❌ MISSING | types.rs validate() |
| **MUST fail if not exactly one** | ❌ MISSING | types.rs validate() |
| **MUST accept optional Encryption Context** | ❌ MISSING | types.rs encryption_context field |

## Spec-Aligned Structure Analysis

### Q1: What is the spec section's logical flow?
The `#behavior` section defines:
1. Five ordered steps (parse → materials → verify header → decrypt body → verify signature)
2. Conditional execution of step 5 based on signature algorithm presence
3. Output release constraints (non-streaming vs streaming)
4. Configuration option for early failure on signed suites

### Q2: Where will each requirement be fulfilled in code?
- Steps 1-5 ordering → sequential calls in `internal_decrypt`
- Signature step conditional → `if state.dec_mat.verification_key.is_some()` guard in `step_verify_signature`
- Output release (non-streaming) → `decrypt()` function structure (returns only after all steps)
- Configuration option → `ProtectionNeeded` enum / `i_accept_the_danger` field

### Q3: Sub-items under normative requirements?
The behavior section has a list of streaming constraints that are sub-items. Each should be annotated individually.

### Q4: Most likely structural mistake?
Annotating the "MUST NOT perform" at the call site instead of at the guard. The call site always calls `step_verify_signature` — the skip happens inside the function.

## Potential Spec Gaps

### 1. `build_encryption_context_to_only_authenticate` has no spec coverage in decrypt.md
- **Code location**: `decrypt.rs` line 462-470
- **Behavior**: Filters decryption materials' encryption context to only keys in `required_encryption_context_keys`
- **Why it matters**: Correctness — this filtering determines what goes into the AAD for header verification
- **Note**: The code has a TODO comment "TODO Post-#619: Duvet this section". The `#verify-the-header` section does describe this behavior, and there IS an annotation for it at line 305. But the helper function itself is not annotated.
- **Suggested spec requirement**: Already covered by verify-the-header section; the TODO is about adding the annotation to the helper function.

### 2. Net v4.0.0 retry policy has no spec coverage
- **Code location**: `decrypt.rs` lines 351-375
- **Behavior**: If header auth fails and `NetV400RetryPolicy::AllowRetry` is set, re-derives keys with a different method and retries header verification
- **Why it matters**: Interop — this handles a compatibility issue with ESDK .NET v4.0.0
- **Note**: The code has a TODO comment "TODO Post-#619: Add to the ESDK Specification"
- **Suggested spec requirement**: "If header verification fails and the client is configured to allow .NET v4.0.0 retry, the decrypt operation SHOULD re-derive keys using the .NET v4.0.0 key derivation method and retry header verification."

### 3. `ProtectionNeeded` enum enforces streaming safety not fully described in spec
- **Code location**: `decrypt.rs` lines 26-40
- **Behavior**: When `ProtectionNeeded::Yes` and the message has a verification key and is multi-frame, the operation fails to prevent releasing unverified data
- **Why it matters**: Security — prevents partial plaintext release before signature verification
- **Note**: This maps to the spec's "MUST provide a configuration option" requirement but the implementation details (enum, flag name) are not in the spec.
