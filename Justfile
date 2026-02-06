codex: 
    sudo npm install -g @openai/codex

bionic:
    cd /workspace/crates/bionic-gpt && cargo watch --workdir /workspace/crates/bionic-gpt -w ./content -w ./src --no-gitignore -x "run --bin bionic-gpt"

bionic-tw:
    cd /workspace/crates/bionic-gpt && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

deploy:
    cd /workspace/crates/deploy-mcp && cargo watch --workdir /workspace/crates/deploy-mcp -w ./content -w ./src --no-gitignore -x "run --bin deploy-mcp"

deploy-tw:
    cd /workspace/crates/bionic-gpt && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch