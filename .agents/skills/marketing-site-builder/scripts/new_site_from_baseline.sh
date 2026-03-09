#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "Usage: $0 <new-site-slug> [source-site-slug]"
  echo "Example: $0 acme-ai"
  echo "Example: $0 acme-ai bionic-gpt"
  exit 1
fi

NEW_SLUG="$1"
SOURCE_SLUG="${2:-bionic-gpt}"

if [[ ! "$NEW_SLUG" =~ ^[a-z0-9][a-z0-9-]*$ ]]; then
  echo "Error: new-site-slug must use lowercase letters, digits, and hyphens."
  exit 1
fi

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../../../../" && pwd)"
SOURCE_DIR="$WORKSPACE_ROOT/crates/$SOURCE_SLUG"
TARGET_DIR="$WORKSPACE_ROOT/crates/$NEW_SLUG"
WORKSPACE_CARGO="$WORKSPACE_ROOT/Cargo.toml"

if [[ ! -d "$SOURCE_DIR" ]]; then
  echo "Error: source site does not exist: $SOURCE_DIR"
  exit 1
fi

if [[ -e "$TARGET_DIR" ]]; then
  echo "Error: target already exists: $TARGET_DIR"
  exit 1
fi

echo "Scaffolding crates/$NEW_SLUG from crates/$SOURCE_SLUG"
mkdir -p "$TARGET_DIR"
rsync -a --exclude dist --exclude target "$SOURCE_DIR/" "$TARGET_DIR/"

OLD_SNAKE="${SOURCE_SLUG//-/_}"
NEW_SNAKE="${NEW_SLUG//-/_}"

if [[ -f "$TARGET_DIR/Cargo.toml" ]]; then
  sed -i "s/^name = \"$SOURCE_SLUG\"/name = \"$NEW_SLUG\"/" "$TARGET_DIR/Cargo.toml"
fi

if [[ -f "$TARGET_DIR/src/main.rs" ]]; then
  sed -i "s/use $OLD_SNAKE::/use $NEW_SNAKE::/g" "$TARGET_DIR/src/main.rs"
  sed -i "s/$OLD_SNAKE::/$NEW_SNAKE::/g" "$TARGET_DIR/src/main.rs"
fi

if ! rg -q "crates/$NEW_SLUG" "$WORKSPACE_CARGO"; then
  sed -i '/^members = \[/a\    "crates/'"$NEW_SLUG"'",' "$WORKSPACE_CARGO"
  echo "Updated workspace members in Cargo.toml"
else
  echo "Workspace already references crates/$NEW_SLUG"
fi

echo "Scaffold complete: crates/$NEW_SLUG"
echo "Next: review branding/content and run validate_site.sh $NEW_SLUG"
