# Skills

```txt
# Minimum primitive for skills
read_file(path: string): string

# Optional helper for discovery
list_dir(path: string): string[]
```

```txt
my-skill/
├── SKILL.md           # Main instructions (required)
├── template.md        # Template for Claude to fill in
├── examples/
│   └── sample.md      # Example output showing expected format
└── scripts/
    └── validate.sh    # Script Claude can execute
```

A practical way to model skills:

- You only need a **read primitive** to load `SKILL.md` instructions.
- The runtime can discover skills from known folders, read their metadata/content, and decide what to expose to the model.
- At prompt-build time, skills are represented as structured context, for example as XML in the system prompt.

## Why Use Skills

Skills solve the gap between raw tools and reliable behavior.

- **Consistency**: every run gets the same workflow instructions.
- **Reuse**: package domain playbooks once, then apply everywhere.
- **Quality control**: templates/examples/scripts define what "good" looks like.
- **Safer execution**: constrain model behavior with explicit steps and checks.
- **Lower prompt burden**: users ask for outcomes, not full operating procedures.

Without skills, agents often re-invent workflows each run.
With skills, agents execute stable, repeatable playbooks.

## Skills in the Agentic Loop

Skills fit into the same tool-calling loop:

1. The user asks for a task.
2. The model decides a skill is relevant.
3. The runtime loads skill content (`SKILL.md`, templates, examples) via `read_file`.
4. The runtime injects skill metadata into the system prompt (for example as XML).
5. The model continues the normal loop: plan -> call tools -> receive results -> finalize.

So skills do not replace tool calls.
They shape **how** tool calls are chosen, ordered, and validated inside the loop.

How this maps to current systems:

- **OpenClaw** loads skill folders and injects a compact XML skills list into the system prompt.
- **Claude Code** loads skill descriptions into context and pulls full skill content when a skill is invoked.

## Further Reading

- [OpenClaw Skills](https://docs.openclaw.ai/tools/skills)
- [Claude Code Skills](https://code.claude.com/docs/en/skills)
