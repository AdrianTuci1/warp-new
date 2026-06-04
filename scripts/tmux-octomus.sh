#!/bin/bash
set -euo pipefail

SESSION="octomus"
PROMPTS_DIR="$(cd "$(dirname "$0")/.." && pwd)/.hermes-prompts"
LOGS_DIR="$(cd "$(dirname "$0")/.." && pwd)/.hermes-logs"
REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Clean old
rm -rf "$LOGS_DIR" && mkdir -p "$LOGS_DIR"

# Kill existing session if any
tmux kill-session -t "$SESSION" 2>/dev/null || true

# ── Create session ──────────────────────────────────────────────────
tmux new-session -d -s "$SESSION" -x 240 -y 80
tmux rename-window -t "$SESSION:0" "octomus"

# ── 3x2 grid (6 panes) ─────────────────────────────────────────────
# Initial pane = 0
# Row 0: pane 0 (left), pane 1 (right)
tmux split-window -h -t "$SESSION:0.0"
# Row 1: pane 2 (left), pane 3 (right)
tmux split-window -v -t "$SESSION:0.0"
tmux split-window -v -t "$SESSION:0.1"
# Row 2: pane 4 (left), pane 5 (right)
tmux split-window -v -t "$SESSION:0.2"
tmux split-window -v -t "$SESSION:0.3"

# Set pane titles
tmux select-pane -t "$SESSION:0.0" -T "🤖 agent-1: delete-cloud-dirs"
tmux select-pane -t "$SESSION:0.1" -T "🤖 agent-2: fix-lib-rs"
tmux select-pane -t "$SESSION:0.2" -T "🤖 agent-3: fix-ai-module"
tmux select-pane -t "$SESSION:0.3" -T "🤖 agent-4: branding-octomus"
tmux select-pane -t "$SESSION:0.4" -T "🤖 agent-5: cargo+notebooks"
tmux select-pane -t "$SESSION:0.5" -T "🎯 coordinator: merge+fix"

tmux set-window-option -t "$SESSION:0" synchronize-panes off

# ── Helper: launch Hermes in a pane and auto-paste its prompt ───────
# Args: pane_index, label, prompt_file, delay_seconds
launch_hermes_pane() {
  local pane=$1
  local label=$2
  local prompt_file=$3
  local delay=$4
  local log_file="$LOGS_DIR/agent-$pane.log"

  tmux send-keys -t "$SESSION:0.$pane" \
    "cd $REPO_DIR && clear && echo '=== $label ==='" Enter
  tmux send-keys -t "$SESSION:0.$pane" \
    "hermes chat -t terminal,file --yolo 2>&1 | tee '$log_file'" Enter

  # Auto-paste prompt after delay (Hermes needs time to start Modal sandbox)
  (
    sleep $delay
    tmux load-buffer -b "agent$pane" "$prompt_file"
    tmux paste-buffer -t "$SESSION:0.$pane" -b "agent$pane"
    tmux send-keys -t "$SESSION:0.$pane" Enter
    echo "[$(date '+%H:%M:%S')] ✅ Auto-pasted $label" >> "$LOGS_DIR/orchestrator.log"
  ) &
}

# ── Launch all 5 agents with staggered delays ───────────────────────
# (Hermes Modal sandbox takes ~5-8s to start, so we stagger)
launch_hermes_pane 0 "agent-1: delete cloud dirs" "$PROMPTS_DIR/agent-1-delete-dirs.txt" 8
launch_hermes_pane 1 "agent-2: fix lib.rs"      "$PROMPTS_DIR/agent-2-fix-lib.txt" 14
launch_hermes_pane 2 "agent-3: fix ai module"    "$PROMPTS_DIR/agent-3-fix-ai.txt" 20
launch_hermes_pane 3 "agent-4: branding Octomus" "$PROMPTS_DIR/agent-4-branding.txt" 26
launch_hermes_pane 4 "agent-5: cargo+notebooks"  "$PROMPTS_DIR/agent-5-fix-cargo.txt" 32

# ── Pane 5: Coordinator shell ───────────────────────────────────────
tmux send-keys -t "$SESSION:0.5" \
  "cd $REPO_DIR && clear" Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "🎯 OCTOMUS COORDINATOR" && echo ""' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "Panouri active (toate rulează pe Modal, 16vCPU/32GB fiecare):"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  P0: agent-1 — șterge directoarele cloud"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  P1: agent-2 — editează lib.rs"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  P2: agent-3 — editează ai/mod.rs + llms.rs"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  P3: agent-4 — redenumește Warp→Octomus în UI"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  P4: agent-5 — Cargo.toml + notebooks + workflows"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  P5: coordinator (aici)"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo ""' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "Când toți agenții au făcut push:"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  bash scripts/merge-octomus.sh"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo ""' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "Vezi log-uri live:"' Enter
tmux send-keys -t "$SESSION:0.5" \
  'echo "  tail -f .hermes-logs/agent-*.log"' Enter

# ── Layout: tiled 3x2 ───────────────────────────────────────────────
tmux select-layout -t "$SESSION:0" tiled 2>/dev/null || true
tmux select-pane -t "$SESSION:0.0"

echo ""
echo "  🐙 Octomus Swarm — 6 panouri (3x2)"
echo "  ════════════════════════════════════"
echo ""
echo "  📡 Remote: 5 Hermes sessions × 16vCPU/32GB RAM pe Modal"
echo "  🔑 Token:  GITHUB_TOKEN auto-pasat via env_passthrough"
echo "  ⏱️  Lifetime: 4 ore (14400s)"
echo ""
echo "  ┌──────────────────────┬──────────────────────┐"
echo "  │ 🤖 agent-1          │ 🤖 agent-2           │"
echo "  │  delete-cloud-dirs   │  fix-lib-rs          │"
echo "  ├──────────────────────┼──────────────────────┤"
echo "  │ 🤖 agent-3          │ 🤖 agent-4           │"
echo "  │  fix-ai-module       │  branding-octomus    │"
echo "  ├──────────────────────┼──────────────────────┤"
echo "  │ 🤖 agent-5          │ 🎯 coordinator       │"
echo "  │  cargo+notebooks     │  merge+fix           │"
echo "  └──────────────────────┴──────────────────────┘"
echo ""
echo "  Attach:  tmux attach -t $SESSION"
echo "  Navig:   Ctrl+b arrows    Zoom: Ctrl+b z    Detach: Ctrl+b d"
echo ""
echo "  Prompt-urile se auto-pastează eșalonat: 8s, 14s, 20s, 26s, 32s"
echo "  După auto-paste, poți da steer oricând scriind în pane-ul dorit."
