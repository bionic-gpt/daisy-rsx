#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 4 ]]; then
  echo "Usage: $0 <site-slug> [site-title] [port] [base-url]"
  echo "Example: $0 acme-site"
  echo "Example: $0 acme-site \"Acme Site\" 8084 https://acme.example.com"
  exit 1
fi

SITE_SLUG="$1"
SITE_TITLE="${2:-}"
SITE_PORT="${3:-8083}"
SITE_BASE_URL="${4:-}"

if [[ ! "$SITE_SLUG" =~ ^[a-z0-9][a-z0-9-]*$ ]]; then
  echo "Error: site-slug must use lowercase letters, digits, and hyphens."
  exit 1
fi

if [[ -z "$SITE_TITLE" ]]; then
  SITE_TITLE="$(awk 'BEGIN { RS="-"; ORS=" " } { print toupper(substr($0,1,1)) tolower(substr($0,2)) }' <<<"$SITE_SLUG" | sed 's/[[:space:]]*$//')"
fi

if [[ -z "$SITE_BASE_URL" ]]; then
  SITE_BASE_URL="https://${SITE_SLUG}.example.com"
fi

SITE_SNAKE="${SITE_SLUG//-/_}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_ROOT="$(pwd)"
TEMPLATE_DIR="$SCRIPT_DIR/../assets/minimal-site-template"
TARGET_DIR="$WORKSPACE_ROOT/crates/$SITE_SLUG"
WORKSPACE_CARGO="$WORKSPACE_ROOT/Cargo.toml"

if [[ ! -f "$WORKSPACE_CARGO" ]]; then
  echo "Error: run this script from the workspace root that contains Cargo.toml"
  exit 1
fi

if [[ ! -d "$TEMPLATE_DIR" ]]; then
  echo "Error: template directory not found: $TEMPLATE_DIR"
  exit 1
fi

if [[ -e "$TARGET_DIR" ]]; then
  echo "Error: target already exists: $TARGET_DIR"
  exit 1
fi

mkdir -p "$TARGET_DIR"
cp -R "$TEMPLATE_DIR"/. "$TARGET_DIR/"

while IFS= read -r -d '' file; do
  sed -i \
    -e "s/__SITE_SLUG__/$SITE_SLUG/g" \
    -e "s/__SITE_SNAKE__/$SITE_SNAKE/g" \
    -e "s/__SITE_TITLE__/$SITE_TITLE/g" \
    -e "s/__SITE_PORT__/$SITE_PORT/g" \
    -e "s|__SITE_BASE_URL__|$SITE_BASE_URL|g" \
    "$file"
done < <(find "$TARGET_DIR" -type f -print0)

if ! rg -q "\"crates/$SITE_SLUG\"" "$WORKSPACE_CARGO"; then
  sed -i '/^members = \[/a\    "crates/'"$SITE_SLUG"'",' "$WORKSPACE_CARGO"
fi

echo "Created crates/$SITE_SLUG"
echo "Next steps:"
echo "  cargo check -p $SITE_SLUG"
echo "  cargo run -p $SITE_SLUG"
