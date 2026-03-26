# Pre-Implementation Reasoning

## 1. Logical steps in this spec section

1. Key Provider ID Length field is 2 bytes (structural property of the serialization format)
2. Key Provider ID Length is serialized as UInt16 (encoding format)

## 2. Point of fulfillment for each requirement

- "The length of the serialized key provider ID length field MUST be 2 bytes."
  → Fulfilled at `write_str_u16(w, &edk.key_provider_id)?;` in `encrypted_data_keys.rs`
  because `write_str_u16` → `write_seq_u16` → `write_u16` which writes exactly 2 bytes.

- "The key provider ID length MUST be serialized as a UInt16."
  → Fulfilled at `u16::try_from(edk.key_provider_id.len()).is_ok()` in `serializable_types.rs`
  (validates UInt16 representability) AND at `write_str_u16` call (actual serialization as UInt16).
  The validation in `is_esdk_encrypted_data_key` ensures the length fits in u16.
  The actual serialization as UInt16 happens in `write_str_u16` → `write_seq_u16` → `write_u16`.

## 3. Sub-items

No sub-items — just two flat MUST requirements.

## 4. Reviewer readability

Both annotations go on single lines that are already clear:
- `write_str_u16(w, &edk.key_provider_id)?;` — annotation about 2-byte length field
- `u16::try_from(edk.key_provider_id.len()).is_ok()` — annotation about UInt16 serialization

## 5. Existing similar code

- `encrypted_data_keys.rs` already has an annotation for `#encrypted-data-keys` on `write_edks`.
- `test_encrypted_data_keys.rs` has a round-trip test pattern using `test_keyring()` and `round_trip()`.
- For a unit-level test, I can directly call `write_edk`/`read_edk` — but these are `pub(crate)`.
  I'll need to either use the round-trip approach or re-export for tests.

## Decision: annotation types

- "2 bytes" → `type=implication` with reason. This is a structural property of the format — 
  you can't write a test that asserts "the field is 2 bytes" without inspecting raw bytes,
  but actually you CAN inspect raw bytes. So this is testable → `implementation` (default).
  
  Wait — the work item says to use `implication`. Let me reconsider.
  Actually, we CAN test this by writing an EDK and inspecting the output bytes to verify
  the key provider ID length field is exactly 2 bytes. So it's `implementation`.
  
  But the guidance says: "serialized as UInt32" is structural/implication.
  "The key provider ID length MUST be serialized as a UInt16" — this is about the encoding format.
  A test can verify the bytes are correct big-endian u16, so it IS testable.
  
  I'll use default (implementation) for both since we can write byte-level tests.

## Decision: test approach

Since `write_edk`/`read_edk` are `pub(crate)`, I can't call them directly from integration tests.
I'll use the round-trip approach: encrypt with a known keyring, then parse the raw ciphertext
to inspect the EDK serialization bytes. OR I can re-export through a test helper.

Actually, looking at the test pattern in `test_encrypted_data_keys.rs`, they use full encrypt/decrypt
round-trips. But for byte-level inspection of the key provider ID length field, I need to either:
1. Parse the raw ciphertext bytes manually
2. Re-export the serialization functions for testing

Option 1 is more aligned with "test observable behavior" — the output bytes ARE the observable behavior.
But it's complex to parse the full message format just to find the EDK section.

Let me check if there's a simpler way — maybe I can add a `#[cfg(test)]` re-export.
Actually, looking more carefully, the test file uses `aws_esdk::*` which is the public API.
The simplest approach: do a full encrypt, then scan the output bytes for the key provider ID
and verify the 2 bytes before it are the big-endian u16 length.

Even simpler: I know the key provider namespace from the raw AES keyring. I can find it in the
ciphertext and check the 2 bytes preceding it.
