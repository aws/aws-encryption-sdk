# Agent 1 Notes — rev-header-auth

## Spec-Aligned Structure Analysis

The header-auth code covers two logical flows:

1. **Write path** (`write_header_auth_tag` → `write_header_auth_tag_v1` / `write_header_auth_tag_v2`):
   - Dispatch by message version (encrypt.md v1/v2-authentication-tag main quotes)
   - V1: write IV then auth tag (data-format "in order" + encrypt.md sub-items)
   - V2: write auth tag only (data-format "Authentication Tag only" + encrypt.md sub-item)

2. **Read path** (`read_header_auth_tag` → `read_header_auth_tag_v1` / `read_header_auth_tag_v2`):
   - V1: read IV (length + bytes) then auth tag (length + bytes)
   - V2: read auth tag (length + bytes), synthesize zero IV

The code structure mirrors the spec well. Annotations are placed at the right code constructs.

## Most Likely Structural Mistake

The blank-line-after-annotation pattern appears to be a stylistic habit of separating "structural roadmap" annotations from "sub-item" annotations. The intent is readability, but it violates Core Rule 4. The fix is simple: remove the blank lines.

## Potential Spec Gaps

### V2 read synthesizes a zero IV
- **Code location**: `read_header_auth_tag_v2`, line `let header_iv = vec![0u8; get_iv_length(suite) as usize];`
- **Behavior**: When reading a V2 header auth, the code creates a zero-filled IV vector even though V2 has no IV field in the wire format.
- **Why it matters**: Interoperability — other implementations must agree on what the IV value is for V2 messages (used in `HeaderAuth::AESMac` struct). If the struct always requires an IV field, the zero-fill convention must be consistent.
- **Suggested spec requirement**: "When deserializing a V2 header authentication, the IV SHOULD be set to a zero-filled byte sequence of length equal to the algorithm suite's IV length."
- **Note**: This may be intentional and handled by the algorithm suite's usage of the IV. Advisory only.
