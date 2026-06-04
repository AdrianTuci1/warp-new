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

# ── Create session: 2 panes (main + coordinator) ────────────────────
tmux new-session -d -s "$SESSION" -x 240 -y 80
tmux rename-window -t "$SESSION:0" "octomus"

# Split: top 70% = hermes, bottom 30% = coordinator
tmux split-window -v -l 25 -t "$SESSION:0.0"

# Set pane titles
tmux select-pane -t "$SESSION:0.0" -T "🐙 Octomus Master Agent"
tmux select-pane -t "$SESSION:0.1" -T "🎯 Coordinator"

tmux set-window-option -t "$SESSION:0" synchronize-panes off

# ── Pane 0: Main hermes chat ────────────────────────────────────────
MASTER_PROMPT="$PROMPTS_DIR/master-octomus.txt"
LOG_FILE="$LOGS_DIR/octomus.log"

tmux send-keys -t "$SESSION:0.0" \
  "cd $REPO_DIR && clear" Enter
tmux send-keys -t "$SESSION:0.0" \
  "echo '🐙 OCTOMUS — 1 sandbox Modal (16vCPU/32GB), 5 faze paralele' && echo ''" Enter
tmux send-keys -t "$SESSION:0.0" \
  "hermes chat -t terminal,file --yolo 2>&1 | tee '$LOG_FILE'" Enter

# Auto-paste master prompt after sandbox is ready (~10s)
(
  sleep 10
  tmux load-buffer -b "octomus" "$MASTER_PROMPT"
  tmux paste-buffer -t "$SESSION:0.0" -b "octomus"
  tmux send-keys -t "$SESSION:0.0" Enter
  echo "[$(date '+%H:%M:%S')] ✅ Auto-pasted master prompt" >> "$LOGS_DIR/orchestrator.log"
) &

# ── Pane 1: Coordinator shell ───────────────────────────────────────
tmux send-keys -t "$SESSION:0.1" \
  "cd $REPO_DIR && clear" Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "🎯 COORDINATOR — 1 sandbox, 5 faze" && echo ""' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "Agentul master (panoul de sus) execută secvențial:"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  PHASE 1 — șterge directoarele cloud"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  PHASE 2 — editează lib.rs"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  PHASE 3 — curăță ai/mod.rs, llms.rs"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  PHASE 4 — branding Warp→Octomus"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  PHASE 5 — Cargo.toml, settings, voice"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  MERGE  — combină branch-urile + cargo check"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo ""' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "STEERING: scrie direct în panoul de sus (Ctrl+b săgeată-sus)"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  Ex: skip phase 4, show status, stop"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo ""' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "COMENZI UTILE:"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  tail -f .hermes-logs/octomus.log     # vezi log-ul live"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  bash scripts/merge-octomus.sh        # merge manual (dacă e nevoie)"' Enter
tmux send-keys -t "$SESSION:0.1" \
  'echo "  git --no-pager log --oneline -10     # vezi commit-uri recente"' Enter

# ── Finalize ────────────────────────────────────────────────────────
tmux select-pane -t "$SESSION:0.0"

echo ""
echo "  🐙 Octomus — 1 sandbox Modal, 5 faze"
echo "  ═════════════════════════════════════"
echo ""
echo "  📡 1 sandbox Modal: 16vCPU / 32GB RAM"
echo "  🔑 GITHUB_TOKEN auto-pasat"
echo "  ⏱️  Lifetime: 4 ore (14400s)"
echo ""
echo "  ┌─────────────────────────────────────┐"
echo "  │ 🐙 Master Agent (hermes chat --yolo) │"
echo "  │ 5 faze → 5 branch-uri → merge final │"
echo "  ├─────────────────────────────────────┤"
echo "  │ 🎯 Coordinator                      │"
echo "  │ steering + comenzi manuale          │"
echo "  └─────────────────────────────────────┘"
echo ""
echo "  Attach:  tmux attach -t $SESSION"
echo "  Navig:   Ctrl+b ↑↓  între panouri"
echo "  Zoom:    Ctrl+b z   (panoul curent devine full-screen)"
echo "  Detach:  Ctrl+b d"
echo ""
echo "  Prompt-ul se auto-pastează după ~10s (cât pornește sandbox-ul)"
echo "  Poți da steer oricând tastând în panoul de sus."
