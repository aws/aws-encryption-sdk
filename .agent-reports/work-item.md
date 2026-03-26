# Work Item: Add Missing `construct-the-body` Annotations to `encrypt_and_serialize_body`

## Specification
- **File**: `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `construct-the-body`
- **Duvet Target**: `specification/client-apis/encrypt.md#construct-the-body`

## Type of Work
ADD_ANNOTATIONS (missing implementation annotations on existing code)

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Regular frame serialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
  ```
- **Current State**: missing — code exists in `encrypt_and_serialize_body` loop calling `construct_frame` with `is_final: false`, but no annotation

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  Before the end of the input is indicated,
  this operation MUST process as much of the consumable bytes as possible
  by [constructing regular frames](#construct-a-frame).
  ```
- **Current State**: missing — the `loop` in `encrypt_and_serialize_body` reads full frames and constructs them, but no annotation

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  When the end of the input is indicated,
  this operation MUST perform the following until all consumable plaintext bytes are processed:
  ```
- **Current State**: missing — the code after the loop break handles end-of-input, but no annotation

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - If there are exactly enough consumable plaintext bytes to create one regular frame,
  such that creating a regular frame processes all consumable bytes,
  then this operation MUST [construct either a final frame or regular frame](#construct-a-frame)
  with the remaining plaintext.
  ```
- **Current State**: missing — the `next_char.is_none()` break handles this case (exact match → break to final frame), but no annotation

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - If there are enough input plaintext bytes consumable to create a new regular frame,
  such that creating a regular frame does not processes all consumable bytes,
  then this operation MUST [construct a regular frame](#construct-a-frame)
  using the consumable plaintext bytes.
  ```
- **Current State**: missing — the `next_char.is_some()` path continues the loop to construct a regular frame, but no annotation

### Requirement 6
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - If there are not enough input consumable plaintext bytes to create a new regular frame,
  then this operation MUST [construct a final frame](#construct-a-frame)
  ```
- **Current State**: missing — the `in_size != frame_length` break handles this case, but no annotation

### Requirement 7
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If an end to the input has been indicated, there are no more consumable plaintext bytes to process,
  and a final frame has not yet been constructed,
  this operation MUST [construct an empty final frame](#construct-a-frame).
  ```
- **Current State**: missing — the final `construct_frame` call with `plaintext_frame[0..in_size]` where `in_size` can be 0 handles this, but no annotation

### Requirement 8
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If [Plaintext Length Bound](#plaintext-length-bound) was specified on input
  and this operation determines at any time that the plaintext being encrypted
  has a length greater than this value,
  this operation MUST immediately fail.
  ```
- **Current State**: missing — Plaintext Length Bound is NOT implemented in the codebase. This should be annotated as `type=todo` or `type=exception` with a reason.

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/body.rs`
```rust
/// Encrypt plaintext and serialize the message body (framed) to the output stream.
pub(crate) fn encrypt_and_serialize_body(
    plaintext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    let mut total_data_size: usize = 0;
    let frame_length = header.body.frame_length() as usize;
    // ... setup ...
    let mut sequence_number = START_SEQUENCE_NUMBER;
    // ... more setup ...

    loop {
        in_size = read_up_to_peek(plaintext, &mut plaintext_frame, next_char)?;
        if in_size != frame_length {
            break;  // ← Req 6: not enough bytes for regular frame
        }
        next_char = read_opt_u8(plaintext)?;
        if next_char.is_none() {
            break;  // ← Req 4: exactly enough bytes, no more input
        }
        // ← Req 5: more bytes available, construct regular frame
        // ... frame count check ...
        construct_frame(/* ... is_final: false ... */);  // ← Req 1, Req 2
        sequence_number += 1;
    }

    // ← Req 3: end of input processing
    // Final frame
    construct_frame(/* ... is_final: true ... */);  // ← Req 7 (when in_size == 0)
    Ok(())
}
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_construct_a_frame.rs`
Tests exist for `construct-a-frame` but NOT for `construct-the-body`. A new test file or additional tests are needed.

## Implementation Guidance

### Adding Implementation Annotations (Requirements 1-7)

All 7 requirements map to existing code in `encrypt_and_serialize_body`. Add annotations at the exact point of fulfillment:

- **Req 1** ("Regular frame serialization MUST conform..."): Annotate before the `construct_frame` call inside the loop (the regular frame path).
- **Req 2** ("Before the end of the input is indicated, this operation MUST process as much..."): Annotate at the `loop` statement itself — the loop processes all consumable bytes as regular frames.
- **Req 3** ("When the end of the input is indicated..."): Annotate after the loop, before the final frame construction.
- **Req 4** ("If there are exactly enough consumable plaintext bytes..."): Annotate at the `if next_char.is_none() { break; }` — this is where exact-match input falls through to final frame.
- **Req 5** ("If there are enough input plaintext bytes consumable..."): Annotate inside the loop after the `next_char.is_some()` check (i.e., the continuation path that constructs a regular frame).
- **Req 6** ("If there are not enough input consumable plaintext bytes..."): Annotate at the `if in_size != frame_length { break; }` — this is where short input falls through to final frame.
- **Req 7** ("If an end to the input has been indicated... MUST construct an empty final frame"): Annotate at the final `construct_frame` call — the `plaintext_frame[0..in_size]` slice handles the empty case when `in_size == 0`.

### Handling Requirement 8 (Plaintext Length Bound)

Plaintext Length Bound is not implemented. Add a `type=todo` annotation near the top of `encrypt_and_serialize_body`:
```rust
//= specification/client-apis/encrypt.md#construct-the-body
//= type=todo
//# If [Plaintext Length Bound](#plaintext-length-bound) was specified on input
//# and this operation determines at any time that the plaintext being encrypted
//# has a length greater than this value,
//# this operation MUST immediately fail.
```

### Adding Test Annotations

The existing `test_construct_a_frame.rs` tests exercise the body construction path through round-trip encrypt/decrypt. Add `type=test` annotations for `construct-the-body` requirements to the existing tests or create new targeted tests:

- `test_construct_frame_regular_frame_plaintext_equals_frame_length` → can carry Req 1, 2, 5 test annotations
- `test_construct_frame_final_frame_remaining_plaintext` → can carry Req 3, 6 test annotations
- `test_construct_frame_empty_plaintext` → can carry Req 7 test annotation
- `test_construct_frame_single_final_frame` → can carry Req 4 test annotation (exact match case)

### Spec-Aligned Structure
The spec describes this flow:
1. Process regular frames in a loop → annotate at the `loop` and `construct_frame(is_final: false)` call
2. Detect end of input → annotate at the `break` conditions
3. Handle exact-match case → annotate at `next_char.is_none()` break
4. Handle more-data case → annotate at the loop continuation (after `next_char` check)
5. Handle short-data case → annotate at `in_size != frame_length` break
6. Construct final frame → annotate at `construct_frame(is_final: true)` call

### Patterns to Follow
- Follow the annotation style in `read_and_decrypt_framed_message_body` in the same file (body.rs lines 81-270) — it has dense annotations for the decrypt path
- Follow the test annotation style in `test_construct_a_frame.rs` — each test has `//= type=test` annotations with exact TOML quotes

### Common Mistake to Avoid
Do NOT annotate Req 4/5/6 at the final `construct_frame` call. These requirements describe the *decision logic* (which branch to take), not the frame construction itself. Annotate at the branch/break points where the decision is made.

## Targeted Tests
- `test_construct_frame_regular_frame_plaintext_equals_frame_length` — proves regular frames use frame_length bytes
- `test_construct_frame_final_frame_remaining_plaintext` — proves final frame gets remaining bytes
- `test_construct_frame_empty_plaintext` — proves empty final frame is constructed
- `test_construct_frame_single_final_frame` — proves single final frame for short input
- `test_construct_frame_serialization_regular_and_final` — proves both frame types serialize correctly
- `test_construct_frame_sequence_number_increments` — proves multi-frame processing

## Success Criteria
```bash
cargo test --test test_construct_a_frame
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `encrypt.md#construct-the-body`
- [ ] All requirements have `type=implementation` (not `type=todo`) except Plaintext Length Bound which should be `type=todo`
- [ ] All implementations have corresponding `type=test`
