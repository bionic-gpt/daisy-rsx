# Sandboxes

```txt
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
