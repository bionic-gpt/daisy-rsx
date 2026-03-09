#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <site-slug>"
  exit 1
fi

SITE="$1"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../../../../" && pwd)"
CRATE_DIR="$WORKSPACE_ROOT/crates/$SITE"
DIST_DIR="$CRATE_DIR/dist"
BASELINE_DIR="$SCRIPT_DIR/../references/broken-link-baselines"
BASELINE_FILE="$BASELINE_DIR/$SITE.txt"

if [[ ! -d "$CRATE_DIR" ]]; then
  echo "Error: site crate not found: $CRATE_DIR"
  exit 1
fi

echo "[1/3] Build/generate site: $SITE"
BUILD_LOG="/tmp/validate_site_${SITE}.log"
(cd "$CRATE_DIR" && DO_NOT_RUN_SERVER=1 cargo run --bin "$SITE") 2>&1 | tee "$BUILD_LOG" || {
  echo "Build failed. Showing tail of log:"
  tail -n 80 "$BUILD_LOG"
  exit 1
}

echo "[2/3] Placeholder content check"
PLACEHOLDER_MATCHES="$(rg -n -i --glob '*.html' 'TODO|lorem ipsum' "$DIST_DIR" || true)"
if [[ -n "$PLACEHOLDER_MATCHES" ]]; then
  echo "Placeholder-like text found in generated HTML:"
  echo "$PLACEHOLDER_MATCHES"
  exit 1
fi

echo "[3/3] Internal link and asset check"
if [[ ! -f "$DIST_DIR/index.html" ]]; then
  echo "Missing dist/index.html for site: $SITE"
  exit 1
fi

broken=0
checked=0
broken_tmp="$(mktemp)"

while IFS= read -r match; do
  file="${match%%:*}"
  rest="${match#*:}"
  rest="${rest#*:}"
  url="${rest#*=\"}"
  url="${url%\"}"

  case "$url" in
    ''|'#'*|http://*|https://*|//*|mailto:*|tel:*|data:*|javascript:*)
      continue
      ;;
  esac

  clean="${url%%#*}"
  clean="${clean%%\?*}"
  [[ -z "$clean" ]] && continue

  if [[ "$clean" == /* ]]; then
    target_abs="$DIST_DIR/${clean#/}"
  else
    target_abs="$(realpath -m "$(dirname "$file")/$clean")"
  fi

  checked=$((checked + 1))

  if [[ "$target_abs" != "$DIST_DIR"* ]]; then
    echo "$file -> $url" >> "$broken_tmp"
    broken=$((broken + 1))
    continue
  fi

  if [[ "$target_abs" == */ ]]; then
    candidate="$target_abs/index.html"
    [[ -f "$candidate" ]] || {
      echo "$file -> $url" >> "$broken_tmp"
      broken=$((broken + 1))
    }
    continue
  fi

  if [[ -f "$target_abs" || -d "$target_abs" || -f "$target_abs/index.html" ]]; then
    continue
  fi

  echo "$file -> $url" >> "$broken_tmp"
  broken=$((broken + 1))
done < <(rg -n --glob '*.html' -o '(href|src)="[^"]+"' "$DIST_DIR")

if [[ "$broken" -eq 0 ]]; then
  rm -f "$broken_tmp"
  echo "Validation passed: $checked references checked, 0 broken, no placeholder content found."
  exit 0
fi

sort -u "$broken_tmp" -o "$broken_tmp"

if [[ -f "$BASELINE_FILE" ]]; then
  unexpected_tmp="$(mktemp)"
  comm -23 "$broken_tmp" <(sort -u "$BASELINE_FILE") > "$unexpected_tmp"

  if [[ -s "$unexpected_tmp" ]]; then
    echo "Validation failed: unexpected broken references detected:"
    cat "$unexpected_tmp"
    echo "Total checked: $checked"
    rm -f "$broken_tmp" "$unexpected_tmp"
    exit 1
  fi

  echo "Validation passed with known baseline issues for $SITE ($broken known broken references)."
  echo "Known baseline file: $BASELINE_FILE"
  rm -f "$broken_tmp" "$unexpected_tmp"
  exit 0
fi

echo "Validation failed: $broken broken references out of $checked checked."
cat "$broken_tmp"
rm -f "$broken_tmp"
exit 1
