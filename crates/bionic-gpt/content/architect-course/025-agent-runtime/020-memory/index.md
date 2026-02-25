# Memory

```txt
# Search memory by semantic query
memory_search(query: string, limit: number): object[]

# List memory entries for an agent/session
memory_list(scope: string, limit: number): object[]

# Store a new memory item
memory_write(scope: string, content: string, metadata: object): string

# Remove a memory item
memory_delete(memory_id: string): string
```
