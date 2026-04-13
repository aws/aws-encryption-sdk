# Work Item: Fix Annotation Placement and Quote Formatting in header_auth.rs

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message-header.md` and `aws-encryption-sdk-specification/client-apis/encrypt.md`
- **Section**: `header-authentication-version-1-0`, `header-authentication-version-2-0`, `iv`, `authentication-tag`, `v1-authentication-tag`, `v2-authentication-tag`
- **Duvet Targets**:
  - `specification/data-format/message-header.md#header-authentication-version-1-0`
  - `specification/data-format/message-header.md#header-authentication-version-2-0`
  - `specification/data-format/message-header.md#iv`
  - `specification/data-format/message-header.md#authentication-tag`
  - `specification/client-apis/encrypt.md#v1-authentication-tag`
  - `specification/client-apis/encrypt.md#v2-authentication-tag`

## Type of Work
FIX_ANNOTATION

## Issues Found

### Issue 1: Blank line after annotation — source file, v1 "in order" annotation
- **File**: `src/message/header_auth.rs`
- **Lines**: 42–45
- **Rule violated**: Core Rule 4 — "The line immediately after the annotation must be executable code (not a comment or blank line)."
- **Current**:
  ```rust
            //= specification/data-format/message-header.md#header-authentication-version-1-0
            //# The V1 Header Authentication MUST consist of, in order,
            //# IV,
            //# and Authentication Tag.
                                          ← blank line 45
            //= specification/client-apis/encrypt.md#v1-authentication-tag
  ```
- **Fix**: Remove the blank line between the annotation block (line 44) and the next annotation block (line 46). The two annotation blocks should be adjacent, with the second one's executable code (`write_bytes(w, header_iv)?;`) serving as the executable line. Alternatively, move this annotation directly above `write_bytes(w, header_iv)?;` and merge the two annotation blocks so the "in order" annotation is immediately before the first write call.

### Issue 2: Blank line after annotation — source file, v2 "Authentication Tag only" annotation
- **File**: `src/message/header_auth.rs`
- **Lines**: 65–67
- **Rule violated**: Core Rule 4 — "The line immediately after the annotation must be executable code (not a comment or blank line)."
- **Current**:
  ```rust
            //= specification/data-format/message-header.md#header-authentication-version-2-0
            //# The V2 Header Authentication MUST consist of the Authentication Tag only.
                                          ← blank line 67
            //= specification/client-apis/encrypt.md#v2-authentication-tag
  ```
- **Fix**: Remove the blank line between the annotation block (line 66) and the next annotation block (line 68).

### Issue 3: Quote line-wrapping mismatch — source file, v2-authentication-tag main quote
- **File**: `src/message/header_auth.rs`
- **Lines**: 23–27
- **Current** (re-wrapped differently from TOML):
  ```rust
        //# With the authentication tag calculated, if the message format version associated
        //# with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0, 
        //# this operation MUST serialize the
        //# [message header authentication](../data-format/message-header.md#header-authentication-version-2-0)
        //# with the following specifics:
  ```
- **TOML quote**:
  ```toml
  With the authentication tag calculated,
  if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
  this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
  ```
- **Additional problem**: Line 24 has a trailing space after `2.0, `.
- **Fix**: Re-wrap the `//# ` lines to match the TOML line breaks exactly, and remove the trailing space:
  ```rust
        //# With the authentication tag calculated,
        //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
        //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
  ```

### Issue 4: Blank line after annotation — test file, v1-authentication-tag IV sub-item
- **File**: `tests/test_header_auth.rs`
- **Lines**: 85–89
- **Rule violated**: Core Rule 4
- **Current**:
  ```rust
    //= specification/client-apis/encrypt.md#v1-authentication-tag
    //= type=test
    //# - [IV](../data-format/message-header.md#iv): MUST have the value of the IV used in the calculation above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
                                          ← blank line 89
    //= specification/client-apis/encrypt.md#v1-authentication-tag
  ```
- **Fix**: Remove the blank line so the two test annotation blocks are adjacent, with the second block's executable code (`let result = ...`) serving as the executable line.

### Issue 5: Blank line after annotation — test file, v2-authentication-tag main quote
- **File**: `tests/test_header_auth.rs`
- **Lines**: 100–105
- **Rule violated**: Core Rule 4
- **Current**:
  ```rust
    //= specification/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# With the authentication tag calculated,
    //# if the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
    //# this operation MUST serialize the [message header authentication](../data-format/message-header.md#header-authentication-version-2-0) with the following specifics:
                                          ← blank line 105
    //= specification/client-apis/encrypt.md#v2-authentication-tag
  ```
- **Fix**: Remove the blank line.

### Issue 6: Blank line after annotation — test file, v2-authentication-tag auth tag sub-item
- **File**: `tests/test_header_auth.rs`
- **Lines**: 106–110
- **Rule violated**: Core Rule 4
- **Current**:
  ```rust
    //= specification/client-apis/encrypt.md#v2-authentication-tag
    //= type=test
    //# - [Authentication Tag](../data-format/message-header.md#authentication-tag): MUST have the value
    //# of the authentication tag calculated above.
                                          ← blank line 110
    let pt = b"v2 encrypt header auth tag test";
  ```
- **Fix**: Remove the blank line so `let pt = ...` immediately follows the annotation.

## Summary

| # | File | Lines | Issue | Severity |
|---|------|-------|-------|----------|
| 1 | `src/message/header_auth.rs` | 44–46 | Blank line after annotation | Rule violation |
| 2 | `src/message/header_auth.rs` | 66–68 | Blank line after annotation | Rule violation |
| 3 | `src/message/header_auth.rs` | 23–27 | Quote line-wrapping differs from TOML + trailing space | Style / consistency |
| 4 | `tests/test_header_auth.rs` | 88–90 | Blank line after annotation | Rule violation |
| 5 | `tests/test_header_auth.rs` | 104–106 | Blank line after annotation | Rule violation |
| 6 | `tests/test_header_auth.rs` | 109–111 | Blank line after annotation | Rule violation |

## What Was Verified Clean

- All annotation paths use the correct `specification/` short prefix inside function bodies ✓
- All annotation types are correct: default (implementation) in source, `type=test` in tests ✓
- No `type=implication` misuse — no module-level cross-references exist (none needed) ✓
- All quotes match TOML content (except Issue 3's line-wrapping) ✓
- Every `type=implementation` in source has a corresponding `type=test` in the test file ✓
- No missing annotations — all TOML requirements are covered ✓
- No `type=implication` annotations that would need `reason=` lines ✓
- Duvet snapshot confirms full `implementation,test` coverage for all sections ✓

## Success Criteria
- [ ] No blank lines between annotation blocks and their executable code
- [ ] v2-authentication-tag quote in source matches TOML line breaks exactly
- [ ] No trailing whitespace in annotation lines
- [ ] `make duvet` still shows full coverage after changes
