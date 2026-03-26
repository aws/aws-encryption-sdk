# Agent 2 Notes — Round 2 (Review Feedback)

## Review Feedback Summary

Two critical issues to fix:

1. **4-annotation stack in `step_construct_header`** — Reqs 9, 10, 11, 12 were stacked before `serialize_header`. Need to restructure:
   - Req 8 (not released until complete) stays before `build_header_for_encrypt` call
   - Req 9 (streaming release) goes before `serialize_header` call
   - Req 12 (signature feeding) goes on `dw` parameter of multi-line `serialize_header`
   - Reqs 10+11 (header equality/inequality) go after `serialize_header`, on `Ok(header)`

2. **Wrong path prefix** — All annotations used `aws-encryption-sdk-specification/` but codebase convention is `specification/`. Fix all to `specification/`.

## Current State

The implementation annotations in `encrypt.rs` have been reverted to clean state — no authentication-tag annotations exist. The test file still has the old wrong-prefix annotations.

## Plan

1. Add all 12 implementation annotations to `encrypt.rs` with correct `specification/` prefix
2. Follow reviewer's restructuring guidance for `step_construct_header`
3. Fix all test annotations in `test_authentication_tag.rs` to use `specification/` prefix
4. Cross-reference annotations for linked specs (data-format/message-header.md#authentication-tag)

## Annotation Placement Plan

### `build_header_for_encrypt`:
- Req 4 (required EC filtering) → before `required_encryption_context_map` construction
- Req 1 (calculate auth tag) → before `build_header_auth_tag` call
- Cross-ref: data-format/message-header.md#authentication-tag → before same call

### `build_header_auth_tag`:
- Req 5 (IV=0) → before `let iv = vec![0; ...]`
- Req 2 (output of authenticated encryption) → before `aes_encrypt` call
- Req 6 (cipherkey) → on `data_key` param
- Req 7 (empty plaintext) → on `&[]` param
- Req 3 (AAD concatenation) → on `&[raw_header, ...].concat()` param

### `step_construct_header`:
- Req 8 (not released until complete) → before `build_header_for_encrypt` call (implication)
- Req 9 (streaming release) → before multi-line `serialize_header` call (implication)
- Req 12 (signature feeding) → on `dw` param of `serialize_header` (implication)
- Reqs 10+11 (header equality/inequality) → on `Ok(header)` return (implication)
