# Site Playbooks

## bionic-gpt

- Crate root: `crates/bionic-gpt`
- Main entry: `crates/bionic-gpt/src/main.rs`
- Common page edits:
  - `crates/bionic-gpt/src/pages/home.rs`
  - `crates/bionic-gpt/src/pages/pricing.rs`
  - `crates/bionic-gpt/src/pages/contact.rs`
- Navigation/footer/meta:
  - `crates/bionic-gpt/src/ui_links.rs`
- Content-driven sections:
  - `crates/bionic-gpt/content/docs/**`
  - `crates/bionic-gpt/content/blog/**`
  - `crates/bionic-gpt/content/pages/**`

## decision

- Crate root: `crates/decision`
- Main entry: `crates/decision/src/main.rs`
- Primary landing page:
  - `crates/decision/src/pages/index.rs`
- Navigation/footer/meta:
  - `crates/decision/src/ui_links.rs`
- Page summaries/generation:
  - `crates/decision/src/pages_summary.rs`
  - `crates/decision/src/generator.rs`

## deploy-mcp

- Crate root: `crates/deploy-mcp`
- Main entry: `crates/deploy-mcp/src/main.rs`
- Marketing pages:
  - `crates/deploy-mcp/src/pages/home.rs`
  - `crates/deploy-mcp/src/pages/enterprise.rs`
  - `crates/deploy-mcp/src/pages/pricing.rs`
  - `crates/deploy-mcp/src/pages/contact.rs`
- Navigation/footer/meta:
  - `crates/deploy-mcp/src/ui_links.rs`
- Content-driven sections:
  - `crates/deploy-mcp/content/docs/**`
  - `crates/deploy-mcp/content/blog/**`
  - `crates/deploy-mcp/content/pages/**`

## Shared Marketing Components

- Path: `crates/daisy_rsx/src/marketing`
- Common blocks include:
  - hero variants
  - features, benefits, testimonials
  - navigation/footer
  - FAQ and section primitives

Use shared edits only when reuse across two or more sites is clear.

## Shared Layout Layer

- Path: `crates/ssg_whiz/src/layouts`
- Responsibilities:
  - page shell
  - blog/docs/pages rendering
  - metadata and summary wiring

Treat as framework-level changes; validate at least target site and one additional site when touched.
