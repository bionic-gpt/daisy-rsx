codex: 
    sudo npm install -g @openai/codex

ws:
    cd /workspace/crates/static-website && cargo watch --workdir /workspace/crates/static-website -w ./content -w ./src --no-gitignore -x "run --bin static-website"

wts:
    cd /workspace/crates/static-website && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch
