# Discovery Notes — types.rs

## Spec-Aligned Structure Analysis

### 1. What is the spec section's logical flow?

`client.md#initialization` describes the ESDK client initialization:
1. Caller MAY provide a commitment policy → maps to `commitment_policy` field on input structs
2. Caller MAY provide a maximum number of encrypted data keys → maps to `max_encrypted_data_keys` field
3. If no commitment policy provided, default to REQUIRE_ENCRYPT_REQUIRE_DECRYPT → maps to `Default` impl
4. If no max EDKs provided, default to no limit → maps to `Default` impl (`Option<NonZeroUsize>` = `None`)
5. Commitment policy SHOULD be immutable once set → structural (non-exhaustive enum, no setter)

### 2. Where will each requirement be fulfilled in code?

- "provide a commitment policy" → `pub commitment_policy: EsdkCommitmentPolicy` field on `EncryptInput` / `DecryptInput`
- "provide a maximum number of encrypted data keys" → `pub max_encrypted_data_keys: Option<NonZeroUsize>` field
- "default MUST be REQUIRE_ENCRYPT_REQUIRE_DECRYPT" → `#[derive(Default)]` on `EncryptInput` which uses `EsdkCommitmentPolicy::default()` = `RequireEncryptRequireDecrypt`
- "default MUST result in no limit" → `Option<NonZeroUsize>` defaults to `None` via `#[derive(Default)]`

### 3. Sub-items?

No sub-items — each requirement is a standalone statement.

### 4. Most likely structural mistake?

The implementer might be tempted to annotate at the struct definition level (above `pub struct EncryptInput`).
But the correct placement is:
- For "provide option" requirements → at the specific field declaration
- For "default" requirements → at the `Default` impl or derive

The `Default` for `EncryptInput` is derived, not manually implemented.
The annotation for the default commitment policy should go on the `#[derive(Default)]` line
or on the `commitment_policy` field with a reason explaining the derive chain.

The annotation for "default no limit" should go on the `max_encrypted_data_keys` field
since `Option<NonZeroUsize>` defaults to `None` via derive.

Note: `decrypt.rs` already has `implication` annotations for the commitment policy default
and immutability. The gap is that `types.rs` has NO annotations for the "provide option" requirements
and the "no limit default" requirement.

## Potential Spec Gaps

### 1. `EncryptInput` has `commitment_policy` as a direct field, not via a "client" object

The spec says "On client initialization" but the Rust ESDK doesn't have a separate client object
with initialization. Instead, commitment policy and max EDKs are fields on each input struct.
This is a design choice that differs from the spec's model but achieves the same result.

- **Code location**: `EncryptInput.commitment_policy`, `DecryptInput.commitment_policy`
- **Why it matters**: Interop — other SDKs may have a client object
- **Suggested spec wording**: "Implementations MAY provide commitment policy and max encrypted data keys
  as per-operation input parameters rather than client-level configuration."

### 2. `NetV400RetryPolicy` has no spec requirement

`DecryptInput` has a `net_v4_retry_policy` field that controls retry behavior for
incorrectly serialized .NET v4.0.0 messages. This is not mentioned in the spec.

- **Code location**: `DecryptInput.net_v4_retry_policy`, `DecryptStreamInput.net_v4_retry_policy`
- **Why it matters**: Interop — this is a compatibility feature for a known bug
- **Suggested spec wording**: "Implementations SHOULD provide an option to retry decryption
  of messages produced by ESDK .NET v4.0.0 which incorrectly serialized the message header."
