# Agent 2 Notes — header_auth (Round 2)

## Pre-Implementation Reasoning

### 1. Logical steps in this spec section
1. The authentication tag has a length constraint (already annotated)
2. The authentication tag must be interpreted as bytes (MISSING — this is the work item)

### 2. Point of fulfillment for each requirement
- "The authentication tag MUST be interpreted as bytes" → fulfilled at `let header_auth_tag = read_vec(...)` in both `read_header_auth_tag_v1` and `read_header_auth_tag_v2`, because `read_vec` returns `Vec<u8>` — the tag is stored and handled as raw bytes throughout.

### 3. Sub-items?
No sub-items. Single requirement.

### 4. Reviewer readability
In `read_header_auth_tag_v1`: the auth tag length annotation is already on `let header_auth_tag = read_vec(...)`. Adding the "interpreted as bytes" annotation creates a 2-stack, which is within the limit (same pattern as IV which has length + bytes stacked). 

In `read_header_auth_tag_v2`: same situation — length annotation already present, adding bytes annotation creates a 2-stack.

### 5. Existing similar code
The IV annotations in `read_header_auth_tag_v1` (lines 92-99) are the exact pattern to follow:
- Block 1: length constraint with `type=implication` and `reason=`
- Block 2: "interpreted as bytes" with `type=implication` and `reason=`
- Both before the same `read_vec` call

For tests, `test_v1_header_auth_iv_length_and_bytes` shows the pattern — it has both IV length and IV bytes test annotations in the same test function.

## Plan
1. Add `//= aws-encryption-sdk-specification/data-format/message-header.md#authentication-tag` / `type=implication` / `reason=` / `The authentication tag MUST be interpreted as bytes.` annotation before `let header_auth_tag = read_vec(...)` in `read_header_auth_tag_v1` (after existing length annotation)
2. Same in `read_header_auth_tag_v2`
3. Add `type=test` annotation for this requirement in `test_v1_header_auth_tag_length` and `test_v2_header_auth_tag_length` tests
4. Verify with cargo test and make duvet
