#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")/.."

INJECTED="/tmp/octomus-stage2a-master-$$.txt"
trap "rm -f $INJECTED" EXIT

# Obtinem token-ul din hermes .env
if [ -f "$HOME/.hermes/.env" ]; then
  export $(grep -v '^#' "$HOME/.hermes/.env" | grep GITHUB_TOKEN | xargs)
fi

if [ -z "$GITHUB_TOKEN" ]; then
  echo "Nu am putut obtine token-ul. Adauga GITHUB_TOKEN in ~/.hermes/.env."
  echo "   Sau: $0 <GITHUB_TOKEN>"
  exit 1
fi

echo "Token obtinut ($(echo $GITHUB_TOKEN | head -c 10)...)"

mkdir -p .hermes-logs
rm -f /tmp/octomus-stage2a-master-*.txt

sed "s|__GH_TOKEN__|$GITHUB_TOKEN|g" .hermes-prompts/master-octomus-stage2a.txt > "$INJECTED"

if ! grep -q "$GITHUB_TOKEN" "$INJECTED"; then
  echo "ERROR: Token-ul nu a fost injectat in prompt!"
  exit 1
fi

echo "Prompt generat. Lansez Stage 2a (Cleanup & Infra: A8+A9+A10+A15)..."
echo ""

hermes chat --yolo < "$INJECTED"
