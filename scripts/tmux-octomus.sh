#!/bin/bash
set -euo pipefail

SESSION="octomus"
PROMPTS_DIR="$(cd "$(dirname "$0")/.." && pwd)/.hermes-prompts"
LOGS_DIR="$(cd "$(dirname "$0")/.." && pwd)/.hermes-logs"
REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PROMT_FILE="$PROMPTS_DIR/00-octomus-combined.txt"

# Clean old
rm -rf "$LOGS_DIR" && mkdir -p "$LOGS_DIR"

# Kill existing session if any
tmux kill-session -t "$SESSION" 2>/dev/null || true

# ── Create session ──────────────────────────────────────────────────
tmux new-session -d -s "$SESSION" -x 240 -y 80
tmux rename-window -t "$SESSION:0" "octomus"

# ── 2x2 grid ────────────────────────────────────────────────────────
# Initial pane = 0
tmux split-window -h -t "$SESSION:0.0"
tmux split-window -v -t "$SESSION:0.0"
tmux split-window -v -t "$SESSION:0.1"

tmux select-pane -t "$SESSION:0.0" -T "🐙 HERMES — 8vCPU/16GB Modal"
tmux select-pane -t "$SESSION:0.1" -T "📋 LOG — tail -f"
tmux select-pane -t "$SESSION:0.2" -T "🔧 SHELL — git/cargo/merge"
tmux select-pane -t "$SESSION:0.3" -T "📊 STATUS — cargo check"

tmux set-window-option -t "$SESSION:0" synchronize-panes off

# ── Pane 0: Interactive Hermes → 1 VM (8vCPU/16GB/100GB) on Modal ──
tmux send-keys -t "$SESSION:0.0" \
  "cd $REPO_DIR && clear && echo '🐙 OCTOMUS REFACTOR — 1 VM (8vCPU / 16GB / 100GB disk)'" Enter
tmux send-keys -t "$SESSION:0.0" \
  "hermes chat -t terminal,file --yolo | tee '$LOGS_DIR/hermes.log'" Enter

# ── Auto-paste prompt after 10 seconds (wait for Hermes to boot + Modal sandbox) ──
(
  sleep 10
  # Load prompt into tmux buffer and paste to pane 0
  tmux load-buffer -b octomus "$PROMT_FILE"
  tmux paste-buffer -t "$SESSION:0.0" -b octomus
  tmux send-keys -t "$SESSION:0.0" Enter
  echo "[$(date '+%H:%M:%S')] ✅ Prompt auto-pasted to pane 0" >> "$LOGS_DIR/orchestrator.log"
) &
AUTOPASTE_PID=$!
echo $AUTOPASTE_PID > "$LOGS_DIR/autopaste.pid"

# ── Pane 1: tail -f log ─────────────────────────────────────────────
tmux send-keys -t "$SESSION:0.1" \
  "cd $REPO_DIR && clear" Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "=== Watching Hermes log (auto-refresh) ===" && echo"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'tail -f .hermes-logs/hermes.log 2>/dev/null || echo "Waiting for log..." && sleep 2 && exec tail -f .hermes-logs/hermes.log' Enter

# ── Pane 2: Shell (git / cargo / merge) ─────────────────────────────
tmux send-keys -t "$SESSION:0.2" \
  "cd $REPO_DIR && clear" Enter
tmux send-keys -t "$SESSION:0.2" \
  'echo "=== Octomus Shell ===" && echo "After agent finishes:" && echo "  scripts/merge-octomus.sh    — merge branch locally" && echo "  cargo check                  — check compilation"' Enter

# ── Pane 3: cargo check monitor ─────────────────────────────────────
tmux send-keys -t "$SESSION:0.3" \
  "cd $REPO_DIR && clear" Enter
tmux send-keys -t "$SESSION:0.3" \
  'echo "Ready. Run cargo check after merge."' Enter

# ── Layout ──────────────────────────────────────────────────────────
tmux select-layout -t "$SESSION:0" tiled 2>/dev/null || true
tmux select-pane -t "$SESSION:0.0"

echo ""
echo "  🐙 Octomus Refactor — Swarm Launcher"
echo "  ═══════════════════════════════════"
echo ""
echo "  📡 Remote: 1 Hermes session pe Modal (8vCPU / 16GB RAM / 100GB disk)"
echo "  📋 Prompt:  .hermes-prompts/00-octomus-combined.txt"
echo "  🔑 Token:   GITHUB_TOKEN auto-pasat via env_passthrough"
echo ""
echo "  ┌─────────────────────────────┬─────────────────────────────┐"
echo "  │ 🐙 HERMES (interactive)    │ 📋 LOG (tail -f)            │"
echo "  │   1 VM care face TOT       │   vezi progresul in timp     │"
echo "  │   prompt auto-pasted       │   real                       │"
echo "  ├─────────────────────────────┼─────────────────────────────┤"
echo "  │ 🔧 SHELL (git/merge)       │ 📊 STATUS (cargo check)     │"
echo "  │   comenzi dupa ce agentul  │   verifici compilarea        │"
echo "  │   termina                   │                             │"
echo "  └─────────────────────────────┴─────────────────────────────┘"
echo ""
echo "  Attach:  tmux attach -t $SESSION"
echo "  Navig:   Ctrl+b arrows    Zoom: Ctrl+b z    Detach: Ctrl+b d"
echo ""
echo "  ⏱️  Prompt-ul se auto-pastează după 10 secunde (cât pornește"
echo "     Hermes + sandbox-ul Modal). Poți da steer oricând."
