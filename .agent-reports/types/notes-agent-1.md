# Agent 1 Notes — types.rs Discovery

## Discovery Process

### Step 2: Coverage Validation
- `make validate-all-tests` target does not exist in any Makefile. Skipped.

### Step 3: Infrastructure
- `design/requirements/infrastructure.md` does not exist. No infrastructure gaps to address.

### Step 4: Full Duvet Checks
- Unable to run `make duvet` due to shell restrictions. Analyzed existing duvet snapshot and compliance summary report instead.

### Step 5: Prioritization

Analyzed all requirements from scoped specs (`encrypt.md#input`, `decrypt.md#input`, `client.md`) against annotations in `types.rs`.

**Summary of coverage for types.rs:**

| Spec Section | Total Reqs | Covered | Gaps |
|---|---|---|---|
| `encrypt.md#input` | 11 | 8 | 3 (Plaintext Length Bound) |
| `decrypt.md#input` | 6 | 6 | 0 |
| `client.md#initialization` | 5 | 5 | 0 |
| `client.md#commitment-policy` | 1 | 1 | 0 |
| `client.md#encrypt` | 1 | 1 (in encrypt.rs) | 0 for types.rs |
| `client.md#decrypt` | 1 | 1 (in decrypt.rs) | 0 for types.rs |

The 3 missing annotations are all for Plaintext Length Bound requirements in `encrypt.md#input`.

### Compliance Report Staleness Note
The compliance summary report (`compliance_summary_report.html`) appears to be stale — it shows "no implementation found" for several `decrypt.md#input` requirements that DO have annotations in the current `types.rs`. The duvet snapshot (`.duvet/snapshot.txt`) is more reliable and shows these requirements as covered.

## Spec-Aligned Structure Analysis

### encrypt.md#input logical flow:
1. Required arguments (plaintext, CMM/keyring) → struct fields
2. Optional arguments (algorithm suite, encryption context, frame length) → struct fields
3. Validation (exactly one CMM/keyring) → `validate()` method
4. Plaintext Length Bound (streaming only) → `EncryptStreamInput.data_size`
5. Construction constraints (can't specify both known-length and bound) → type system

### Where each requirement is fulfilled:
- "MUST accept a required plaintext" → `EncryptInput.plaintext: &'a [u8]`
- "MUST accept CMM and keyring" → `EncryptInput.source: Option<MaterialSource>`
- "SHOULD be optional" → `Option<MaterialSource>` type
- "MUST validate exactly one" → `EncryptInput::validate()` method
- "MUST fail if not exactly one" → `Err(val_err(...))` in `validate()`
- "MUST accept optional Algorithm Suite" → `EncryptInput.algorithm_suite_id: Option<...>`
- "MUST accept optional Encryption Context" → `EncryptInput.encryption_context`
- "MUST accept optional Frame Length" → `EncryptInput.frame_length`
- "MAY input Plaintext Length Bound" → `EncryptStreamInput.data_size: Option<usize>`
- "SHOULD ensure can't specify both" → `EncryptInput` has no `plaintext_length_bound` field
- "MUST NOT use if both specified" → impossible by construction in `EncryptInput`

## Potential Spec Gaps

### 1. EncryptStreamInput lacks spec annotations
- **Code location**: `EncryptStreamInput` struct in `types.rs` (lines ~315-330)
- **Behavior**: `EncryptStreamInput` mirrors `EncryptInput` but for streaming. It has the same fields (algorithm_suite_id, encryption_context, frame_length, source, max_encrypted_data_keys, commitment_policy) plus `data_size`.
- **Why it matters**: The spec's `encrypt.md#input` requirements apply to both streaming and non-streaming encrypt operations, but `EncryptStreamInput` has NO duvet annotations at all. This means the streaming input struct is completely untraced to the spec.
- **Suggested spec requirement**: The streaming encrypt input SHOULD have the same required and optional arguments as the non-streaming encrypt input, with the addition of an optional data size parameter.

### 2. DecryptStreamInput lacks spec annotations
- **Code location**: `DecryptStreamInput` struct in `types.rs` (lines ~420-435)
- **Behavior**: `DecryptStreamInput` mirrors `DecryptInput` but for streaming. It has the same fields minus `ciphertext` (which comes from the stream), plus `i_accept_the_danger`.
- **Why it matters**: Same as above — the streaming decrypt input is completely untraced to the spec.
- **Suggested spec requirement**: The streaming decrypt input SHOULD have the same required and optional arguments as the non-streaming decrypt input.

### 3. NetV400RetryPolicy not in spec
- **Code location**: `NetV400RetryPolicy` enum and `DecryptInput.net_v4_retry_policy` field
- **Behavior**: Allows retrying decryption with a workaround for ESDK .NET v4.0.0 header serialization bug.
- **Why it matters (interop)**: This is an interoperability-relevant behavior — it handles a known bug in another ESDK implementation. The spec doesn't describe this retry mechanism.
- **Suggested spec requirement**: The decrypt operation MAY accept a configuration option to retry header authentication with an alternate byte ordering to handle messages produced by ESDK .NET v4.0.0.

### 4. EncryptOutput/DecryptOutput lack spec annotations
- **Code location**: `EncryptOutput` and `DecryptOutput` structs in `types.rs`
- **Behavior**: These define the output of encrypt/decrypt operations.
- **Why it matters**: The spec's `encrypt.md#output` and `decrypt.md#output` sections describe required output fields, but the output structs have no duvet annotations.
- **Note**: These are outside the scoped specs for this work item but should be addressed in a future work item.
