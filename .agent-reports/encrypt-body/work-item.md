# Work Item: Add Missing Annotations for Final Frame Serialization Fields in construct-a-frame

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Sections**: `## Construct the body`, `### Construct a frame`, `### Un-Framed Message Body Encryption`
- **Duvet Target**: `specification/client-apis/encrypt.md#construct-a-frame`

## Type of Work
ADD_ANNOTATIONS (implementation + test annotations for 6 missing requirements)

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  For a regular frame, each field MUST be serialized according to its specification:
  ```
- **Current State**: missing (no implementation or test annotation)
- **Placement**: `src/message/body.rs` — in `construct_frame()`, before the regular frame field serialization block (near line 702 where `write_u32(frame_buf, input.sequence_number)` begins). Use `type=implication` with `reason=` explaining the subsequent lines serialize each field per spec.

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  For a final frame, each field MUST be serialized according to its specification:
  ```
- **Current State**: missing (no implementation or test annotation)
- **Placement**: `src/message/body.rs` — in `construct_frame()`, at the `if input.is_final` block (near line 684) where final frame serialization begins. Use `type=implication` with `reason=` explaining the subsequent lines serialize each final frame field per spec.

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be serialized according to the
  [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
  ```
- **Current State**: missing (no implementation or test annotation)
- **Placement**: `src/message/body.rs` — at `write_u32(frame_buf, input.sequence_number)` (line ~706). This line serializes the sequence number for BOTH regular and final frames. Add annotation here with `reason=` explaining this shared code path handles both frame types.

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [IV](../data-format/message-body.md#final-frame-iv): MUST be serialized according to the
  [Final Frame IV](../data-format/message-body.md#final-frame-iv) specification.
  ```
- **Current State**: missing (no implementation or test annotation)
- **Placement**: `src/message/body.rs` — at `write_bytes(frame_buf, iv)` (line ~720). This line serializes the IV for BOTH regular and final frames. Add annotation here with `reason=` explaining this shared code path handles both frame types.

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content): MUST be serialized according to the
  [Final Frame Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content) specification.
  ```
- **Current State**: missing (no implementation or test annotation)
- **Placement**: `src/message/body.rs` — after the `aes_encrypt` call (line ~761) where encrypted content is appended to `frame_buf`. The existing annotation covers the regular frame encrypted content; add a parallel annotation for the final frame variant at the same location.

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag): MUST be serialized according to the
  [Final Frame Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag) specification.
  ```
- **Current State**: missing (no implementation or test annotation)
- **Placement**: `src/message/body.rs` — after the `aes_encrypt` call (line ~770) where the authentication tag is appended to `frame_buf`. The existing annotation covers the regular frame auth tag; add a parallel annotation for the final frame variant at the same location.

## Existing Code Context

### Source File: `src/message/body.rs`

The `construct_frame()` function (starting at line 625) handles both regular and final frames. Key locations for annotations:

```rust
    // Line ~702: Sequence number serialization (shared for regular + final)
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be serialized according to the
    //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
    //# The value MUST be the sequence number of this frame.
    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //# The sequence number MUST be interpreted as a UInt32.
    write_u32(frame_buf, input.sequence_number)?;
```

```rust
    // Line ~714-720: IV serialization (shared for regular + final)
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be serialized according to the
    //# [Regular Frame IV](../data-format/message-body.md#regular-frame-iv) specification.

    //= specification/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the IV used when calculating the encrypted content for this frame.
    write_bytes(frame_buf, iv)?;
```

```rust
    // Line ~761-771: Encrypted content + auth tag (shared for regular + final)
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content): MUST be serialized according to the
    //# [Regular Frame Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the encrypted content calculated for this frame.
    let _encrypted_content_written = ();
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag): MUST be serialized according to the
    //# [Regular Frame Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag) specification.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the authentication tag output when calculating the encrypted content for this frame.
    let _authentication_tag_written = ();
```

### Test File: `tests/test_construct_a_frame.rs`

Existing tests cover regular frame serialization and round-trip verification. The final frame-specific serialization requirements need test annotations added to existing tests that already exercise final frame paths.

```rust
// test_construct_frame_single_final_frame already tests final frame serialization
#[tokio::test(flavor = "multi_thread")]
async fn test_construct_frame_single_final_frame() {
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= type=test
    //# For a final frame, the serialization MUST follow the [Final Frame](../data-format/message-body.md#final-frame) specification.
    let pt = b"short";
    let result = round_trip(pt, 4096).await;
    assert_eq!(result, pt.to_vec());
}
```

## Implementation Guidance

### Critical: Shared Code Path
The `construct_frame()` function uses a SINGLE code path for both regular and final frames. The sequence number, IV, encrypted content, and auth tag serialization lines are NOT inside `if input.is_final` blocks — they are shared. The final frame annotations MUST be placed at these shared locations alongside the existing regular frame annotations.

### Pattern to Follow
The existing regular frame annotations demonstrate the pattern. For each shared serialization line, add a parallel annotation block for the final frame variant. Example:

```rust
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be serialized according to the
    //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
    //# The value MUST be the sequence number of this frame.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //= reason=write_u32 serializes the sequence number for both regular and final frames in this shared code path
    //# - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be serialized according to the
    //# [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
    write_u32(frame_buf, input.sequence_number)?;
```

### For Requirements 1 and 2 (preamble requirements)
These are structural/preamble requirements. Use `type=implication` with `reason=` explaining that the subsequent code serializes each field per the spec.

- Requirement 1 ("For a regular frame, each field MUST be serialized..."): Place before the sequence number write, with `reason=The following lines serialize SeqNum, IV, EncContent, and AuthTag in order per the Regular Frame spec`.
- Requirement 2 ("For a final frame, each field MUST be serialized..."): Place at the `if input.is_final` block entry, with `reason=The following lines serialize SeqNumEnd, SeqNum, IV, EncContentLen, EncContent, and AuthTag in order per the Final Frame spec`.

### For Requirements 3-6 (field-level requirements)
These are implementation annotations at the shared serialization code. Each needs a `reason=` line explaining the shared code path.

### Test Annotations
Add `type=test` annotations to existing tests that exercise final frame paths:
- `test_construct_frame_single_final_frame` — tests final frame serialization (covers Req 2, 3, 4, 5, 6)
- `test_construct_frame_serialization_regular_and_final` — tests both frame types (covers Req 1, 2)
- `test_construct_frame_auth_tag_serialized` — tests encrypted content and auth tag (covers Req 5, 6)
- `test_construct_frame_final_frame_has_endframe_marker` — tests final frame structure (covers Req 2)

### Spec-Aligned Structure
The spec describes this flow for construct-a-frame serialization:

**Regular frame fields** (annotate at shared code):
1. Sequence Number → `write_u32(frame_buf, input.sequence_number)` ✅ already annotated
2. IV → `write_bytes(frame_buf, iv)` ✅ already annotated
3. Encrypted Content → after `aes_encrypt` ✅ already annotated
4. Authentication Tag → after `aes_encrypt` ✅ already annotated

**Final frame fields** (annotate at shared code + `if input.is_final` blocks):
1. Sequence Number End → `write_u32(frame_buf, ENDFRAME_SEQUENCE_NUMBER)` ✅ already annotated
2. Sequence Number → `write_u32(frame_buf, input.sequence_number)` ❌ MISSING
3. IV → `write_bytes(frame_buf, iv)` ❌ MISSING
4. Encrypted Content Length → `write_u32(frame_buf, input.plaintext.len() as u32)` ✅ already annotated
5. Encrypted Content → after `aes_encrypt` ❌ MISSING
6. Authentication Tag → after `aes_encrypt` ❌ MISSING

**Preamble requirements:**
- "For a regular frame, each field MUST be serialized..." ❌ MISSING
- "For a final frame, each field MUST be serialized..." ❌ MISSING

## Targeted Tests
- `test_construct_frame_single_final_frame` — exercises final frame serialization path
- `test_construct_frame_serialization_regular_and_final` — exercises both regular and final frame serialization
- `test_construct_frame_auth_tag_serialized` — exercises encrypted content and auth tag serialization
- `test_construct_frame_final_frame_has_endframe_marker` — exercises final frame structure
- `test_construct_frame_final_frame_content_length_serialized` — exercises final frame content length
- `test_construct_frame_final_frame_content_length_less_than_frame_length` — exercises final frame with partial content

## Success Criteria
```bash
cargo test test_construct_a_frame
cargo test test_construct_the_body
make duvet
```
- [ ] All tests pass
- [ ] duvet report shows no gaps for `construct-the-body`, `construct-a-frame`, and `un-framed-message-body-encryption` sections
- [ ] All 6 missing requirements have `type=implementation` (or `type=implication` for preamble requirements)
- [ ] All 6 missing requirements have corresponding `type=test` annotations
- [ ] No new duvet errors (all annotation quotes match spec text exactly)
