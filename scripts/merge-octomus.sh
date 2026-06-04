#!/bin/bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_DIR"

MERGE_BRANCH="octomus/merged"

echo "🎯 Octomus Merge Coordinator"
echo "════════════════════════════"
echo ""

# Fetch latest
git fetch origin

BRANCHES=(
  "origin/octomus/agent-1-delete"
  "origin/octomus/agent-2-lib"
  "origin/octomus/agent-3-ai"
  "origin/octomus/agent-4-branding"
  "origin/octomus/agent-5-cargo"
)

# Check which branches exist
AVAILABLE=()
for branch in "${BRANCHES[@]}"; do
  if git show-ref --verify "refs/remotes/$branch" &>/dev/null; then
    AVAILABLE+=("$branch")
    echo "  ✅ $branch"
  else
    echo "  ❌ $branch (not pushed yet — skipping)"
  fi
done

echo ""

if [ ${#AVAILABLE[@]} -eq 0 ]; then
  echo "❌ Niciun branch disponibil. Așteaptă să termine agenții."
  exit 1
fi

# Create merge branch from master
git checkout master
git pull origin master
git checkout -b "$MERGE_BRANCH"

echo ""
echo "--- Merging ${#AVAILABLE[@]} branches in order ---"
echo ""

for branch in "${AVAILABLE[@]}"; do
  echo "→ Merging $branch..."
  if git merge --no-edit "$branch" 2>&1; then
    echo "  ✅ Merged cleanly"
  else
    echo "  ⚠️  CONFLICT — resolving with --theirs (agent branch wins)"
    git diff --name-only --diff-filter=U | while read f; do
      git checkout --theirs "$f" 2>/dev/null || true
    done
    git add .
    git commit -m "merge: resolve conflicts from $branch" 2>/dev/null || echo "  (nothing to commit)"
    echo "  ✅ Resolved"
  fi
done

echo ""
echo "=== Checking compilation... ==="
cargo check 2>&1 | head -60
EC=$?

echo ""
if [ $EC -eq 0 ]; then
  echo "✅ Compilation OK!"
  echo ""
  echo "Pentru a împinge rezultatul:"
  echo "  git push origin $MERGE_BRANCH"
else
  echo "⚠️  Erori de compilare. Fix manual, apoi:"
  echo "  git push origin $MERGE_BRANCH"
fi

echo ""
echo "=== Done ==="
