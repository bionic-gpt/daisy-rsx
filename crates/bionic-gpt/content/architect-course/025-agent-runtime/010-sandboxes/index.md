# Sandboxes

```js
# Read a file from the sandbox
read(path: string): string

# Write a file in the sandbox
write(path: string, content: string): string

# Apply a patch/edit to an existing file
edit(path: string, diff: string): string

# Execute code and return stdout/stderr
exec(code: string): string

# Run a command/process in the sandbox
process(command: string, args: string[]): string
```

## Further Reading

- [OpenClaw Sandbox](https://docs.openclaw.ai/gateway/sandboxing)
- [Code Mode: give agents an entire API in 1,000 tokens](https://blog.cloudflare.com/code-mode-mcp/)
- [IronClaw sandbox implementation](https://github.com/nearai/ironclaw/tree/main/src/sandbox)
- [AI Sandboxes Startup](https://e2b.dev/)
