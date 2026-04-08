# Agent 2 Notes — Round 2 (Review Feedback)

## Feedback Summary

4 issues to fix, all annotation placement:

1. **Sequence number**: 4 annotation blocks stacked before `write_u32` → split to 2 before `write_u32` + 2 on anchor after
2. **Encrypted content**: 4 blocks before `let _encrypted_content_written` → split to 2 + 2
3. **Authentication tag**: 4 blocks before `let _authentication_tag_written` → split to 2 + 2
4. **IV**: blank lines between annotations and `write_bytes` → remove blanks, split to 2 before `write_bytes` + 2 on anchor after

Plus optional: collapsible_if clippy warning in test file.

## Approach

For each stacking violation, the pattern is:
- Keep regular frame annotations (max 2 blocks) immediately before the actual serialization code
- Add a separate anchor line after the serialization code for the final frame annotations (max 2 blocks)

Also discovered the "regular frame preamble" implication annotation was going to create a 3-block stack with the sequence number annotations after removing the blank line. Gave it its own anchor line `let _regular_frame_serialization = ();`.

## Verification

- All 22 tests pass
- Duvet report generates successfully
- No new clippy warnings from our changes
- Every code line has at most 2 annotation blocks before it
