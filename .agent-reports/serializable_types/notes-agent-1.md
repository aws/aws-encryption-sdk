# Agent 1 Notes — serializable_types.rs

## Potential Spec Gaps

### 1. Encryption context total serialized length constraint
- **Code location**: `serializable_types.rs`, `is_esdk_encryption_context()` — checks `length(ec) >= ESDK_CANONICAL_ENCRYPTION_CONTEXT_MAX_LENGTH` where the max is `u16::MAX - 2`.
- **Why it matters**: Correctness / interop. The spec describes individual key/value length constraints and the KVP length field as UInt16, but does not explicitly state a total serialized length bound for the encryption context. The code enforces `u16::MAX - 2` as the max total serialized length.
- **Suggested spec requirement**: "The total serialized length of all key-value pairs in the encryption context MUST NOT exceed 65533 bytes (UInt16 max minus 2 bytes for the key-value pair count field)."

### 2. Encryption context pair count constraint
- **Code location**: `serializable_types.rs`, `is_esdk_encryption_context()` — checks `ec.len() >= u16::MAX as usize`.
- **Why it matters**: Correctness / interop. The spec does not explicitly state a maximum number of encryption context pairs, but the code enforces that the count fits in a u16 (since the KVP count is serialized as UInt16 in the AAD).
- **Suggested spec requirement**: "The number of key-value pairs in the encryption context MUST be representable as a UInt16."

### 3. EDK ciphertext length not validated
- **Code location**: `serializable_types.rs`, `is_esdk_encrypted_data_key()` — only checks `key_provider_id.len()` and `key_provider_info.len()` fit in u16, but does NOT check `ciphertext.len()` fits in u16.
- **Why it matters**: Correctness. The spec says "The encrypted data key length MUST be serialized as a UInt16", which implies the ciphertext must fit in u16. The validation function is incomplete — it would allow a ciphertext longer than u16::MAX, which would fail at serialization time in `write_seq_u16`.
- **Suggested fix**: Add `u16::try_from(edk.ciphertext.len()).is_ok()` to `is_esdk_encrypted_data_key`.

## Spec Structure Analysis (Step 6.8)

### 1. Spec section logical flow for `#key-provider-id-length`
The `#key-provider-id-length` section is a sub-field of `#encrypted-data-key-entries`. The flow is:
1. Each EDK entry has a Key Provider ID Length field (2 bytes, UInt16)
2. Followed by the Key Provider ID (variable, length = previous field)
3. The length field constrains the Key Provider ID to fit in UInt16

### 2. Where each requirement is fulfilled in code
- "The length of the serialized key provider ID length field MUST be 2 bytes." → `write_str_u16` in `serialize_functions.rs` calls `write_u16` which writes 2 bytes. The validation in `is_esdk_encrypted_data_key` ensures the value fits.
- "The key provider ID length MUST be serialized as a UInt16." → `write_str_u16` → `write_u16` → `data.to_be_bytes()` (big-endian u16). The `is_esdk_encrypted_data_key` function validates `u16::try_from(edk.key_provider_id.len()).is_ok()`.

### 3. Sub-items
The section has 2 MUST requirements, no sub-lists.

### 4. Most likely structural mistake
The implementer may be tempted to annotate at the `write_str_u16` call in `encrypted_data_keys.rs` instead of at the validation in `serializable_types.rs`. Since the task is scoped to `serializable_types.rs`, the annotation should go on the `u16::try_from(edk.key_provider_id.len()).is_ok()` line as an `implication` — the validation ensures the constraint is met. The actual serialization annotation belongs in `encrypted_data_keys.rs` or `serialize_functions.rs` (separate work item).
