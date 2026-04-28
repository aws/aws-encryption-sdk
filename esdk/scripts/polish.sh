#!/bin/bash
# polish.sh — spawn one kiro-cli agent per source file to review and fix code
#
# Usage:
#   ./scripts/polish.sh                          # polish all .rs files in src/
#   ./scripts/polish.sh src/encrypt.rs src/decrypt.rs  # polish specific files
#   ./scripts/polish.sh src/format/              # polish all .rs files in a directory
#   ./scripts/polish.sh kill                     # kill all running polish agents

set -euo pipefail

MODEL="claude-opus-4.7"

# Kill mode
if [ "${1:-}" = "kill" ]; then
    echo "Killing all kiro-cli polish agents..."
    pkill -9 -f "kiro-cli" 2>/dev/null || true
    pkill -9 -f "kiro" 2>/dev/null || true
    # Kill any orphaned child processes from the script's process group
    pkill -9 -P $$ 2>/dev/null || true
    echo "Done. Verify with: ps aux | grep kiro"
    exit 0
fi

# Determine files to polish
if [ $# -eq 0 ]; then
    FILES=$(find src/ -name '*.rs' -type f | sort)
elif [ -d "$1" ]; then
    FILES=$(find "$1" -name '*.rs' -type f | sort)
else
    FILES="$@"
fi

FILE_COUNT=$(echo "$FILES" | wc -w | tr -d ' ')
echo "=== Polish: $FILE_COUNT files ==="

PIDS=()
SLUGS=()

mkdir -p .agent-reports

for file in $FILES; do
    slug="polish-$(basename "$file" .rs)"
    SLUGS+=("$slug")
    
    echo "  Spawning: $file (slug: $slug)"
    
    kiro-cli chat --no-interactive --trust-all-tools --agent duvet-pipeline --model "$MODEL" "
You are running in polish mode for a single file. This is a non-interactive session — do NOT ask questions or wait for input. Complete all work autonomously.

File to polish: $file
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
" &
    
    PIDS+=($!)
done

echo ""
echo "=== Waiting for $FILE_COUNT agents... ==="

FAILED=0
SUCCEEDED=0
for i in "${!PIDS[@]}"; do
    pid=${PIDS[$i]}
    slug=${SLUGS[$i]}
    if wait "$pid"; then
        ((SUCCEEDED++))
        echo "  ✓ $slug"
    else
        ((FAILED++))
        echo "  ✗ $slug (exit code: $?)"
    fi
done

echo ""
echo "=== Results: $SUCCEEDED succeeded, $FAILED failed ==="

if [ "$FAILED" -gt 0 ]; then
    echo "Some agents failed. Check .agent-reports/polish-*/status.txt for details."
    echo "Skipping commit."
    exit 1
fi

# Verify
echo ""
echo "=== Running cargo check... ==="
if ! cargo check --workspace; then
    echo "FAILED: cargo check"
    exit 1
fi

echo "=== Running cargo test... ==="
if ! cargo test --workspace; then
    echo "FAILED: cargo test"
    exit 1
fi

# Commit
echo ""
echo "=== Committing... ==="
git add -A
git commit -m "style: polish codebase against updated standards

Polished $FILE_COUNT files using duvet-review-annotations, duvet-review-tests,
and duvet-review-code SOPs."

echo "=== Done. ==="
