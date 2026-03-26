# Agent 1 Notes — encrypted_data_keys.rs

## Spec Section Logical Flow

The `data-format/message-header.md#encrypted-data-keys` section describes a hierarchical serialization format:

1. **Encrypted Data Keys** (top-level container)
   - Serialized as: Encrypted Data Key Count + Encrypted Data Key Entries
2. **Encrypted Data Key Count** — 2-byte UInt16, must be > 0
3. **Encrypted Data Key Entries** — sequence of entries, each containing:
   - Key Provider ID Length (2-byte UInt16)
   - Key Provider ID (variable, UTF-8)
   - Key Provider Information Length (2-byte UInt16)
   - Key Provider Information (variable, bytes)
   - Encrypted Data Key Length (2-byte UInt16)
   - Encrypted Data Key (variable, bytes)

## Code-to-Spec Mapping

| Spec Requirement | Code Construct |
|---|---|
| EDKs serialized as count + entries | `write_edks` writes count then loops; `read_edks` reads count then loops |
| Count is 2-byte UInt16 | `write_u16` / `read_u16` in serialize_functions.rs |
| Count > 0 | `read_edks` does NOT enforce this; `header.rs::validate_max_encrypted_data_keys` does |
| Entry serialized in order | `write_edk` calls `write_str_u16`, `write_seq_u16`, `write_seq_u16` in order |
| Key Provider ID Length = 2-byte UInt16 | `write_seq_u16` writes u16 length prefix |
| Key Provider ID = UTF-8 | `write_str_u16` writes as bytes; `read_str_u16` decodes UTF-8 |
| Key Provider Info Length = 2-byte UInt16 | `write_seq_u16` writes u16 length prefix |
| Key Provider Info = bytes | `write_seq_u16` / `read_seq_u16` |
| EDK Length = 2-byte UInt16 | `write_seq_u16` writes u16 length prefix |
| EDK = bytes | `write_seq_u16` / `read_seq_u16` |

## Duplicate Annotation Issue

`This value MUST be greater than 0.` is annotated in TWO places:
1. `encrypted_data_keys.rs:37-38` — on `read_u16` call (does NOT enforce the constraint)
2. `header.rs:98-100` — on `validate_max_encrypted_data_keys` (DOES enforce the constraint)

The annotation in `encrypted_data_keys.rs` is misplaced. The `read_u16` call just reads the value; it doesn't validate that it's > 0. The actual enforcement happens in `header.rs`. This should be addressed but is lower priority than the missing test.

## Traceability Answers

1. **Spec logical flow**: Count → Entries → (per entry: provider ID length, provider ID, provider info length, provider info, EDK length, EDK)
2. **Code constructs**: `write_edks`/`read_edks` for container; `write_edk`/`read_edk` for entries; `write_u16`/`read_u16` for count; `write_str_u16`/`read_str_u16` for provider ID; `write_seq_u16`/`read_seq_u16` for provider info and EDK
3. **Sub-items**: Yes — the entry serialization order has 6 sub-items that map to individual function calls
4. **Most likely structural mistake**: Annotating `write_seq_u16` for both "length field MUST be 2 bytes" and "MUST be serialized as UInt16" — these are fulfilled by the same call. Use `implication` + `reason=` for the structural properties since they're enforced by the type system / function signature, not by runtime checks.

## Potential Spec Gaps

### 1. No validation of count > 0 during deserialization
- **Code location**: `encrypted_data_keys.rs::read_edks` — reads count but doesn't validate > 0
- **Why it matters**: Correctness — a message with 0 EDKs is invalid per spec but would be silently accepted during parsing (caught later by `validate_max_encrypted_data_keys` only if `max_edks` is set)
- **Suggested spec requirement**: "During deserialization, if the Encrypted Data Key Count is 0, the operation MUST fail."

### 2. No upper bound on EDK count during serialization
- **Code location**: `encrypted_data_keys.rs::write_edks` — casts `edks.len()` to `u16` without checking for empty
- **Why it matters**: Correctness — `write_edks` would write count=0 if given an empty slice, violating the spec
- **Suggested spec requirement**: "During serialization, the implementation MUST verify the Encrypted Data Key Count is greater than 0."
