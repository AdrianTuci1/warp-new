#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")/.."

INJECTED="/tmp/octomus-merge-stage1-$$.txt"
trap "rm -f $INJECTED" EXIT

# Obtinem token-ul din hermes .env
if [ -f "$HOME/.hermes/.env" ]; then
  export $(grep -v '^#' "$HOME/.hermes/.env" | grep GITHUB_TOKEN | xargs)
fi

if [ -z "$GITHUB_TOKEN" ]; then
  echo "Nu am putut obtine token-ul. Adauga GITHUB_TOKEN in ~/.hermes/.env."
  exit 1
fi

echo "Token obtinut ($(echo $GITHUB_TOKEN | head -c 10)...)"

# Generam merge prompt cu token-ul injectat
sed "s|__GH_TOKEN__|$GITHUB_TOKEN|g" .hermes-prompts/agent-merge-stage1.txt > "$INJECTED"

# Verificam injectia
if ! grep -q "$GITHUB_TOKEN" "$INJECTED"; then
  echo "ERROR: Token-ul nu a fost injectat!"
  exit 1
fi

echo "Prompt generat. Lansez Hermes pentru merge stage 1..."
echo ""

hermes chat --yolo < "$INJECTED"
