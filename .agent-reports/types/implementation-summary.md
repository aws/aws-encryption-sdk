## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs` — Added 3 `type=implication` duvet annotations for Plaintext Length Bound requirements

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs
```

### Requirements Addressed
- ✅ `Implementations SHOULD ensure that a caller is not able to specify both a [plaintext](#plaintext) with known length and a [Plaintext Length Bound](#plaintext-length-bound) by construction.` — annotated as `type=implication` on `EncryptInput` struct
- ✅ `If a caller is able to specify both an input [plaintext](#plaintext) with known length and a [Plaintext Length Bound](#plaintext-length-bound), the [Plaintext Length Bound](#plaintext-length-bound) MUST NOT be used during the Encrypt operation and MUST be ignored.` — annotated as `type=implication` on `EncryptInput` struct
- ✅ `If the [plaintext](#plaintext) is of unknown length, the caller MAY also input a [Plaintext Length Bound](#plaintext-length-bound).` — annotated as `type=implication` on `EncryptStreamInput.data_size` field

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: None — all three annotations use `type=implication`, which satisfies both implementation and test checks per duvet-patterns.md
- **Number of `type=test` annotations added**: 0 (not needed for `type=implication`)
- **Test function names**: N/A

### Proposed Commit Message

```
feat(encrypt): add duvet annotations for Plaintext Length Bound requirements

Add three missing type=implication annotations to types.rs for the
Plaintext Length Bound requirements in encrypt.md#input:

- SHOULD ensure caller cannot specify both known-length plaintext and
  Plaintext Length Bound (on EncryptInput struct)
- MUST NOT use/MUST ignore Plaintext Length Bound when both specified
  (on EncryptInput struct)
- MAY input Plaintext Length Bound for unknown-length plaintext
  (on EncryptStreamInput.data_size field)

All three are satisfied by construction: EncryptInput has no
plaintext_length_bound field, and EncryptStreamInput.data_size
serves as the optional bound for streaming input.

Spec: aws-encryption-sdk-specification/client-apis/encrypt.md#input
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf compliance
duvet extract -o compliance -f MARKDOWN ...
duvet report ...
    Scanning sources
     Scanned 539 sources 27ms
     Parsing annotations
      Parsed 2473 annotations 93ms
     Loading specifications
      Loaded 68 specifications 20ms
     Mapping sections
      Mapped 485 sections 9ms
    Matching references
     Matched 4204 references 5ms
     Sorting references
      Sorted 4204 references 19ms
     Writing specification_compliance_report.html
       Wrote specification_compliance_report.html 18ms
```

### Test Results (actual command output)
```
$ cargo check
warning: `aws-esdk` (lib) generated 4 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.83s
```

Note: `cargo test` has 8 pre-existing failures in `test_authentication_tag` due to AWS credential issues (UnrecognizedClientException), unrelated to this change.

### Notes
- All three requirements are `type=implication` because they are satisfied by the Rust type system / struct construction, not by runtime logic.
- Each annotation includes a `reason=` line explaining how the struct design satisfies the requirement.
- The annotation pattern follows existing `type=implication` + `reason=` usage on `EncryptInput.source` and `EncryptInput.max_encrypted_data_keys`.
- No cross-references needed: the quoted text contains markdown links to `#plaintext` and `#plaintext-length-bound` which are anchors within the same `encrypt.md#input` section, not links to other spec files.
