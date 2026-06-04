#!/bin/bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_DIR"

echo "=== Octomus merge script ==="
echo ""

# Fetch latest
git fetch origin

BRANCH="origin/octomus/refactor-all"

echo "Looking for branch: $BRANCH"
echo ""

if git show-ref --verify "refs/remotes/$BRANCH" &>/dev/null; then
  echo "--- Merging $BRANCH into master ---"
  if git merge --no-edit "$BRANCH" 2>&1; then
    echo "  ✅ Merged cleanly"
  else
    echo "  ⚠️  CONFLICT! Check files above, then:"
    echo "     git mergetool  (sau rezolvă manual)"
    echo "     git add . && git commit"
    exit 1
  fi

  echo ""
  echo "=== Checking compilation... ==="
  cargo check 2>&1 | head -80
else
  echo "  ❌ Branch $BRANCH not found on remote."
  echo "     Agentul Hermes nu a terminat sau n-a făcut push."
  echo "     Verifică: tail -f .hermes-logs/hermes.log"
fi

echo ""
echo "=== Done ==="
