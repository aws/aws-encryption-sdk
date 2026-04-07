# Agent 2 Notes — decrypt-body (Non-Framed Conformance Annotation)

## Pre-Implementation Reasoning

### 1. What are the logical steps?
This is a FIX_ANNOTATION task — no new logic needed. Steps:
1. Add implementation annotation for "Non-framed data deserialization MUST conform to..." at the call site in decrypt.rs
2. Add cross-reference annotation for the linked `message-body.md#non-framed-data` spec
3. Add `type=test` annotation to an existing test that exercises non-framed decryption

### 2. Point of fulfillment
- "Non-framed data deserialization MUST conform to the [Non-Framed Data]..." → fulfilled at the `ContentType::NonFramed` match arm in `step_decrypt_body` where `read_and_decrypt_non_framed_message_body` is called
- Cross-reference to `message-body.md#non-framed-data` → same location (the function being called implements the non-framed data format)

### 3. Sub-items?
No sub-items — single conformance requirement with one cross-reference link.

### 4. Reviewer readability
The annotation goes on the `ContentType::NonFramed =>` arm, immediately before the function call. Single annotation + cross-reference, no stacking issue.

### 5. Existing similar code
The `ContentType::Framed` arm already has annotations. The regular/final frame conformance annotations are in the test file. Follow the same pattern.

### Cross-reference check
The quote contains `[Non-Framed Data](../data-format/message-body.md#non-framed-data)` — must add cross-reference from `message-body.md#non-framed-data`. The TOML has one `[[spec]]` entry:
```
Non-framed data MUST consist of, in order, IV, Encrypted Content Length, Encrypted Content, and Authentication Tag.
```
This is the right cross-reference — it describes the structure that the deserialization conforms to. Use `type=implication` since "consist of, in order" is structural.

### Test annotation
The test file already has `test_decrypt_regular_frame_deserialization` and `test_decrypt_final_frame_deserialization` for the other two conformance requirements. Need to find or create a test for non-framed. Looking at existing tests... `test_decrypt_unframed_sequence_number_is_one` exercises non-framed path indirectly but is about sequence numbers. `test_decrypt_content_type_determines_framed_or_nonframed` only tests framed. Since ESDK only encrypts framed data, we can't easily create a non-framed round-trip test. But we can add the annotation to an existing test that exercises the non-framed code path, or create a new test that constructs a non-framed message manually. Actually, looking more carefully — the existing tests all use `round_trip` which encrypts framed. None of them exercise the non-framed path. We need a test that actually calls the non-framed deserialization. This may require constructing a non-framed message manually.

Wait — let me re-read the test for unframed sequence number. It says "We cannot encrypt non-framed data with this ESDK" and uses framed path. So there's no existing test that exercises non-framed deserialization.

For a proper test, I'll need to construct a non-framed encrypted message manually (encrypt with AES-GCM directly, build the message format by hand) and then decrypt it.
