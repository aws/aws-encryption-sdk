# Agent 3 Notes — footer

## Round 3 — Adversarial Pre-Review

### 1. Does each annotation's next line actually implement THAT requirement?

**write_footer:**
- "This operation MUST then serialize a message footer with the following specifics:" → `let len = u16::try_from(signature.len())...`
  This is a general "serialize footer" annotation at the start of the serialization block. The `let len` line begins the serialization. ✅ Pattern 3 (general behavior at block start).
- "Signature Length MUST be the length of the output of the calculation above." → `let len = u16::try_from(signature.len())...`
  The annotation is about the VALUE of the length field equaling the signature length. The `let len` line computes that value from `signature.len()`. ✅ Direct semantic match — this is where the value is determined.
- "This length of the signature length field MUST be 2 bytes." → `write_u16(w, len)?;`
  `write_u16` writes `len.to_be_bytes()` which is always exactly 2 bytes. The `reason=` line confirms. ✅ Direct semantic match.
- "The signature length field MUST be interpreted as a UInt16." → `write_u16(w, len)?;`
  `write_u16` takes a `u16` and serializes as big-endian. ✅ Direct semantic match.
- "Signature MUST be the output of the calculation above." → `write_bytes(w, signature)?;`
  `write_bytes` writes the raw signature bytes. ✅ Direct semantic match.

**read_footer:**
- "This length of the signature length field MUST be 2 bytes." → `read_seq_u16(r, raw)`
  `read_seq_u16` calls `read_u16` which reads `[0u8; 2]` — exactly 2 bytes. ✅ Valid via delegation, reason line explains.
- "The signature length field MUST be interpreted as a UInt16." → `read_seq_u16(r, raw)`
  `read_seq_u16` calls `read_u16` which returns `u16::from_be_bytes(result)`. ✅ Valid via delegation, reason line explains.

### 2. Annotation stacking check

**write_footer — `let len` line:** 2 blocks. ✅ Under limit.
**write_footer — `write_u16(w, len)?;` line:** 2 blocks. ✅ Under limit.
**write_footer — `write_bytes(w, signature)?;` line:** 1 block. ✅ Under limit.
**read_footer — `read_seq_u16(r, raw)` line:** 2 blocks. ✅ Under limit.

No stacking violations. The Round 2 feedback (move encrypt.md "Signature Length" to `let len`) has been correctly applied.

### 3. Per-block isolation evaluation

**Block: encrypt.md "serialize a message footer" → `let len`**
Context reset: "This operation MUST then serialize a message footer with the following specifics" and `let len = u16::try_from(signature.len())`. This is a general "start of serialization" annotation. The code begins the serialization process. ✅ Pattern 3 — general behavior at block start.

**Block: encrypt.md "Signature Length MUST be the length" → `let len`**
Context reset: "Signature Length MUST be the length of the output of the calculation above" and `let len = u16::try_from(signature.len())`. The annotation says the length field must equal the signature length. The code computes `len` from `signature.len()`. ✅ Immediately obvious — this is where the value is determined.

**Block: message-footer.md "MUST be 2 bytes" → `write_u16(w, len)?;`**
Context reset: "signature length field MUST be 2 bytes" and `write_u16(w, len)`. The "u16" = 2 bytes. Reason line: "write_u16 writes exactly 2 bytes as a big-endian u16". ✅ Immediately obvious.

**Block: message-footer.md "MUST be interpreted as UInt16" → `write_u16(w, len)?;`**
Context reset: "signature length field MUST be interpreted as a UInt16" and `write_u16(w, len)`. The "u16" = UInt16. Reason line: "write_u16 serializes the value as a big-endian UInt16". ✅ Immediately obvious.

**Block: encrypt.md "Signature MUST be the output" → `write_bytes(w, signature)?;`**
Context reset: "Signature MUST be the output of the calculation above" and `write_bytes(w, signature)`. The code writes the signature bytes. ✅ Immediately obvious.

**Block: message-footer.md "MUST be 2 bytes" → `read_seq_u16(r, raw)` in read_footer**
Context reset: "signature length field MUST be 2 bytes" and `read_seq_u16(r, raw)`. The "u16" hints at 2 bytes. Reason line: "read_seq_u16 calls read_u16 which reads exactly 2 bytes". ✅ Acceptable with reason line.

**Block: message-footer.md "MUST be interpreted as UInt16" → `read_seq_u16(r, raw)` in read_footer**
Context reset: "MUST be interpreted as a UInt16" and `read_seq_u16`. Reason line: "read_seq_u16 calls read_u16 which interprets 2 bytes as a big-endian UInt16". ✅ Acceptable with reason line.

### 4. Semantic relationship check
All annotations semantically relate to their code lines. The Round 2 fix improved the encrypt.md "Signature Length" annotation — it's now on the `let len` line where the value is computed, which is a better semantic fit than the `write_u16` line.

### 5. Sub-items check
Both requirements from the TOML are annotated in both functions. No sub-items missed. ✅

### 6. Code structure mirrors spec
The spec describes Signature Length as a 2-byte UInt16 field. The code writes/reads it as u16. Structure matches. ✅

### 7. Top-to-bottom readability
`write_footer`: Clean flow — compute value (2 annotations) → write length (2 annotations) → write signature (1 annotation). Easy to follow. ✅
`read_footer`: Clean, 2 annotations then the call. ✅

### Step 3: Anti-Rationalization Check

No problems found. No rationalization patterns detected. All Round 2 issues have been addressed.

### Cross-reference check
- encrypt.md "Signature Length" annotation references `../data-format/message-footer.md#signature-length` → message-footer.md#signature-length annotations exist at `write_u16` line ✅
- encrypt.md "Signature" annotation references `../data-format/message-footer.md#signature` → pre-existing gap (no message-footer.md#signature annotation at `write_bytes`). Not Agent 2's responsibility — this was pre-existing.

Links found: 2 relevant cross-references
Cross-refs present: 1/2 (signature cross-ref is pre-existing gap)

### Decision
APPROVED — All Round 2 issues resolved. Stacking is under limit everywhere. Annotations are semantically precise. Reason lines are present and correct.

## Round 2 — Adversarial Pre-Review

### 1. Does each annotation's next line actually implement THAT requirement?

**write_footer:**
- "This length of the signature length field MUST be 2 bytes." → `write_u16(w, len)?;`
  `write_u16` calls `write_bytes(w, &data.to_be_bytes())` where `data` is `u16`. `u16::to_be_bytes()` always produces exactly 2 bytes. ✅ Direct semantic match.
- "The signature length field MUST be interpreted as a UInt16." → `write_u16(w, len)?;`
  `write_u16` takes a `u16` parameter and serializes it as big-endian. The value IS a UInt16. ✅ Direct semantic match.

**read_footer:**
- "This length of the signature length field MUST be 2 bytes." → `read_seq_u16(r, raw)`
  `read_seq_u16` calls `read_u16` which reads `[0u8; 2]` — exactly 2 bytes. ✅ Valid via delegation.
- "The signature length field MUST be interpreted as a UInt16." → `read_seq_u16(r, raw)`
  `read_seq_u16` calls `read_u16` which returns `u16::from_be_bytes(result)`. ✅ Valid via delegation.

### 2. Annotation stacking check

**write_footer — `write_u16(w, len)?;` line:**
3 annotation blocks:
1. encrypt.md#construct-the-signature — "Signature Length MUST be the length..."
2. message-footer.md#signature-length — "MUST be 2 bytes" (implication)
3. message-footer.md#signature-length — "MUST be interpreted as UInt16" (implication)

This is 3 blocks → still at the hard limit (3+ = automatic rejection).

**Fix available**: The encrypt.md annotation about "Signature Length MUST be the length of the output" is about the *value* of the length field equaling the signature length. The `let len = u16::try_from(signature.len())` line is where that value is computed. Moving the encrypt.md annotation to the `let len` line is semantically more precise AND reduces the stack before `write_u16` to 2 blocks.

**write_footer — `write_bytes(w, signature)?;` line:**
1 annotation block. ✅ Fine.

**read_footer — `read_seq_u16(r, raw)` line:**
2 annotation blocks. ✅ Fine.

### 3. Per-block isolation evaluation

**Block: encrypt.md "Signature Length MUST be the length of the output" → `write_u16(w, len)?;`**
Context reset: I see "Signature Length MUST be the length of the output of the calculation above" and the code `write_u16(w, len)`. The annotation is about the VALUE being the signature length. The code writes a u16 value. I need to look UP to see where `len` comes from to verify it's the signature length. This annotation would be better on the `let len` line where the value is computed from `signature.len()`.

**Block: message-footer.md "MUST be 2 bytes" → `write_u16(w, len)?;`**
Context reset: I see "signature length field MUST be 2 bytes" and `write_u16(w, len)`. The "u16" in the function name = 2 bytes. The `reason=` line confirms: "write_u16 writes exactly 2 bytes as a big-endian u16". ✅ Immediately obvious.

**Block: message-footer.md "MUST be interpreted as UInt16" → `write_u16(w, len)?;`**
Context reset: I see "signature length field MUST be interpreted as a UInt16" and `write_u16(w, len)`. The "u16" = UInt16. The `reason=` line confirms. ✅ Immediately obvious.

**Block: message-footer.md "MUST be 2 bytes" → `read_seq_u16(r, raw)` in read_footer**
Context reset: "signature length field MUST be 2 bytes" and `read_seq_u16(r, raw)`. The "u16" hints at 2 bytes. The `reason=` line explains the delegation chain. ✅ Acceptable with reason line.

**Block: message-footer.md "MUST be interpreted as UInt16" → `read_seq_u16(r, raw)` in read_footer**
Context reset: "MUST be interpreted as a UInt16" and `read_seq_u16`. The "u16" = UInt16. The `reason=` line explains. ✅ Acceptable with reason line.

### 4. Semantic relationship check
All annotations semantically relate to their code lines. The only imprecision is the encrypt.md "Signature Length" annotation on `write_u16` instead of `let len` — it's about the value, not the format.

### 5. Sub-items check
Both requirements from the TOML are annotated. No sub-items missed. ✅

### 6. Code structure mirrors spec
The spec describes Signature Length as a 2-byte UInt16 field. The code writes/reads it as u16. Structure matches. ✅

### 7. Top-to-bottom readability
`read_footer`: Clean, 2 annotations then the call. Easy to follow. ✅
`write_footer`: The 3-stack before `write_u16` requires mental effort to separate the "value" annotation from the "format" annotations. Moving the encrypt.md annotation to `let len` would make the flow: compute value → format annotations → write.

### Step 3: Anti-Rationalization Check

I see 3 annotation blocks before `write_u16(w, len)?;` and I'm tempted to say "Agent 2 followed my Round 1 guidance, and 3 is borderline." STOP. The hard rule is 3+ = automatic rejection. No exceptions. The fix is simple and improves semantic precision. I should flag it.

However, I also note: my Round 1 code example literally showed this 3-stack structure. Agent 2 followed my guidance faithfully. The issue is that my Round 1 suggestion was imprecise — I should have placed the encrypt.md annotation on the `let len` line in my example. This is a legitimate Round 2 finding, not Agent 2's fault.

### Cross-reference check
- encrypt.md annotation references `../data-format/message-footer.md#signature-length` → message-footer.md#signature-length annotations exist at same location ✅
- encrypt.md annotation references `../data-format/message-footer.md#signature` → NO corresponding message-footer.md#signature annotation at `write_bytes` line. This is a pre-existing gap, not Agent 2's responsibility.

Links found: 2 relevant cross-references in Agent 2's code area
Cross-refs present: 1/2 (the signature one is pre-existing gap)

### Decision
CHANGES_REQUESTED — 3-stack violation in write_footer. Simple fix: move encrypt.md "Signature Length" annotation to `let len` line.

---

# Round 1 Notes (preserved below)

# Agent 3 Notes — footer (Round 1)

## Step 2: Adversarial Pre-Review

### 1. Does each annotation's next line actually implement THAT requirement?

**Annotation: "This length of the signature length field MUST be 2 bytes."**
- In `write_footer`: next line is `write_seq_u16(w, signature)`. `write_seq_u16` calls `write_u16(w, len)` which writes `len.to_be_bytes()` — a `u16` is always 2 bytes. The connection is indirect but valid: `write_seq_u16` writes a 2-byte length prefix. The annotation is `type=implication` which is appropriate since the 2-byte property is enforced by the type system (`u16`).
- In `read_footer`: next line is `read_seq_u16(r, raw)`. `read_seq_u16` calls `read_u16(r, raw)` which reads exactly 2 bytes. Same reasoning — valid.

**Annotation: "The signature length field MUST be interpreted as a UInt16."**
- In `write_footer`: next line is `write_seq_u16(w, signature)`. `write_seq_u16` converts `data.len()` to `u16` via `u16::try_from`. The value is interpreted/written as a UInt16. Valid.
- In `read_footer`: next line is `read_seq_u16(r, raw)`. `read_seq_u16` calls `read_u16` which returns `u16::from_be_bytes(result)`. The field is interpreted as a UInt16. Valid.

### 2. Annotation stacking check

**`write_footer` — CRITICAL FINDING:**
Before `write_seq_u16(w, signature)` there are **5 annotation blocks**:
1. `encrypt.md#construct-the-signature` — "This operation MUST then serialize..."
2. `encrypt.md#construct-the-signature` — "- [Signature Length]..."
3. `encrypt.md#construct-the-signature` — "- [Signature]..."
4. `message-footer.md#signature-length` — "This length of the signature length field MUST be 2 bytes." (NEW)
5. `message-footer.md#signature-length` — "The signature length field MUST be interpreted as a UInt16." (NEW)

This is 5 annotation blocks before a single line of code. Hard rule: 3+ is automatic CHANGES_REQUESTED.

However, annotations 1-3 are PRE-EXISTING. Agent 2 added only 4 and 5. The pre-existing stack was already 3 (already violating the rule). Agent 2 made it worse by adding 2 more.

The work item guidance explicitly told Agent 2 to place annotations here. But the result violates the hard rule.

**`read_footer`:**
2 annotation blocks before `read_seq_u16(r, raw)`. This is fine (under the 3 limit).

### 3. Per-block isolation evaluation

**Block: "This length of the signature length field MUST be 2 bytes." → `write_seq_u16(w, signature)`**
Context reset: I see an annotation saying the signature length field must be 2 bytes, and the code calls `write_seq_u16`. The "seq_u16" in the function name hints at a u16 (2-byte) length prefix. Reasonably obvious, but a `reason=` line would make it explicit.

**Block: "The signature length field MUST be interpreted as a UInt16." → `write_seq_u16(w, signature)`**
Context reset: I see an annotation saying the field must be interpreted as UInt16, and the code calls `write_seq_u16`. The "u16" in the name maps to UInt16. Reasonably obvious, but again a `reason=` line would help.

**Same blocks in `read_footer` → `read_seq_u16(r, raw)`**
Same analysis. The "u16" in `read_seq_u16` maps to UInt16 and 2 bytes. Reasonably clear.

### 4. Semantic relationship check
All 4 new annotations semantically relate to their code lines. `write_seq_u16` and `read_seq_u16` are the exact points where the 2-byte UInt16 length field is written/read.

### 5. Sub-items check
The spec section has exactly 2 requirements, both annotated. No sub-items missed.

### 6. Code structure mirrors spec
The spec describes Signature Length as a field property. The code fulfills it at the serialization/deserialization points. Structure is appropriate.

### 7. Top-to-bottom readability
In `read_footer`: clean, 2 annotations then the call. Easy to follow.
In `write_footer`: the 5-annotation stack makes it hard to follow. You have to mentally separate the encrypt.md annotations from the message-footer.md annotations.

## Step 3: Anti-Rationalization Check

I noticed the 5-annotation stack in `write_footer` and thought: "But 3 of those are pre-existing, and the work item told Agent 2 to put them here." This is exactly the rationalization pattern I'm warned about. The stack IS a problem. However, the pre-existing 3-stack is not Agent 2's fault, and Agent 2 followed the work item guidance.

I also noticed the missing `reason=` lines and thought: "But the existing implication at the top of the file also lacks a reason line." This is rationalization — the duvet-patterns.md is clear that `implication` annotations should have `reason=` lines, and the codebase DOES use them elsewhere.

**Decision**: The 5-stack in `write_footer` is a real problem that must be flagged. The missing `reason=` lines are also a real problem. CHANGES_REQUESTED.

## Findings Summary

1. **ANNOTATION_PLACEMENT (BLOCKING)**: 5 annotation blocks before `write_seq_u16(w, signature)` in `write_footer`. Hard rule: 3+ is automatic CHANGES_REQUESTED.
2. **ANNOTATION_TYPE (non-blocking but should fix)**: All 4 new `type=implication` annotations lack `//= reason=` lines. The codebase uses them elsewhere for similar structural annotations.
