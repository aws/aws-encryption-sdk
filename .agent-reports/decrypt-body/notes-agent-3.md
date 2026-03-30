# Agent 3 Notes — decrypt-body — Cycle 1, Round 1

## Adversarial Pre-Review

### 1. Per-Annotation Challenge: Does the next line actually implement THAT requirement?

**Req 20 (streaming SHOULD input to signature algorithm) at function header (line 88-93):**
The annotation is placed at the function signature area, before `let mut expected_frame`. The `raw` parameter IS a DigestWriter, and the function does feed bytes to it via `read_bytes(r, ..., raw)` and `read_u32(r, raw)`. But the annotation is NOT at the point where bytes are fed to the signature algorithm — it's at the function entry. The fulfillment happens at each `read_bytes`/`read_u32` call. This is a Pattern 3 (general behavior at method start) usage, which is acceptable for a structural property. The `type=implication` with reason is appropriate. **Marginal — acceptable as Pattern 3.**

**Req 1 (wait for bytes) at `loop {` (line 113-119):**
The annotation is before the `loop {`. The loop + blocking reads implicitly wait. `type=implication` with reason is appropriate. The fulfillment is structural (the loop itself). **Acceptable as Pattern 3.**

**Req 2 (Seq Num End) and Req 3 (Seq Num) at `read_u32` (lines 126-133):**
Both annotations are on the same `read_u32` call. The same 4 bytes serve as both the Sequence Number End check AND the Sequence Number for regular frames. This is a genuine dual-purpose read. The annotations are semantically correct — `read_u32` does deserialize both. **Acceptable — but contributes to stacking problem.**

**Req 14 (cipherkey = derived data key) in decrypt.rs (line 444-447):**
Placed at `let key = state.derived_data_keys.data_key.clone();`. This IS the line where the cipherkey is set to the derived data key. Semantically correct. **But creates 3-stack with pre-existing annotations.**

**Req 19 (plaintext SHOULD be released without signature) at `write_bytes` area (line 293-298):**
The annotation is before the `if expected_frame != START_SEQUENCE_NUMBER` block which calls `write_bytes(w, &result)?`. The `write_bytes` IS the release point. But the annotation is separated from `write_bytes` by the `if` condition and `fail_if_multi_frame` check. The annotation is NOT immediately before the fulfilling code — there's a conditional block between them. **Finding: annotation should be inside the conditional, closer to `write_bytes`.**

**Req 12 (un-framed seq num = 1) at `NONFRAMED_SEQUENCE_NUMBER` (line 520-522):**
The annotation is at `header::NONFRAMED_SEQUENCE_NUMBER` in the non-framed body_aad call. This is correct — the value IS 1. But there are now 3 annotation blocks before this parameter:
1. `message-body-aad.md#body-aad-content` (pre-existing)
2. `message-body-aad.md#sequence-number` (pre-existing)
3. `decrypt.md#decrypt-the-message-body` (new from Agent 2)
Wait — let me re-check. The `body-aad-content` annotation is on `BodyAADContent::SingleBlock` (previous parameter), not on `NONFRAMED_SEQUENCE_NUMBER`. So the stacking on `NONFRAMED_SEQUENCE_NUMBER` is:
1. `message-body-aad.md#sequence-number` (pre-existing)
2. `decrypt.md#decrypt-the-message-body` (new)
That's 2 blocks — acceptable.

### 2. Annotation Stacking Analysis (CRITICAL)

**VIOLATION 1: Function header — 4 blocks before `let mut expected_frame`**
- Block 1: Regular frame deserialization (pre-existing)
- Block 2: Final frame deserialization (pre-existing)
- Block 3: Streaming SHOULD input to signature (NEW — Agent 2)
- Block 4: First frame seq num = 1 (pre-existing)
Agent 2 added Block 3, making a pre-existing 3-stack into a 4-stack. **HARD VIOLATION.**

**VIOLATION 2: Before `let seq_num = read_u32(r, raw)?;` — 6 blocks**
- Block 1: Seq Num End deserialization (NEW — Agent 2)
- Block 2: Seq Num deserialization (NEW — Agent 2)
- Block 3-6: data-format implications (pre-existing)
Agent 2 added 2 blocks to a pre-existing 4-stack, making it 6. **HARD VIOLATION.**

**VIOLATION 3: Before `read_seq_u32_bounded(...)` — 6 blocks**
- Block 1: Content length ≤ frame length (pre-existing)
- Block 2: Encrypted Content Length deserialization (NEW — Agent 2)
- Block 3-6: data-format implications (pre-existing)
Agent 2 added 1 block to a pre-existing 5-stack, making it 6. **HARD VIOLATION.**

**VIOLATION 4: Before `aes_decrypt(` in regular frame — 3 blocks**
- All 3 pre-existing. Agent 2 did NOT add to this stack.
- Agent 2 correctly put per-parameter annotations inside the multi-line call.
**Pre-existing violation — not Agent 2's fault, but should be noted.**

**VIOLATION 5: decrypt.rs before `let key = ...` — 3 blocks**
- Block 1: Body deserialized after header (pre-existing)
- Block 2: Content type determines framed/non-framed (pre-existing)
- Block 3: Cipherkey = derived data key (NEW — Agent 2)
Agent 2 added 1 block to a pre-existing 2-stack, making it 3. **HARD VIOLATION.**

### 3. Per-Block Isolation Evaluation

**Block at line 88-93 (streaming signature input):**
Quote: "The streamed Decrypt operation SHOULD input the serialized frame to the signature algorithm..."
Code: `let mut expected_frame: u32 = START_SEQUENCE_NUMBER;`
Is it obvious why? NO. The code line is about sequence numbers, not signature algorithms. The annotation relates to the `raw` parameter (DigestWriter) which is used throughout the function. This is a Pattern 3 annotation at function start, which is acceptable for structural properties, but it's buried in a stack of 4 annotations. **Finding: move to its own location or accept as Pattern 3 with caveat.**

**Block at lines 126-129 (Seq Num End) and 130-133 (Seq Num):**
Both on `read_u32`. In isolation: "Sequence Number End MUST be deserialized" → `read_u32` reads 4 bytes → yes, obvious. "Sequence Number MUST be deserialized" → same `read_u32` → yes, obvious. But the 6-stack makes it impossible to evaluate in isolation. **Finding: stacking.**

### 4. Semantic Relationship Check

All new annotations have clear semantic relationships to their code lines. The per-parameter annotations on `body_aad()` and `aes_decrypt()` are well-placed — each annotation is on the specific argument it describes.

### 5. Spec Sub-Items

The spec lists deserialization sub-items (Seq Num End, Seq Num, IV, Content Length, Content, Auth Tag) and AAD sub-items (message ID, body AAD content, sequence number, content length). Agent 2 annotated each at the specific code element. **Good — Pattern 4 applied correctly.**

### 6. Code Structure vs Spec Structure

The spec describes: deserialize → construct AAD → set decryption inputs → decrypt. The code follows this flow. **Good.**

### 7. Top-to-Bottom Readability

The file reads well top-to-bottom EXCEPT for the stacking violations, which require scrolling through walls of annotations to find the code.

## Anti-Rationalization Check

I noticed the stacking violations and my instinct was to say "but the pre-existing code already had stacks, Agent 2 just added to them." STOP. Agent 2 was told to reformat calls to multi-line for per-argument annotations. They did that for `body_aad()` and `aes_decrypt()` (good). But they did NOT address the pre-existing stacks at the function header and `read_u32`. They made them WORSE by adding more annotations to existing stacks. The work item guidance said to annotate at specific calls — it did NOT say to stack them on top of existing stacks.

However: the stacking at `read_u32` is genuinely difficult because the same call serves dual purposes (Seq Num End + Seq Num). And the pre-existing data-format implications were already stacked there. Agent 2 can't easily restructure pre-existing code.

The decrypt.rs 3-stack is Agent 2's fault — they could have placed the cipherkey annotation on the `&key` argument inside the `body::read_and_decrypt_framed_message_body(...)` call or inside the function itself (which they did in body.rs). Having it in BOTH places is redundant.

## Key Findings

1. **STACKING (HARD VIOLATIONS)**: Multiple locations with 3+ annotation blocks before a single code line. Agent 2 worsened pre-existing stacks.
2. **Redundant cipherkey annotation**: Req 14 is annotated in BOTH decrypt.rs AND body.rs. The body.rs placement (on the `key` parameter of `aes_decrypt`) is the point of fulfillment. The decrypt.rs placement creates a 3-stack.
3. **Explicit `type=implementation`**: Lines 83, 86 have explicit `type=implementation` — but these are PRE-EXISTING, not Agent 2's additions.
4. **Req 19 placement**: The "plaintext SHOULD be released" annotation is separated from `write_bytes` by conditional logic.

## Potential Spec Gaps

None identified. The implementation matches the spec well.
