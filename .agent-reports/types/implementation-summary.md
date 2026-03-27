## Changes Made

### Files Modified
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs` — Added 3 `type=implication` annotations on `EncryptInput` fields for `client.md#initialization` requirements (commitment policy option, max EDKs option, max EDKs default)
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs` — Added 6 `type=test` annotations covering all 4 initialization requirements

### How to View Changes
```bash
git diff -- AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/types.rs AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs
```

### Requirements Addressed
- ✅ `- On client initialization, the caller MUST have the option to provide a [commitment policy](#commitment-policy).` — implication on `EncryptInput.commitment_policy` field + tested
- ✅ `- On client initialization, the caller MUST have the option to provide a [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys).` — implication on `EncryptInput.max_encrypted_data_keys` field + tested
- ✅ `If no [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys) is provided the default MUST result in no limit on the number of encrypted data keys (aside from the limit imposed by the [message format](../format/message-header.md)).` — implication on `EncryptInput.max_encrypted_data_keys` field + tested
- ✅ `If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).` — already has implication in `decrypt.rs` + now tested

### Test Annotations Added (REQUIRED)
- **Test file(s) modified**: `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/tests/test_create_esdk_client.rs`
- **Number of `type=test` annotations added**: 6 for 4 requirements (some requirements tested on both EncryptInput and DecryptInput)
- **Test function names**:
  - `test_encrypt_input_default_commitment_policy` — tests Req 4 default commitment policy
  - `test_decrypt_input_default_commitment_policy` — tests Req 4 default commitment policy
  - `test_encrypt_input_default_max_edks_is_none` — tests Req 3 default no limit
  - `test_decrypt_input_default_max_edks_is_none` — tests Req 3 default no limit
  - `test_encrypt_input_custom_commitment_policy` — tests Req 1 caller can provide commitment policy
  - `test_encrypt_input_custom_max_edks` — tests Req 2 caller can provide max EDKs

### Proposed Commit Message

```
feat(client): add duvet annotations for client.md#initialization requirements

Add type=implication annotations on EncryptInput fields for three
client initialization requirements: commitment policy option,
maximum encrypted data keys option, and max EDKs default behavior.

Add six type=test annotations in test_create_esdk_client.rs covering
all four MUST requirements in the initialization section, including
the pre-existing commitment policy default requirement.

Spec section: specification/client-apis/client.md#initialization
```

### Duvet Verification (actual command output)
```
$ make duvet
rm -rf .duvet/reports .duvet/requirements
duvet report
  Extracting requirements
   Extracted requirements from 9 specifications 37ms
    Scanning sources
     Scanned 154 sources 4ms
     Parsing annotations
      Parsed 896 annotations 30ms
     Loading specifications
      Loaded 13 specifications 16ms
     Mapping sections
      Mapped 118 sections 15ms
    Matching references
     Matched 1678 references 4ms
     Sorting references
      Sorted 1678 references 12ms
     Writing .duvet/reports/report.html
       Wrote .duvet/reports/report.html 19ms
     Writing .duvet/snapshot.txt
       Wrote .duvet/snapshot.txt 975µs
```

### Test Results (actual command output)
```
$ cargo test --test test_create_esdk_client
running 7 tests
test test_decrypt_input_default_commitment_policy ... ok
test test_encrypt_input_custom_commitment_policy ... ok
test test_encrypt_input_default_max_edks_is_none ... ok
test test_encrypt_input_default_commitment_policy ... ok
test test_encrypt_input_custom_max_edks ... ok
test test_decrypt_input_default_max_edks_is_none ... ok
test test_net_retry_flag ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Notes
- All 4 MUST requirements in `client.md#initialization` now show `implication,test` coverage in the duvet snapshot
- The SHOULD requirement ("Once a commitment policy has been set it SHOULD be immutable") was already covered by an existing `type=implication` annotation in `decrypt.rs` — no changes needed
- The commitment policy requirement (Req 4) already had a `type=implication` annotation in `decrypt.rs` — only `type=test` annotations were added for it
- Annotations placed on `EncryptInput` only (not all 4 input structs) since the fields are structurally identical across all input types and `EncryptInput` is the primary input struct
- Pre-existing clippy warnings (unused doc comment in encrypt.rs, unreachable patterns in materials.rs, missing docs on encrypt_stream, collapsible_if in header body files) are unrelated to this change
