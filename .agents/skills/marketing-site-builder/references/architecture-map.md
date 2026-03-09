# Architecture Map

## Workspace Components

- `crates/ssg_whiz`: static site generator and shared layouts.
- `crates/daisy_rsx`: reusable RSX UI components including marketing blocks.
- `crates/bionic-gpt`: marketing site + docs/blog/pages.
- `crates/decision`: marketing site using shared marketing blocks.
- `crates/deploy-mcp`: marketing site + docs/blog/pages + MCP catalog pages.

## How Rendering Flows

1. Site crate `src/main.rs` builds a `SiteConfig`.
2. Site crate passes summaries and static page generators into `SiteBuilder`.
3. `ssg_whiz` renders pages and writes `dist/`.
4. Optional server runs unless `DO_NOT_RUN_SERVER=1`.

## Shared Surfaces

### High-risk shared files

- `crates/ssg_whiz/src/layouts/*.rs`
- `crates/ssg_whiz/src/lib.rs`
- `crates/daisy_rsx/src/marketing/*.rs`

Changes here can impact multiple sites.

### Site-local surfaces

- `crates/<site>/src/pages/**`
- `crates/<site>/src/ui_links.rs`
- `crates/<site>/content/**`
- `crates/<site>/assets/**`

Prefer these when only one site needs updates.

## Site Runtime Defaults

- `bionic-gpt`: port `8080`
- `deploy-mcp`: port `8081`
- `decision`: port `8082`

## Existing Watch Commands

From `/workspace/Justfile`:

- `just bionic`
- `just deploy`
- `just decision`
- plus `*-tw` commands for Tailwind output.

## Validation Pattern

For deterministic validation in automation, run generation without server:

```bash
DO_NOT_RUN_SERVER=1 cargo run --manifest-path crates/<site>/Cargo.toml --bin <site>
```

Then check `crates/<site>/dist` for broken internal references and placeholder content.
