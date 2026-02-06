codex: 
    sudo npm install -g @openai/codex

ws:
    cd /workspace/crates/bionic-gpt && cargo watch --workdir /workspace/crates/bionic-gpt -w ./content -w ./src --no-gitignore -x "run --bin bionic-gpt"

wts:
    cd /workspace/crates/bionic-gpt && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
