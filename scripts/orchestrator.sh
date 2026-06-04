#!/bin/bash
set -euo pipefail

REPO_DIR="/tmp/repo-octomus"
BASE_BRANCH="master"
PROMPTS_DIR="$(cd "$(dirname "$0")/.." && pwd)/.hermes-prompts"
LOGS_DIR="$(cd "$(dirname "$0")/.." && pwd)/.hermes-logs"

mkdir -p "$LOGS_DIR"

echo "=== Octomus Orchestrator started at $(date) ==="
cd "$(cd "$(dirname "$0")/.." && pwd)"

launch_subagent() {
  local name="$1"
  local prompt_file="$2"
  local log_file="$LOGS_DIR/$name.log"
  local pid_file="$LOGS_DIR/$name.pid"
  local msg="[$(date '+%H:%M:%S')] Launching $name on Modal (lifetime=7200s, tools=terminal,file)..."
  echo "$msg" | tee "$log_file" > /dev/null
  hermes chat -q "$(cat "$prompt_file")" -t terminal,file --yolo >> "$log_file" 2>&1 &
  local child_pid=$!
  echo $child_pid > "$pid_file"
  echo "$msg (PID=$child_pid)" >> "$LOGS_DIR/orchestrator.log"
}

wait_for_pids() {
  local names=("$@")
  for name in "${names[@]}"; do
    local pid_file="$LOGS_DIR/$name.pid"
    if [ ! -f "$pid_file" ]; then
      echo "[$(date '+%H:%M:%S')] $name: no pid file - skipping" | tee -a "$LOGS_DIR/orchestrator.log"
      continue
    fi
    local pid=$(cat "$pid_file")
    echo "[$(date '+%H:%M:%S')] Waiting for $name (PID=$pid)..." | tee -a "$LOGS_DIR/orchestrator.log"
    wait "$pid" 2>/dev/null
    local ec=$?
    echo "[$(date '+%H:%M:%S')] $name finished (exit=$ec)" | tee -a "$LOGS_DIR/orchestrator.log"
    rm -f "$pid_file"
  done
}

# ── Batch 1: Launch all 4 agents in parallel ────────────────────────
echo "[$(date '+%H:%M:%S')] === Batch 1: Launching all 4 agents ==="
launch_subagent "01-auth-pricing-billing-server" "$PROMPTS_DIR/01-auth-pricing-billing-server-cloudobject.txt"
launch_subagent "02-drive-workspaces" "$PROMPTS_DIR/02-drive-workspaces.txt"
launch_subagent "03-ai-cloud-branding-llms" "$PROMPTS_DIR/03-ai-cloud-branding-llms.txt"

echo "[$(date '+%H:%M:%S')] All 3 agents launched. Waiting for them to finish..."

# ── Wait for all agents ─────────────────────────────────────────────
wait_for_pids "01-auth-pricing-billing-server" "02-drive-workspaces" "03-ai-cloud-branding-llms"

echo "[$(date '+%H:%M:%S')] === All agents finished. Starting merge... ==="

# ── Clone fresh repo for merge ──────────────────────────────────────
rm -rf "$REPO_DIR"
git clone git@github.com:AdrianTuci1/warp-new.git "$REPO_DIR"
cd "$REPO_DIR"
git checkout "$BASE_BRANCH"
git pull origin "$BASE_BRANCH"

# ── Merge branches in order (later = higher priority on conflicts) ──
BRANCHES=(
  "hermes/auth-pricing-billing-server"
  "hermes/drive-workspaces"
  "hermes/ai-cloud-branding-llms"
)

MERGE_FAILED=false
for branch in "${BRANCHES[@]}"; do
  echo "[$(date '+%H:%M:%S')] --- Merging $branch ---"
  if git fetch origin "$branch" 2>&1; then
    if git merge --no-edit "origin/$branch" 2>&1; then
      echo "[$(date '+%H:%M:%S')] Merged $branch cleanly"
    else
      echo "[$(date '+%H:%M:%S')] CONFLICT in $branch - resolving with --theirs..."
      git diff --name-only --diff-filter=U | while read f; do
        git checkout --theirs "$f" 2>/dev/null || true
      done
      git add .
      git commit -m "merge: resolve conflicts from $branch" 2>/dev/null || true
    fi
  else
    echo "[$(date '+%H:%M:%S')] WARNING: Branch $branch not found on remote - skipping"
  fi
done

# ── Push merged result ──────────────────────────────────────────────
echo "[$(date '+%H:%M:%S')] === Pushing merged result to $BASE_BRANCH ==="
git push origin "$BASE_BRANCH"

echo "[$(date '+%H:%M:%S')] === DONE at $(date) ==="
echo ""
echo "Next step: cd to repo and run the compile-fix pass:"
echo "  cd $(cd "$(dirname "$0")/.." && pwd)"
echo "  cargo check 2>&1 | head -100"
