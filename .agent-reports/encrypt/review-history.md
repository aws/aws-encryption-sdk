# Review History — encrypt/authentication-tag

## Round 1

## Review: CHANGES REQUESTED

### Summary
Annotation placement and test quality are generally good, with excellent per-parameter annotations on the `aes_encrypt` call. However, there is a hard-limit annotation stacking violation in `step_construct_header` (4 blocks before one line) and all new annotations use the wrong target path prefix (`aws-encryption-sdk-specification/` instead of `specification/`).

### Critical Issues (Must Fix)

1. **ANNOTATION_PLACEMENT — 4-annotation stack in `step_construct_header`**:
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/encrypt.rs`
   - **Line/Area**: Lines ~328-349, before `header::serialize_header(&header, ciphertext, dw)?;`
   - **Problem**: Four annotation blocks (Reqs 9, 10, 11, 12) are stacked before a single line of code. The hard limit is 2. This is an automatic rejection.
   - **Fix**: Restructure as follows:
     1. Keep Req 9 (streaming release) before `serialize_header` — it directly relates to the serialization call.
     2. Reformat `serialize_header` to multi-line and place Req 12 (signature feeding) on the `dw` parameter, since the requirement is specifically about feeding bytes to the signature algorithm via DigestWriter.
     3. Move Reqs 10+11 (header equality / inequality fail) AFTER the `serialize_header` call, on the `Ok(header)` return line. These are about the output having the correct header — the return of the header IS the point where the output header is determined. Keep the `reason=` lines.

     Target structure:
     ```rust
     //= specification/client-apis/encrypt.md#authentication-tag
     //= type=implication
     //= reason=serialize_header writes the complete header to ciphertext, releasing it
     //# If this operation is streaming ...
     //# the serialized message header MUST be released.
     header::serialize_header(
         &header,
         ciphertext,
         //= specification/client-apis/encrypt.md#authentication-tag
         //= type=implication
         //= reason=dw (DigestWriter) feeds header bytes to the signature algorithm during serialization
         //# If the algorithm suite contains a signature algorithm ...
         dw,
     )?;
     //= specification/client-apis/encrypt.md#authentication-tag
     //= type=implication
     //= reason=single code path: the header built here is serialized directly to output
     //# The encrypted message output by the Encrypt operation MUST have a message header equal
     //# to the message header calculated in this step.
     //= specification/client-apis/encrypt.md#authentication-tag
     //= type=implication
     //= reason=single code path means header inequality is impossible by construction
     //# If the message headers are not equal, the Encrypt operation MUST fail.
     Ok(header)
     ```

2. **ANNOTATION_TARGET — Wrong path prefix on ALL new annotations**:
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/encrypt.rs` and `tests/test_authentication_tag.rs`
   - **Line/Area**: All 14 new annotation target lines in encrypt.rs, all 9 in test file
   - **Problem**: New annotations use `aws-encryption-sdk-specification/client-apis/encrypt.md#authentication-tag` but every existing annotation in the entire codebase uses `specification/client-apis/encrypt.md#authentication-tag`. The TOML target is `specification/client-apis/encrypt.md#authentication-tag`. While duvet resolves both via symlink, this is inconsistent with the rest of the codebase and will confuse future reviewers.
   - **Fix**: Replace `aws-encryption-sdk-specification/` with `specification/` in ALL new annotation target lines. This applies to:
     - All `//= aws-encryption-sdk-specification/client-apis/encrypt.md#authentication-tag` → `//= specification/client-apis/encrypt.md#authentication-tag`
     - `//= aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag` → `//= specification/data-format/message-header.md#authentication-tag`
     - `//= aws-encryption-sdk-specification/framework/algorithm-suites.md#encryption-algorithm` → `//= specification/framework/algorithm-suites.md#encryption-algorithm`

### Suggestions (Optional Improvements)

1. The Reqs 10+11 stack (2 blocks) on `Ok(header)` after the restructure is acceptable since it's within the 2-block limit, and these are the positive/negative sides of the same requirement (header equality). No further action needed.
