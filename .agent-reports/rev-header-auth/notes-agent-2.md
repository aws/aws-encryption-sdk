# Agent 2 Notes — rev-header-auth

## Pre-Implementation Reasoning

### 1. What are the logical steps?
This is an annotation-only fix. No executable code changes. Six issues to fix:
1. Remove blank line at source ~line 45 (between v1 "in order" and v1-auth-tag IV annotation)
2. Remove blank line at source ~line 67 (between v2 "auth tag only" and v2-auth-tag annotation)
3. Re-wrap v2-authentication-tag quote at source lines 23-27 to match TOML line breaks, remove trailing space
4. Remove blank line at test ~line 89 (between v1-auth-tag IV sub-item and auth-tag sub-item)
5. Remove blank line at test ~line 105 (between v2-auth-tag main quote and auth-tag sub-item)
6. Remove blank line at test ~line 110 (between v2-auth-tag auth-tag sub-item and `let pt`)

### 2. Point of fulfillment for each fix
Each fix is at the annotation itself — removing blank lines or re-wrapping quote text.

### 3. Sub-items?
N/A — no new annotations being added.

### 4. Can a reviewer read top-to-bottom?
Yes — these are minimal formatting fixes. The diff will be obvious.

### 5. Existing similar code?
The rest of the file already follows the correct pattern (no blank lines between annotation blocks).
