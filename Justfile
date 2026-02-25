codex: 
    sudo npm install -g @openai/codex

install-d2:
    #!/usr/bin/env bash
    set -euo pipefail
    curl -fsSL https://d2lang.com/install.sh | sh -s --
    sudo install -m 0755 "$HOME/.local/bin/d2" /usr/local/bin/d2
    d2 --version

bionic:
    cd /workspace/crates/bionic-gpt && cargo watch --workdir /workspace/crates/bionic-gpt -w ./content -w ./src -w /workspace/crates/ssg_whiz/src --no-gitignore -x "run --bin bionic-gpt"

bionic-tw:
    cd /workspace/crates/bionic-gpt && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

deploy:
    cd /workspace/crates/deploy-mcp && cargo watch --workdir /workspace/crates/deploy-mcp -w ./content -w ./src -w /workspace/crates/ssg_whiz/src --no-gitignore -x "run --bin deploy-mcp"

deploy-tw:
    cd /workspace/crates/bionic-gpt && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

decision:
    cd /workspace/crates/decision && cargo watch --workdir /workspace/crates/decision -w ./content -w ./src -w /workspace/crates/ssg_whiz/src --no-gitignore -x "run --bin decision"

decision-tw:
    cd /workspace/crates/decision && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
