## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/footer.rs` — Moved the encrypt.md "Signature Length" annotation from before `write_u16(w, len)?;` up to the `let len` line, reducing the annotation stack before `write_u16` from 3 blocks to 2 blocks.

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/footer.rs
```

### Requirements Addressed
- ✅ Round 2 feedback: 3-stack before `write_u16(w, len)?;` — fixed by moving encrypt.md "Signature Length" annotation to `let len` line

### Annotation Distribution After Fix
- `let len` line: 2 annotation blocks ("serialize a message footer" + "Signature Length MUST be the length")
- `write_u16(w, len)?;` line: 2 annotation blocks ("MUST be 2 bytes" + "MUST be interpreted as UInt16")
- `write_bytes(w, signature)?;` line: 1 annotation block ("Signature MUST be the output")

All under the 3-block hard limit.

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: N/A — this is an annotation-only fix (all annotations are `type=implication`, which satisfy both implementation and test checks in duvet)
- **Number of `type=test` annotations added**: 0 (no new requirements added; this round only moved an existing annotation)
- **Test function names**: N/A

### Proposed Commit Message

```
fix(footer): move Signature Length annotation to let-binding to fix 3-stack

Move the encrypt.md "Signature Length MUST be the length" annotation
from before write_u16(w, len) up to the let len = u16::try_from(...)
line. This reduces the annotation stack before write_u16 from 3 blocks
to 2 blocks, satisfying the hard stacking limit.

The new placement is also a better semantic fit: the annotation is about
the *value* of the length field equaling signature.len(), and the let
binding is where that value is computed.

Refs: specification/data-format/message-footer.md#signature-length
Refs: specification/client-apis/encrypt.md#construct-the-signature
```

### Duvet Verification (actual command output)
```
$ make duvet
[Pre-existing 2 errors in compliance_exceptions/encrypt.txt for construct-a-frame section — unrelated to footer changes. No footer-specific errors.]
```

### Test Results (actual command output)
```
$ cargo test
test result: FAILED. 0 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out
[All 8 failures are pre-existing test_authentication_tag tests failing due to AWS credential issues — unrelated to this change.]
```

### Notes
- The only change is moving one annotation block (encrypt.md "Signature Length") from before `write_u16` to before `let len`. No functional code changes.
- The `make duvet` errors are pre-existing in `compliance_exceptions/encrypt.txt` for the `construct-a-frame` section, not related to footer annotations.
- The 8 test failures are all in `test_authentication_tag.rs` due to invalid AWS security tokens — pre-existing and unrelated.
