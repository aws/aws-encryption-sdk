# Agent 2 Notes — decrypt-body (Cycle 2)

## SCOPE_CREEP Revert Not Possible

The reviewer asked to revert two encrypt-side annotation changes in `construct_frame`:
1. Line ~671: `"The Sequence Number End MUST only be serialized for the final frame."` 
2. Line ~710: `"The Encrypted Content Length MUST only be serialized for the final frame."`

These old quotes do not exist anywhere in:
- `aws-encryption-sdk-specification/client-apis/encrypt.md` (the spec file)
- `construct-a-frame.toml` (the duvet TOML)

The spec was updated (the `aws-encryption-sdk-specification` submodule is listed in `git diff --name-only`) and the old quotes were replaced with the new list-style quotes. Reverting the annotations causes `make duvet` to fail with:
```
× could not find text in section "construct-a-frame" of specification/client-apis/encrypt.md
```

The current annotations use quotes that match the current spec state. They cannot be reverted without also reverting the spec changes, which is out of scope for this task (and the task instructions say to only modify body.rs).

## ANNOTATION_PLACEMENT Fix Applied Successfully

Moved the parent annotation `"For a final frame, each field MUST be deserialized according to its specification:"` from inside the `if seq_num == ENDFRAME_SEQUENCE_NUMBER` block to before the `if` statement. This:
- Reduces the annotation stack inside the block from 3 to 2
- Places the parent annotation at the block entry point (Pattern 3: general behavior at block start)
- The executable line after the annotation is the `if` statement itself
