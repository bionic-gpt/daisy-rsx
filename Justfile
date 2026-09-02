codex: 
    sudo apt update && sudo apt install -y bubblewrap
    sudo chmod u-s /usr/bin/bwrap
    sudo chown -R vscode:vscode /home/vscode/.codex
    sudo npm install -g @openai/codex

install-d2:
    #!/usr/bin/env bash
    set -euo pipefail
    curl -fsSL https://d2lang.com/install.sh | sh -s --
    sudo install -m 0755 "$HOME/.local/bin/d2" /usr/local/bin/d2
    d2 --version

example:
    cd /workspace/crates/example-site && cargo watch --workdir /workspace/crates/example-site -w ./content -w ./src -w /workspace/crates/ssg_whiz/src --no-gitignore -x "run --bin example-site"

example-tw:
    cd /workspace/crates/example-site && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch