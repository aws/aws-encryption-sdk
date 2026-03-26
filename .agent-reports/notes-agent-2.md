# Agent 2 Notes — Test Update: Parse Output Bytes

## Pre-Implementation Reasoning

### 1. What are the logical steps?
1. Create a `count_frames` helper that parses ciphertext bytes to count regular and final frames
2. Create an `encrypt_with_frame_length` helper (duplicated from test_construct_a_frame.rs)
3. Update each test to add structural assertions using `count_frames`
4. Keep existing round-trip assertions and duvet annotations

### 2. Approach for `count_frames`
The simplest approach: scan for the 0xFFFFFFFF ENDFRAME marker to find the final frame boundary.
- Walk from body start: try to find SeqNum=1 (regular frame start) or 0xFFFFFFFF+SeqNum=1 (final frame start)
- For regular frames: advance by 4(seq) + 12(iv) + frame_length(content) + 16(tag)
- For final frames: read content_length at offset +20, advance by 4(end) + 4(seq) + 12(iv) + 4(len) + content_length + 16(tag)
- Return (regular_count, final_count)

Also add a `final_frame_content_length` helper to extract the content length from the final frame.

### 3. What existing code does something similar?
`test_construct_a_frame.rs` already has:
- `encrypt_with_frame_length` helper
- Pattern of scanning for ENDFRAME marker and sequence numbers
- Pattern of parsing final frame content length at offset +20 from ENDFRAME marker

### 4. Key insight for exact-match test
When plaintext length == frame_length, the implementation constructs a FINAL frame (not regular).
The test should verify: 0 regular frames, 1 final frame, content_length == frame_length.
