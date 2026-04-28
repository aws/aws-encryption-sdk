#!/bin/bash
# polish-sequential.sh — polish one file at a time, sequentially
# Designed to run overnight. If one file fails, logs the error and moves on.
#
# Usage:
#   ./scripts/polish-sequential.sh                          # all .rs files in src/
#   ./scripts/polish-sequential.sh src/encrypt.rs src/decrypt.rs  # specific files
#   ./scripts/polish-sequential.sh kill                     # kill running agent
#   nohup ./scripts/polish-sequential.sh > polish.log 2>&1 &  # run overnight

set -uo pipefail

MODEL="claude-opus-4.7"

if [ "${1:-}" = "kill" ]; then
    pkill -9 -f "kiro-cli" 2>/dev/null || true
    echo "Killed. Check: ps aux | grep kiro"
    exit 0
fi

# Determine files
if [ $# -eq 0 ]; then
    FILES=$(find src/ -name '*.rs' -type f | sort)
elif [ -d "$1" ]; then
    FILES=$(find "$1" -name '*.rs' -type f | sort)
else
    FILES="$@"
fi

FILE_COUNT=$(echo "$FILES" | wc -w | tr -d ' ')
SUCCEEDED=0
FAILED=0
SKIPPED=0

echo "=== Sequential Polish: $FILE_COUNT files ==="
echo "Started: $(date)"
echo ""

mkdir -p .agent-reports

# Track test files already covered via source file matching
COVERED_TESTS=""

for file in $FILES; do
    slug="polish-$(basename "$file" .rs)"
    
    # Find corresponding test files
    basename_no_ext=$(basename "$file" .rs)
    test_files=$(find tests/ -name "*${basename_no_ext}*" -type f 2>/dev/null | tr '\n' ' ')
    COVERED_TESTS="$COVERED_TESTS $test_files"
    
    echo "--- [$((SUCCEEDED + FAILED + SKIPPED + 1))/$FILE_COUNT] $file (slug: $slug) ---"
    echo "  Tests: ${test_files:-none found}"
    echo "Started: $(date)"
    
    # Skip if already done
    if [ -f ".agent-reports/$slug/status.txt" ] && grep -q "DONE" ".agent-reports/$slug/status.txt"; then
        echo "  Already done, skipping."
        ((SKIPPED++))
        continue
    fi
    
    if kiro-cli chat --no-interactive --trust-all-tools --agent duvet-pipeline --model "$MODEL" "
You are running in polish mode for a single file. This is a non-interactive session — do NOT ask questions or wait for input. Complete all work autonomously.

File to polish: $file
Test files: ${test_files:-none found}
Slug: $slug
Report directory: .agent-reports/$slug/

Execute the following review SOPs yourself directly. For each SOP:
1. Read the entire SOP
2. Write the EXACT checklist from the SOP to .agent-reports/$slug/
3. Execute every step, checking each box after completion
4. Write the report

Reviews to perform (in order):
1. duvet-review-annotations.sop.md — check quotes, paths, placement quality
   Checklist: .agent-reports/$slug/checklist-annotations.md
   Report: .agent-reports/$slug/review-annotations.md

2. duvet-review-tests.sop.md — check test coverage, quality, location
   Checklist: .agent-reports/$slug/checklist-tests.md
   Report: .agent-reports/$slug/review-tests.md

3. duvet-review-code.sop.md — check spec compliance, error patterns, style
   Checklist: .agent-reports/$slug/checklist-code.md
   Report: .agent-reports/$slug/review-code.md

After all 3 reviews:
- If ANY review found issues, write a combined work item to .agent-reports/$slug/work-item.md
  and follow duvet-implement.sop.md to fix them. Then re-review. Loop max 3 rounds.
- Do NOT run duvet discovery. Do NOT commit. Just fix the files.
- When done, write DONE to .agent-reports/$slug/status.txt
"; then
        ((SUCCEEDED++))
        echo "  ✓ Done"
    else
        ((FAILED++))
        echo "  ✗ Failed (exit code: $?)"
        echo "FAILED" > ".agent-reports/$slug/status.txt"
    fi
    
    echo "  Finished: $(date)"
    echo ""
done

echo "=== Results (Pass 1: Source Files) ==="
echo "Succeeded: $SUCCEEDED"
echo "Failed: $FAILED"
echo "Skipped (already done): $SKIPPED"
echo "Total: $FILE_COUNT"
echo ""

# Pass 2: Find test files not already covered
echo "=== Pass 2: Uncovered Test Files ==="
ALL_TESTS=$(find tests/ -name '*.rs' -type f 2>/dev/null | sort)
UNCOVERED_TESTS=""
for test_file in $ALL_TESTS; do
    if ! echo "$COVERED_TESTS" | grep -q "$test_file"; then
        UNCOVERED_TESTS="$UNCOVERED_TESTS $test_file"
    fi
done

UNCOVERED_COUNT=$(echo "$UNCOVERED_TESTS" | wc -w | tr -d ' ')
if [ "$UNCOVERED_COUNT" -eq 0 ]; then
    echo "All test files were covered in pass 1."
else
    echo "$UNCOVERED_COUNT test files not covered by source matching."
    
    for test_file in $UNCOVERED_TESTS; do
        slug="polish-test-$(basename "$test_file" .rs)"
        echo "--- [test] $test_file (slug: $slug) ---"
        echo "  Started: $(date)"
        
        if [ -f ".agent-reports/$slug/status.txt" ] && grep -q "DONE" ".agent-reports/$slug/status.txt"; then
            echo "  Already done, skipping."
            ((SKIPPED++))
            continue
        fi
        
        if kiro-cli chat --no-interactive --trust-all-tools --agent duvet-pipeline --model "$MODEL" "
You are running in polish mode for a test file. This is a non-interactive session — do NOT ask questions or wait for input. Complete all work autonomously.

Test file to polish: $test_file
Slug: $slug
Report directory: .agent-reports/$slug/

This is a TEST FILE that was not matched to a source file. Focus on:
1. Follow duvet-review-tests.sop.md — check test coverage, quality, location, assertions
   Checklist: .agent-reports/$slug/checklist-tests.md
   Report: .agent-reports/$slug/review-tests.md

2. Follow duvet-review-code.sop.md — check error patterns, style, clippy
   Checklist: .agent-reports/$slug/checklist-code.md
   Report: .agent-reports/$slug/review-code.md

If ANY review found issues, follow duvet-implement.sop.md to fix them. Loop max 3 rounds.
Do NOT commit. Just fix the files.
When done, write DONE to .agent-reports/$slug/status.txt
"; then
            ((SUCCEEDED++))
            echo "  ✓ Done"
        else
            ((FAILED++))
            echo "  ✗ Failed"
            echo "FAILED" > ".agent-reports/$slug/status.txt"
        fi
        echo "  Finished: $(date)"
        echo ""
    done
fi

echo ""
echo "=== Final Results ==="
echo "Succeeded: $SUCCEEDED"
echo "Failed: $FAILED"
echo "Skipped: $SKIPPED"
echo "Finished: $(date)"

if [ "$FAILED" -gt 0 ]; then
    echo ""
    echo "Failed files:"
    for file in $FILES; do
        slug="polish-$(basename "$file" .rs)"
        if [ -f ".agent-reports/$slug/status.txt" ] && grep -q "FAILED" ".agent-reports/$slug/status.txt"; then
            echo "  - $file"
        fi
    done
fi
