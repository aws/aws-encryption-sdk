# Agent 2 Notes — Footer Round 3

## What Changed
Moved the encrypt.md "Signature Length" annotation from before `write_u16(w, len)?;` up to the `let len` line per Round 2 reviewer feedback.

## Why This Is Correct
The annotation `- [Signature Length](...): MUST be the length of the output of the calculation above.` is about the *value* of the length field being derived from `signature.len()`. The `let len = u16::try_from(signature.len())` line is where that derivation happens — a better semantic fit than the `write_u16` call which is about serialization format.

## Annotation Count Verification
- Before `let len`: 2 blocks ✅ (under 3)
- Before `write_u16`: 2 blocks ✅ (under 3)
- Before `write_bytes`: 1 block ✅ (under 3)
