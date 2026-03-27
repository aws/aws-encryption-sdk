# Discovery Notes — body-format (Cycle 4)

## Coverage Analysis Summary

Analyzed all 18 TOML files under `compliance/aws-encryption-sdk-specification/data-format/message-body/`.
Total normative requirements: 48 across all sections.

### Coverage Status by Section

| Section | Requirements | Covered | Gap |
|---------|-------------|---------|-----|
| `#non-framed-data` | 2 | 2 (1 exception, 1 implication) | 0 |
| `#non-framed-data-iv` | 4 | 4 (2 exception, 2 implication) | 0 |
| `#non-framed-data-encrypted-content-length` | 4 | 4 (2 exception, 2 implication) | 0 |
| `#non-framed-data-encrypted-content` | 2 | 2 (implication) | 0 |
| `#non-framed-data-authentication-tag` | 2 | 2 (implication) | 0 |
| `#framed-data` | 2 | 1 impl+test, **1 test-only** | **1** |
| `#regular-frame` | 1 | 1 (implication + test) | 0 |
| `#regular-frame-sequence-number` | 5 | 5 (impl/implication + test) | 0 |
| `#regular-frame-iv` | 3 | 3 (implication + test) | 0 |
| `#regular-frame-encrypted-content` | 2 | 2 (implication + test) | 0 |
| `#regular-frame-authentication-tag` | 2 | 2 (implication + test) | 0 |
| `#final-frame` | 7 | 7 (impl/implication + test) | 0 |
| `#sequence-number-end` | 3 | 3 (impl/implication + test) | 0 |
| `#final-frame-sequence-number` | 3 | 3 (implication + test) | 0 |
| `#final-frame-iv` | 3 | 3 (implication + test) | 0 |
| `#final-frame-encrypted-content-length` | 3 | 3 (implication + test) | 0 |
| `#final-frame-encrypted-content` | 2 | 2 (impl/implication + test) | 0 |
| `#final-frame-authentication-tag` | 2 | 2 (implication + test) | 0 |

**Total: 48 requirements, 47 fully covered, 1 gap**

### The Gap

`#framed-data` requirement: "The total bytes allowed in a single frame MUST be less than or equal to `2^32 - 1`."

- Has `type=test` in `test_message_body_format.rs` (line 127)
- Has NO `type=implementation` or `type=implication` annotation anywhere
- Enforcement: `FrameLength` wraps `NonZeroU32` (max value `u32::MAX = 2^32 - 1`), defined in `src/types.rs:56`
- Best annotation location: `src/types.rs` at the `FrameLength::new` method, or `src/message/body.rs` at `encrypt_and_serialize_body` where `frame_length` is used

## Spec-Aligned Structure Analysis

1. The spec's logical flow for framed data: frame size constraint → frame count constraint → regular frame structure → final frame structure
2. The frame size constraint is enforced by the type system (`FrameLength` is `NonZeroU32`)
3. The frame count constraint is enforced at runtime in `encrypt_and_serialize_body`

## Potential Spec Gaps

None identified. The code behaviors align well with the spec requirements.
