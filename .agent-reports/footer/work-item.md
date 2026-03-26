# Work Item: Add Signature Length Annotations to Footer

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-footer.md`
- **Section**: `signature-length`
- **Duvet Target**: `aws-encryption-sdk-specification/data-format/message-footer.md#signature-length`

## Type of Work
FIX_ANNOTATION

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  quote = '''
  This length of the signature length field MUST be 2 bytes.
  '''
  ```
- **Current State**: missing
- **Sub-items**: none

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  quote = '''
  The signature length field MUST be interpreted as a UInt16.
  '''
  ```
- **Current State**: missing
- **Sub-items**: none

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/footer.rs`
```rust
/// Write a message footer (signature length + signature bytes).
pub(crate) fn write_footer(
    w: &mut dyn SafeWrite,
    signature: &[u8],
) -> Result<(), Error> {
    if signature.len() >= u16::MAX.into() {
        return ser_err("Length of signature bytes is larger than the uint16 limit.");
    }
    // ... existing encrypt.md annotations ...
    write_seq_u16(w, signature)
    // ...
}

/// Read a message footer, returning the signature bytes.
pub(crate) fn read_footer(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    read_seq_u16(r, raw)
}
```

### Relevant Helper: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/serialize_functions.rs`
```rust
pub(crate) fn write_seq_u16(w: &mut dyn SafeWrite, data: &[u8]) -> Result<(), Error> {
    match u16::try_from(data.len()) {
        Ok(len) => {
            write_u16(w, len)?;   // writes exactly 2 bytes (big-endian u16)
            write_bytes(w, data)
        }
        Err(_) => ser_err("Sequence length too long for 16 bits"),
    }
}

pub(crate) fn read_seq_u16(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    let len = read_u16(r, raw)?;  // reads exactly 2 bytes, interprets as big-endian u16
    read_vec(r, len as usize, raw)
}
```

### Test File: NEW FILE NEEDED: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_footer.rs`

## Implementation Guidance

These two requirements are structural properties of the serialization format.
The `write_seq_u16` function writes a `u16` (2 bytes, big-endian) length prefix,
and `read_seq_u16` reads 2 bytes and interprets them as a big-endian `u16`.
Both `write_footer` and `read_footer` delegate to these functions.

The requirements are fulfilled by the type system and function signatures —
they are not runtime-testable in isolation.
Use `type=implication` annotations.

1. Add `type=implication` annotations to `write_footer` at the `write_seq_u16` call,
   since that is where the 2-byte u16 length is written.
2. Add `type=implication` annotations to `read_footer` at the `read_seq_u16` call,
   since that is where the 2-byte u16 length is read and interpreted.

### Annotation Placement

For `write_footer`, place both annotations immediately before the `write_seq_u16(w, signature)` call
(after the existing `encrypt.md` annotations, before the function call).

For `read_footer`, place both annotations immediately before the `read_seq_u16(r, raw)` call.

### Pattern to Follow

Follow the existing `type=implication` pattern already used in `footer.rs`:
```rust
//= specification/data-format/message-footer.md#overview
//= type=implication
//# When an [algorithm suite](...) includes a [signature algorithm](...),
//# the [message](message.md) MUST contain a footer.
```

### Spec-Aligned Structure
The spec describes the footer structure as:
1. Signature Length field (2 bytes, UInt16) → annotate at `write_seq_u16` / `read_seq_u16` calls
2. Signature field (variable bytes) → already covered by `encrypt.md#construct-the-signature` annotations

Requirements to annotate individually:
- "This length of the signature length field MUST be 2 bytes." → at `write_seq_u16(w, signature)` in `write_footer` AND at `read_seq_u16(r, raw)` in `read_footer`
- "The signature length field MUST be interpreted as a UInt16." → at `write_seq_u16(w, signature)` in `write_footer` AND at `read_seq_u16(r, raw)` in `read_footer`

### Most Likely Structural Mistake
The implementer may be tempted to place annotations only in `write_footer` or only in `read_footer`.
Both functions fulfill these requirements — `write_footer` serializes the 2-byte u16 length,
and `read_footer` deserializes it. Place annotations in BOTH functions.

Alternatively, the implementer might try to annotate inside `serialize_functions.rs` at `write_seq_u16`/`read_seq_u16`.
Do NOT do this — those are generic helpers used by many serialization paths.
The annotation belongs in `footer.rs` where the footer-specific serialization happens,
because the spec requirement is about the footer's signature length field specifically.

## Targeted Tests
No unit tests are needed for `type=implication` annotations.
The `implication` type satisfies both implementation and test checks in duvet.

## Success Criteria
```bash
make duvet
```
- [ ] duvet report shows no gaps for `data-format/message-footer.md#signature-length`
- [ ] All requirements have `type=implication` (structural, not runtime-testable)
- [ ] Annotations use exact quotes from TOML
- [ ] Annotations are in `footer.rs`, not in `serialize_functions.rs`
