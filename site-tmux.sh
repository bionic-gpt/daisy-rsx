#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${SCRIPT_DIR}"
TMUX_CONFIG="/tmp/site-tmux.conf"
DEBUG="${SITE_TMUX_DEBUG:-0}"
SESSION_NAME="example-site"
SITE_ROOT="${REPO_ROOT}/crates/example-site"

on_error() {
  local exit_code="$?"
  echo "[site-tmux] failed at line ${BASH_LINENO[0]} with exit ${exit_code}" >&2
  exit "$exit_code"
}

trap on_error ERR

in_devcontainer() {
  [ -n "${REMOTE_CONTAINERS:-}" ] || [ -n "${CODESPACES:-}" ] || [ -f "/.dockerenv" ]
}

debug() {
  if [[ "$DEBUG" == "1" ]]; then
    echo "[site-tmux] $*" >&2
  fi
}

ensure_tmux() {
  debug "ensuring tmux is installed"
  if command -v tmux >/dev/null 2>&1; then
    debug "tmux already installed at $(command -v tmux)"
    return
  fi

  debug "tmux missing, installing via apt"
  sudo apt update
  sudo apt install -y tmux
}

write_tmux_config() {
  debug "writing tmux config to $TMUX_CONFIG"
  cat > "$TMUX_CONFIG" <<'EOF'
set -g status on
set -g status-position bottom
set -g pane-border-status top
set -g pane-border-format "#{pane_title}"
set-option -g mouse on
bind -n F1 select-window -t 0
EOF
}

attach_session() {
  local session="$1"

  debug "attaching to session '$session' (TMUX=${TMUX:-<empty>})"

  if [[ -n "${TMUX:-}" ]] && tmux display-message -p '#S' >/dev/null 2>&1; then
    exec tmux switch-client -t "$session"
  fi

  exec tmux attach-session -t "$session"
}

start_site_tmux() {
  debug "starting site mode in $SITE_ROOT"

  if [[ ! -d "$SITE_ROOT" ]]; then
    echo "site root not found: $SITE_ROOT" >&2
    exit 1
  fi

  if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
    attach_session "$SESSION_NAME"
  fi

  tmux -f "$TMUX_CONFIG" new-session -d -s "$SESSION_NAME" -n site -c "$SITE_ROOT"

  tmux select-pane -t "$SESSION_NAME:0.0" -T "site"
  tmux send-keys -t "$SESSION_NAME:0.0" "just example" C-m

  tmux split-window -h -t "$SESSION_NAME:0" -c "$SITE_ROOT"
  tmux select-pane -t "$SESSION_NAME:0.1" -T "tailwind"
  tmux send-keys -t "$SESSION_NAME:0.1" "just example-tw" C-m

  tmux select-pane -t "$SESSION_NAME:0.0"
  tmux select-layout -t "$SESSION_NAME:0" even-horizontal
  attach_session "$SESSION_NAME"
}

main() {
  debug "repo_root='$REPO_ROOT' in_devcontainer=$(in_devcontainer && echo yes || echo no)"

  ensure_tmux
  write_tmux_config
  start_site_tmux
}

if in_devcontainer; then
  main
else
  command -v devcontainer >/dev/null 2>&1 || {
    echo "devcontainer CLI is not installed." >&2
    echo "Install it with: npm install -g @devcontainers/cli" >&2
    exit 1
  }

  devcontainer up --workspace-folder "$REPO_ROOT" >/dev/null
  exec devcontainer exec --workspace-folder "$REPO_ROOT" bash -lc "cd '$REPO_ROOT' && ./site-tmux.sh"
fi
