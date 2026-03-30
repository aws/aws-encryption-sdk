# Agent 2 Notes — decrypt-body (Cycle 2: Add Missing Test Annotation)

## Pre-Implementation Reasoning

### 1. What are the logical steps?
Only one task: add a `type=test` annotation for the final frame hold-back requirement.

### 2. Point of fulfillment?
The requirement: "Any plaintext decrypted from unframed data or a final frame in a streamed Decrypt operation MUST NOT be released until signature verification successfully completes."

The test proves this by: encrypting with a signing algorithm suite, tampering with the signature bytes, and verifying that decrypt fails (meaning the final frame plaintext was never released because signature verification failed).

### 3. Sub-items?
No — single requirement, single test.

### 4. Can a reviewer read this top-to-bottom?
Yes — one new test function at the end of the existing test file.

### 5. What existing code does something similar?
`test_decrypt_streaming_releases_regular_frames` in the same test file uses a signing algorithm suite. The new test follows the same pattern but tampers with the signature to prove hold-back.
