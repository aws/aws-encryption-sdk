# Pre-Implementation Reasoning

## 1. Logical steps in this spec section

1. Serialize the Encrypted Data Key Count (a UInt16)
2. Serialize the Encrypted Data Key Entries (one per EDK)

The `write_edks` function already does exactly this in order.

## 2. Point of fulfillment for each requirement

- "The Encrypted Data Keys MUST be serialized as, in order, Encrypted Data Key Count, and Encrypted Data Key Entries."
  → Fulfilled at the function body of `write_edks`, which writes count first (`write_u16`) then entries (`for edk in edks { write_edk }`)

## 3. Sub-items?

The requirement lists two sub-items: "Encrypted Data Key Count" and "Encrypted Data Key Entries". These map to the two statements in `write_edks`. A single annotation at the top of the function body covers the ordering requirement since the function is only 4 lines.

## 4. Can a reviewer read this top-to-bottom?

Yes. `write_edks` is 4 lines. The annotation at the top, followed by `write_u16` (count) then `for` loop (entries), is immediately clear.

## 5. Existing similar code

- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/v2_header_body.rs` — uses similar serialization order annotations
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs` — round-trip test pattern

## Additional notes

- The misplaced `#encrypted-data-key-count` / "This value MUST be greater than 0." annotation in `read_edks` needs to be removed. The `read_u16` call doesn't enforce > 0; the actual enforcement is in `header.rs::validate_max_encrypted_data_keys`.
- The annotation quote has no markdown links, so no cross-references needed.
