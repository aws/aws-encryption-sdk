# Review History — header/structure

## Round 1

## Review: CHANGES REQUESTED

### Summary
The primary deliverable — the implementation annotation for the serialization order requirement — is missing from `header.rs`. The test file exists and tests pass, but has annotation target path issues. Two critical issues and one important issue must be fixed.

### Critical Issues (Must Fix)

1. **ANNOTATION_MISSING**: Implementation annotation for Requirement 2 not present in `header.rs`
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`
   - **Line/Area**: `serialize_header()` function (around line 148)
   - **Problem**: The implementation summary claims an annotation was added for "The header MUST be serialized as, in order, Header Body, and Header Authentication." but the annotation does not exist in the file. The `serialize_header` function has no duvet annotation before or inside it. Searching the file for "serialized as" or "Header Body" returns zero matches.
   - **Fix**: Add the annotation inside `serialize_header()`, immediately before `let mut w = Vec::new();`:
     ```rust
     //= specification/data-format/message-header.md#structure
     //# The header MUST be serialized as, in order,
     //# Header Body,
     //# and Header Authentication.
     let mut w = Vec::new();
     ```
     Note the target path must be `specification/...` (not `aws-encryption-sdk-specification/...`) to match the TOML target and the existing annotation at line 29.

2. **ANNOTATION_TARGET**: Test file annotations use wrong target path prefix
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_header_structure.rs`
   - **Lines**: 37, 47
   - **Problem**: Both test annotations use `aws-encryption-sdk-specification/data-format/message-header.md#structure` but the TOML target is `specification/data-format/message-header.md#structure`. The existing implementation annotation at `header.rs:29` uses `specification/...`. The annotations should match the TOML target.
   - **Fix**: Change both annotation targets from `aws-encryption-sdk-specification/data-format/message-header.md#structure` to `specification/data-format/message-header.md#structure`.

### Important Issues

3. **ANNOTATION_TYPE**: Unnecessary `type=implementation` on existing annotations (pre-existing, non-blocking)
   - **File**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/header.rs`
   - **Lines**: 99, 117
   - **Problem**: Per duvet patterns guide, `implementation` is the default type and MUST be omitted. These lines have explicit `//= type=implementation`.
   - **Note**: These are pre-existing and not part of this work item. Non-blocking, but worth noting for future cleanup.

### Observations (Non-Blocking)

4. **Duvet config scope**: The duvet config at `.duvet/config.toml` only scans `./src/**/*.rs`. The `tests/` directory is not included as a source pattern. This means `type=test` annotations in test files are invisible to duvet. This is a systemic issue affecting all test files in the codebase, not specific to this work item. The work item's success criterion "duvet report shows no gaps for `specification/data-format/message-header.md#structure`" cannot be fully satisfied for test coverage until the duvet config is updated. This is informational — do not block on this.

5. **Test quality**: The round-trip tests are indirect proofs (successful decrypt proves correct serialization). This matches the established pattern in `test_construct_a_frame.rs` and `test_construct_the_body.rs`. Acceptable.

### Action Required
Fix issues #1 and #2, then resubmit.
