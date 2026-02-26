# Memory

```js
# Search memory by semantic query
memory_search(query: string, limit: number): object[]

# List memory entries for an agent/session
memory_list(scope: string, limit: number): object[]

# Store a new memory item
memory_write(scope: string, content: string, metadata: object): string

# Remove a memory item
memory_delete(memory_id: string): string
```

## 📌 Why Memory Matters for LLMs & Agents

Without memory, an LLM (or agent built on top of one) is **stateless**: every interaction is independent, and the model has no concept of *past experiences*, *user context*, or *learned preferences*. That severely limits practical utility:

* **No lasting personalization:** It can’t remember user names, preferences, or long-running tasks.
* **Poor continuity:** Each session starts from scratch, leading to repetitive questions or redundant reasoning.
* **Limited long-horizon reasoning:** Agents can’t integrate past outcomes to inform future planning.
* **Higher cost & inefficiency:** Large context windows are inefficient; memory lets models retrieve context only when needed. ([Medium][1])

As one developer put it:

> *“Without memory, an AI agent is just a stateless function. With memory, it becomes a continuous mind.”*

Memory transforms LLMs into adaptive systems that are **persistent, contextual, and capable of growth**—key traits for agentic intelligence.

## Memory in OpenClaw
