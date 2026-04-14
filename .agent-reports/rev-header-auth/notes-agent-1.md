# Discovery Notes — rev-header-auth

## Analysis Summary

Performed a full revision check of the header-auth file pair:
- `src/message/header_auth.rs`
- `tests/test_header_auth.rs`

## TOML Sections Checked

1. `data-format/message-header/header-authentication-version-1-0.toml` — 1 requirement
2. `data-format/message-header/header-authentication-version-2-0.toml` — 1 requirement
3. `data-format/message-header/iv.toml` — 2 requirements
4. `data-format/message-header/authentication-tag.toml` — 2 requirements
5. `client-apis/encrypt/v1-authentication-tag.toml` — 3 requirements
6. `client-apis/encrypt/v2-authentication-tag.toml` — 2 requirements

Total: 11 unique TOML `[[spec]]` entries.

## Verification Results

All 11 requirements have:
- ✅ Implementation annotations in source file (default type, no type line)
- ✅ Test annotations in test file (type=test)
- ✅ Exact quote match against TOML
- ✅ Correct path prefix (`specification/` inside function bodies)
- ✅ No blank lines between annotation blocks and executable code
- ✅ No misuse of `type=implication` inside function bodies
- ✅ No missing reason lines

## Potential Spec Gaps

None identified. The code behaviors align well with the spec requirements.
The read functions use `get_iv_length(suite)` and `get_tag_length(suite)`
which derive lengths from the algorithm suite, matching the spec's
requirement that lengths equal the algorithm suite's defined values.
The v2 read function creates a zero IV (`vec![0u8; ...]`) which is
an implementation detail not explicitly called out in the v2 spec section,
but this is consistent with v2 not having an IV field in the header auth.

## Spec Structure Traceability

1. **Logical flow**: Write path dispatches on message version (v1 vs v2),
   then serializes fields in order. Read path does the same in reverse.
2. **Code constructs**: `match` arms for version dispatch, `write_bytes`/`read_vec`
   calls for field serialization/deserialization.
3. **No sub-items missed**: The v1-authentication-tag section has two sub-items
   (IV and Authentication Tag) — both annotated individually at the correct
   `write_bytes` calls.
4. **No structural mistakes identified**: The annotation placement is clean
   and follows the patterns guide correctly.
