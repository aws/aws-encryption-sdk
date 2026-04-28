#!/bin/bash
# polish-v2.sh — maximum determinism polish
#
# Architecture:
#   Script parses review SOPs to extract checklist items
#   Spawns one reviewer agent per checklist item (all parallel)
#   Merges findings → auditor filters → implementer fixes → loop
#
# Usage:
#   ./scripts/polish-v2.sh src/encrypt.rs
#   ./scripts/polish-v2.sh src/encrypt.rs src/decrypt.rs
#   ./scripts/polish-v2.sh kill

set -uo pipefail

MODEL="claude-opus-4.7"
MAX_ROUNDS=3
REVIEWER_AGENT="reviewer"
IMPLEMENTER_AGENT="implementer"

# Path to SOPs — adjust if running from a different directory
SOP_DIR="$(dirname "$0")/../agent-sops"
if [ ! -d "$SOP_DIR" ]; then
    # Try installed AIM package location
    SOP_DIR="$HOME/.aim/packages/local/CryptoToolsAISkills-1.0/agent-sops"
fi

if [ "${1:-}" = "kill" ]; then
    pkill -9 -f "kiro-cli" 2>/dev/null || true
    echo "Killed."
    exit 0
fi

if [ $# -eq 0 ]; then
    echo "Usage: $0 <file1.rs> [file2.rs ...]"
    exit 1
fi

FILES="$@"

# Extract actionable checklist items from an SOP file
# Skips "Read" and "Write" steps, keeps Check/Placement/Count/Run steps
extract_checks() {
    local sop_file="$1"
    local category="$2"
    grep '^- \[ \]' "$sop_file" \
        | grep -iv 'read \|write \|create \|anti-rational' \
        | sed 's/^- \[ \] [0-9]*\. //' \
        | while read -r check; do
            echo "${category}|${check}"
        done
}

# Extract the full step content from an SOP for a given step number
get_step_content() {
    local sop_file="$1"
    local step_num="$2"
    # Get content between ### N. and the next ### or ##
    awk "/^### ${step_num}\./,/^###? /" "$sop_file" | head -20
}

for file in $FILES; do
    slug="polish-$(basename "$file" .rs)"
    echo "=== Polishing: $file (slug: $slug) ==="
    mkdir -p ".agent-reports/$slug"
    
    # Find test files
    basename_no_ext=$(basename "$file" .rs)
    test_files=$(find tests/ -name "*${basename_no_ext}*" -type f 2>/dev/null | tr '\n' ' ')
    
    # Parse all checklist items from the 3 review SOPs
    CHECKS=()
    while IFS= read -r line; do
        CHECKS+=("$line")
    done < <(
        extract_checks "$SOP_DIR/duvet-review-code.sop.md" "code"
        extract_checks "$SOP_DIR/duvet-review-annotations.sop.md" "annotations"
        extract_checks "$SOP_DIR/duvet-review-tests.sop.md" "tests"
    )
    
    echo "  Parsed ${#CHECKS[@]} checklist items from SOPs"
    
    for round in $(seq 1 $MAX_ROUNDS); do
        echo ""
        echo "--- Round $round ---"
        
        # Clear previous findings
        rm -f .agent-reports/$slug/check-*.md
        
        PIDS=()
        IDX=0
        
        # Step 1: Spawn one reviewer per checklist item (all parallel)
        for check_line in "${CHECKS[@]}"; do
            IFS='|' read -r category check_desc <<< "$check_line"
            ((IDX++))
            output_file=".agent-reports/$slug/check-${category}-${IDX}.md"
            
            echo "  [$IDX] Spawning: $category — $check_desc"
            
            kiro-cli chat --no-interactive --trust-all-tools --agent "$REVIEWER_AGENT" --model "$MODEL" "
You are checking ONE specific thing on a file. This is non-interactive — complete autonomously.

File to review: $file
Test files: ${test_files:-none}
Category: $category
Check: $check_desc

Read the file. Perform ONLY this one check. Be thorough but focused.

Write your findings to $output_file in this exact format:
## ${category}: ${check_desc}
**Result:** PASS or FAIL
**Findings:**
- (list each finding with line number and description)
- If no issues: 'No issues found.'
" &
            PIDS+=($!)
        done
        
        echo "  Waiting for ${#PIDS[@]} reviewer agents..."
        FAILED_SPAWNS=0
        for pid in "${PIDS[@]}"; do
            wait "$pid" || ((FAILED_SPAWNS++))
        done
        echo "  All reviewers done ($FAILED_SPAWNS failed to spawn)."
        
        # Step 2: Merge all findings
        echo "  Merging findings..."
        MERGED=".agent-reports/$slug/merged-findings-round-$round.md"
        echo "# Merged Review Findings — $file — Round $round" > "$MERGED"
        echo "" >> "$MERGED"
        
        FAIL_COUNT=0
        for report in .agent-reports/$slug/check-*.md; do
            [ -f "$report" ] || continue
            if grep -q "FAIL" "$report" 2>/dev/null; then
                ((FAIL_COUNT++))
                cat "$report" >> "$MERGED"
                echo "" >> "$MERGED"
            fi
        done
        
        echo "  Raw findings: $FAIL_COUNT checks reported FAIL"
        
        if [ "$FAIL_COUNT" -eq 0 ]; then
            echo "  ✓ All checks passed. File is clean."
            break
        fi
        
        # Step 3: Auditor filters noise
        echo "  Spawning auditor to filter findings..."
        kiro-cli chat --no-interactive --trust-all-tools --agent "$REVIEWER_AGENT" --model "$MODEL" "
You are a senior reviewer auditing findings from ${FAIL_COUNT} sub-reviewers. This is non-interactive.

Read $MERGED and also read $file to understand the code context.

The sub-reviewers each checked one specific thing. They tend to over-report — they feel obligated to find something. Your job is to filter:
- Which findings are REAL issues that would cause bugs, spec non-compliance, or maintenance problems?
- Which are noise, nitpicks, or technically correct but not worth changing?

Write the filtered work item to .agent-reports/$slug/work-item-round-$round.md:
- If nothing is worth fixing, write just the word PASS on the first line.
- Otherwise, list only the genuine findings with file, line, and what to fix.
"
        
        # Step 4: Check if auditor said PASS
        WORK_ITEM=".agent-reports/$slug/work-item-round-$round.md"
        if [ -f "$WORK_ITEM" ] && head -1 "$WORK_ITEM" | grep -q "PASS"; then
            echo "  ✓ Auditor says all findings are noise. File is clean."
            break
        fi
        
        # Step 5: Implementer fixes
        echo "  Spawning implementer to fix genuine issues..."
        kiro-cli chat --no-interactive --trust-all-tools --agent "$IMPLEMENTER_AGENT" --model "$MODEL" "
Fix code issues in $file. This is non-interactive — complete autonomously.

Read .agent-reports/$slug/work-item-round-$round.md for the findings to fix.
These have been filtered by a senior reviewer — they are all genuine issues.

Fix every finding. Do not change anything that wasn't flagged. Do NOT commit.
"
        
        # Step 6: Verify fixes
        echo "  Spawning verifier..."
        kiro-cli chat --no-interactive --trust-all-tools --agent "$REVIEWER_AGENT" --model "$MODEL" "
Verify that the implementer fixed all findings. This is non-interactive.

Read .agent-reports/$slug/work-item-round-$round.md for what was supposed to be fixed.
Read $file to check the current state.

Did the implementer fix everything? Write verdict to .agent-reports/$slug/verdict-round-$round.md:
- First line: PASS or FAIL
- If FAIL: list what was not fixed.
"
        
        VERDICT=".agent-reports/$slug/verdict-round-$round.md"
        if [ -f "$VERDICT" ] && head -1 "$VERDICT" | grep -q "PASS"; then
            echo "  ✓ Verifier confirms all fixes applied."
        else
            echo "  ✗ Verifier found unfixed issues."
        fi
        
        if [ "$round" -eq "$MAX_ROUNDS" ]; then
            echo "  ⚠ Max rounds reached."
        fi
    done
    echo ""
done

echo "=== Done: $(date) ==="
