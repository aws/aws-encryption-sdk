# Work Item: Add Test Annotations for data-format/message.md#structure (Footer-Related Requirements)

## Specification
- **File**: `aws-encryption-sdk-specification/data-format/message.md`
- **Section**: `structure`
- **Duvet Target**: `specification/data-format/message.md#structure`

## Type of Work
ADD_TESTS

## Requirements to Address

### Requirement 1
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the [message header](message-header.md) contains an [algorithm suite](../framework/algorithm-suites.md) in the
  [algorithm suite ID](message-header.md#algorithm-suite-id) field that contains a
  [signature algorithm](../framework/algorithm-suites.md#signature-algorithm), the message MUST also contain a
  [message footer](message-footer.md) serialized after the [message body](message-body.md).
  ```
- **Current State**: needs-test (implementation annotation exists in `footer.rs` line 20 and `encrypt.rs` line 361, but no `type=test` annotation anywhere)

### Requirement 2
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer.
  ```
- **Current State**: needs-test (implementation annotation exists in `encrypt.rs` line 395, but no `type=test` annotation anywhere)

### Requirement 3
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  If the algorithm suite contain an unrecognized signature algorithm, the operation MUST raise an error.
  ```
- **Current State**: needs-test (implementation annotation exists in `encrypt.rs` line 399, but no `type=test` annotation anywhere)

### Requirement 4
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The message MUST begin with [Message Header](message-header.md)
  ```
- **Current State**: needs-test (implementation annotation exists in `encrypt.rs` line 145, but no `type=test` annotation anywhere)
- **Placement**: unclear — this is about header ordering, not footer behavior. May belong in a header/message-level test file rather than `test_footer.rs`.

### Requirement 5
- **Level**: MUST
- **Exact Quote** (from TOML):
  ```toml
  - The [Message Body](message-body.md) MUST follow the Message Header
  ```
- **Current State**: needs-test (implementation annotation exists in `encrypt.rs` line 161, but no `type=test` annotation anywhere)
- **Placement**: unclear — this is about body ordering, not footer behavior. May belong in a header/message-level test file rather than `test_footer.rs`.

## Existing Code Context

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/footer.rs`
```rust
//= specification/data-format/message.md#structure
//# If the [message header](message-header.md) contains an [algorithm suite](../framework/algorithm-suites.md) in the
//# [algorithm suite ID](message-header.md#algorithm-suite-id) field that contains a
//# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm), the message MUST also contain a
//# [message footer](message-footer.md) serialized after the [message body](message-body.md).
pub(crate) fn write_footer(
    w: &mut dyn SafeWrite,
    signature: &[u8],
) -> Result<(), Error> {
```

### Source File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/encrypt.rs` (lines 395-403)
```rust
        //= specification/data-format/message.md#structure
        //# If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer.
        aws_mpl_legacy::suites::SignatureAlgorithm::None => {}
    
        //= specification/data-format/message.md#structure
        //# If the algorithm suite contain an unrecognized signature algorithm, the operation MUST raise an error.
        _ => {
            return Err("Unrecognized signature algorithm in algorithm suite".into());
        }
```

### Test File: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_footer.rs`
```rust
#[tokio::test(flavor = "multi_thread")]
async fn test_footer_present_with_signing_suite() {
    //= specification/data-format/message-footer.md#overview
    //= type=test
    //# When an [algorithm suite](../framework/algorithm-suites.md) includes a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the [message](message.md) MUST contain a footer.

    let ct_signing = encrypt_with_signing_suite(b"footer presence test").await;
    let ct_no_signing = encrypt_without_signing_suite(b"footer presence test").await;
    // ...
}
```

## Implementation Guidance

- **Requirements 1-3** (footer-related) should have test annotations added to `test_footer.rs`.
- **Requirements 4-5** (header/body ordering) are not footer-specific. Agent 2 should evaluate whether to add them to `test_footer.rs` or defer to a message-structure test file. If deferred, mark with `type=todo` or skip.
- The existing `test_footer_present_with_signing_suite` test already exercises Requirement 1 (footer present with signing suite) and partially Requirement 2 (it encrypts with a non-signing suite and compares lengths). Add `message.md#structure` test annotations to this existing test.
- For Requirement 2 ("MUST NOT contain a message footer"), the existing test already encrypts with `AlgAes256GcmHkdfSha512CommitKey` (non-signing) and could verify the absence of footer bytes. Add a test annotation to the existing test or add a small dedicated test.
- For Requirement 3 ("unrecognized signature algorithm MUST raise an error"), this is difficult to test directly because `SignatureAlgorithm` is a Rust enum and you can't easily construct an "unrecognized" variant from the public API. Consider annotating with `type=implication` and `reason=` explaining that the Rust enum's exhaustive match ensures this, or find a way to test it.
- Follow the annotation pattern in `test_footer.rs` — each test function gets the `//= specification/...` + `//= type=test` + `//#` block immediately inside the function body.

### Spec-Aligned Structure
The spec describes this flow:
1. Message begins with header → annotate at test verifying header is first bytes (Requirement 4, **Placement**: unclear)
2. Body follows header → annotate at test verifying body follows header (Requirement 5, **Placement**: unclear)
3. Signing suite → footer present after body → annotate at `test_footer_present_with_signing_suite` (Requirement 1)
4. Non-signing suite → no footer → annotate at test verifying no footer bytes (Requirement 2)
5. Unrecognized signature → error → annotate at test or implication (Requirement 3)

Sub-items to annotate individually:
- Requirement 1 → at the existing `test_footer_present_with_signing_suite` test (add a second annotation block for `message.md#structure`)
- Requirement 2 → at a test that encrypts with non-signing suite and verifies no trailing footer bytes
- Requirement 3 → at a test or implication annotation for unrecognized signature algorithm error

## Targeted Tests
- `test_footer::test_footer_present_with_signing_suite` — already tests footer presence with signing suite; add `message.md#structure` test annotation for Requirement 1
- `test_footer::test_footer_signature_calculated_over_header_and_body` — round-trip test that could also serve Requirement 1
- New or extended test needed for Requirement 2 (no footer with non-signing suite)
- New test or implication needed for Requirement 3 (unrecognized signature algorithm error)

## Success Criteria
```bash
cargo test test_footer
make duvet
```
- [ ] Each test passes
- [ ] duvet report shows no gaps for `specification/data-format/message.md#structure` (test column)
- [ ] duvet report shows no gaps for `specification/data-format/message-footer.md` (already passing)
- [ ] All requirements have `type=implementation` or `type=implication` (not `type=todo`)
- [ ] All implementations have corresponding `type=test`
