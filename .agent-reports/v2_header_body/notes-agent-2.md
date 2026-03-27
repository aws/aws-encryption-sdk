# Agent 2 Notes — v2_header_body (ADD_TESTS)

## Step 1.5: Pre-Implementation Reasoning

### 1. Logical steps in this spec section
The spec describes 9 fields that must be serialized in order for V2 headers:
1. Gate condition: algorithm suite version is 2.0
2. Version field (0x02)
3. Algorithm Suite ID (2 bytes matching suite used)
4. Message ID (32 bytes, random)
5. AAD (encryption context serialization)
6. Encrypted Data Keys
7. Content Type (0x02 = framed)
8. Frame Length
9. Algorithm Suite Data (commit key)

### 2. Point of fulfillment for each requirement
All 18 requirements already have implementation annotations in `v2_header_body.rs`.
This work item is ADD_TESTS only — I need `type=test` annotations in the test file.

Each test annotation's point of fulfillment is the assertion that verifies the requirement:
- Req 1 (gate condition): assert version byte is 0x02 (proves V2 suite was used)
- Req 2-3 (Version serialized, value 2.0): assert `ct[0] == 0x02`
- Req 4 (Version field MUST be 02): assert `ct[0] == 0x02`
- Req 5-6 (Algorithm Suite ID): assert suite ID bytes match expected
- Req 7-8 (Message ID, randomness): assert 32-byte length, two encryptions differ
- Req 9-10 (AAD): round-trip with encryption context
- Req 11-12 (EDKs): round-trip proves EDKs work
- Req 13-14 (Content Type): round-trip or byte inspection
- Req 15-16 (Frame Length): round-trip
- Req 17-18 (Algorithm Suite Data): round-trip (commit key verified on decrypt)

### 3. Sub-items
Each field has two requirements: "MUST be serialized according to" + "The value MUST be/correspond to".
These pair naturally into the same test function with separate annotation blocks.

### 4. Reviewer readability
Follow the exact pattern from `test_v1_header_body.rs`:
- One test function per field
- Each test has 2 annotation blocks (serialization + value)
- Req 4 (header-body-version-2-0 TOML) gets its own annotation in the version test
- Existing `test_v2_header_message_id` already covers the data-format spec; add encrypt.md annotations

### 5. Existing similar code
`test_v1_header_body.rs` — exact same pattern, just for V1.
The existing `test_v2_header_body.rs` already has helpers (`encrypt_default`, `parse_v2_header_field_offsets`).
Need a `round_trip_v2` helper for AAD/EDK/content-type/frame-length/suite-data tests.

## Implementation Plan
1. Add `round_trip_v2` helper to existing test file
2. Add test functions following V1 pattern:
   - `test_v2_header_serialized` (Req 1)
   - `test_v2_header_version` (Req 2, 3, 4)
   - `test_v2_header_algorithm_suite_id` (Req 5, 6)
   - Update existing `test_v2_header_message_id` with Req 7, 8 annotations
   - `test_v2_header_aad` (Req 9, 10)
   - `test_v2_header_encrypted_data_keys` (Req 11, 12)
   - `test_v2_header_content_type` (Req 13, 14)
   - `test_v2_header_frame_length` (Req 15, 16)
   - `test_v2_header_algorithm_suite_data` (Req 17, 18)
