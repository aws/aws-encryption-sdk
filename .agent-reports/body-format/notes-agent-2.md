# Agent 2 Notes — body-format Cycle 2

## Pre-Implementation Reasoning (Cycle 2 — Review Feedback)

### Blocking Issues Analysis

1. **6-annotation stack at function entry**: The 5 exception annotations were stacked contiguously with the pre-existing `frame-length` annotation. Fix: blank lines between each exception block, and blank line before `frame-length`. Exception annotations don't need to be before specific code — they document intentional non-implementation.

2. **3-annotation stack before `let iv`**: A2 (deserialization order) is a Pattern 3 general-behavior annotation about the function's overall structure. It doesn't belong stacked with A5/A6 which are specific to the IV read. Fix: move A2 to standalone position after frame-length check.

3. **4-annotation stack before `let enc_content`**: A7+A10 describe the length field reading. A11+A12 describe properties of the content that was read. Fix: A7+A10 before the call, A11+A12 after the call on a sentinel line.

### Non-Blocking Analysis

- **B2**: Can move to after `iv_seq()` since uniqueness is a property of the result. Sentinel `let _iv_is_unique = &iv;` follows the existing pattern of `let _endframe_written = ()` etc.
- **B5**: Can move to after `write_u32()` since "serialized same way" is a property of what was just written. Sentinel `let _seq_num_written = &input.sequence_number;`.
- **B3**: Cannot move without creating a new 3-stack elsewhere or losing semantic connection. The `plaintext` parameter is the only place where "encrypted content length equals frame length" makes sense. Left as-is per reviewer guidance.
- **Duplicate B1**: Removed from `FrameLength::new`. The struct-level annotation is sufficient.
