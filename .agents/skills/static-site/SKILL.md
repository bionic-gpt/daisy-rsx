---
name: static-site
description: Scaffold a minimal Rust static-site crate in this workspace using ssg_whiz and daisy_rsx. Use when the user wants a new site crate created under crates/<name> with a working homepage, blog, assets, custom JavaScript, and Cloudflare build script.
---

# Static Site

Use this skill to create a new site crate with the smallest baseline that still runs.

## Inputs

- Required: site slug, for example `acme-site`
- Optional: site title, port, base URL

## Workflow

1. Run `scripts/new_static_site.sh <slug> [title] [port] [base-url]`.
2. Review the generated crate in `crates/<slug>`.
3. Validate with `cargo check -p <slug>`.
4. If needed, run `cargo run -p <slug>`.

## What The Template Includes

- `src/main.rs`, `src/generator.rs`, `src/pages/index.rs`, `src/ui_links.rs`
- one starter blog post
- `assets/copy-paste.js` and `assets/goat-counter.js`
- `cloudflare-build.sh`
- `input.css`

## Notes

- The script also adds the new crate to the workspace `Cargo.toml`.
- Keep the generated crate generic first. Brand-specific redesign work should happen after scaffold.
