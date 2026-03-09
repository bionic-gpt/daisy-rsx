#!/usr/bin/env bash
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../../../../" && pwd)"

cd "$WORKSPACE_ROOT"

FILES="${1:-}"
if [[ -z "$FILES" ]]; then
  FILES="$(git diff --name-only)"
fi

if [[ -z "$FILES" ]]; then
  echo "No changed files detected."
  exit 0
fi

shared_hit=0
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  case "$file" in
    crates/daisy_rsx/src/marketing/*|crates/ssg_whiz/src/layouts/*|crates/ssg_whiz/src/lib.rs)
      if [[ "$shared_hit" -eq 0 ]]; then
        echo "Shared surface changes detected:"
      fi
      echo "- $file"
      shared_hit=1
      ;;
  esac
done <<< "$FILES"

if [[ "$shared_hit" -eq 1 ]]; then
  echo
  echo "Potentially affected sites:"
  echo "- crates/bionic-gpt"
  echo "- crates/decision"
  echo "- crates/deploy-mcp"
  exit 2
fi

echo "No shared-surface marketing/layout changes detected."
