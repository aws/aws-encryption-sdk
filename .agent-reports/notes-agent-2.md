# Agent 2 Notes — Round 2 (Review Feedback)

## Feedback Received
Two issues from Agent 3:
1. 3-annotation stack before regular frame `construct_frame` call
2. `type=implication` without `reason=` line on cross-reference annotation

## Changes Made

### Issue 1: 3-annotation stack
Moved Req 5 ("If there are enough input plaintext bytes consumable...") from immediately before `construct_frame` to immediately after the `next_char.is_none()` break. This is the correct decision point: at that location, we know `in_size == frame_length` (enough bytes) AND `next_char.is_some()` (more bytes remain), which is exactly what Req 5 describes.

This leaves only 2 annotation blocks before `construct_frame`: Req 1 (regular frame conformance) + cross-ref (regular-frame serialization order), which is within the 2-block limit.

### Issue 2: Missing reason= line
Added `//= reason=Serialization order is enforced by construct_frame's implementation; this annotation traces the structural requirement` to the `type=implication` cross-reference annotation for `message-body.md#regular-frame`.

### Bonus cleanup
Removed redundant `//= type=implementation` from the `framed-data` and `regular-frame-sequence-number` annotations that were part of my original changes. Pre-existing `type=implementation` lines elsewhere in the file were left untouched (out of scope).
