#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")/.."

# Cleanup la iesire (Ctrl+C, exit normal, etc.)
INJECTED="/tmp/octomus-stage1-master-$$.txt"
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

# Curatam orice fisier injectat vechi
rm -f /tmp/octomus-stage1-master-*.txt

# Generam master prompt cu token-ul injectat direct
sed "s|__GH_TOKEN__|$GITHUB_TOKEN|g" .hermes-prompts/master-octomus-stage1.txt > "$INJECTED"

# Verificam ca token-ul a fost injectat corect
if ! grep -q "$GITHUB_TOKEN" "$INJECTED"; then
  echo "ERROR: Token-ul nu a fost injectat in prompt!"
  exit 1
fi

echo "Prompt generat cu token verificat (${GITHUB_TOKEN:0:10}...). Lansez Hermes..."
echo ""

# Pornim Hermes direct — prompt-ul e injectat ca input initial
hermes chat --yolo < "$INJECTED"
