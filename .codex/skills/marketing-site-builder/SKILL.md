---
name: marketing-site-builder
description: Maintain and build marketing websites in this Rust static-site workspace using crates/ssg_whiz and crates/daisy_rsx marketing components. Use for requests involving existing sites (crates/bionic-gpt, crates/decision, crates/deploy-mcp), shared UI components in crates/daisy_rsx/src/marketing, layout work in crates/ssg_whiz/src/layouts, or scaffolding a new site from the bionic-gpt baseline.
---

# Marketing Site Builder

Use this skill to safely modify current sites and create new ones without breaking shared architecture.

## Required Preflight Checklist

Complete this checklist before edits:

1. Confirm target mode: `maintain-existing` or `create-new-site`.
2. Confirm target site crate: `bionic-gpt`, `decision`, `deploy-mcp`, or a new slug.
3. Map likely touchpoints:
   - site-local files: `crates/<site>/src`, `crates/<site>/content`, `crates/<site>/assets`
   - shared marketing components: `crates/daisy_rsx/src/marketing`
   - shared layouts/rendering: `crates/ssg_whiz/src/layouts`, `crates/ssg_whiz/src/lib.rs`
4. State impact risk:
   - `site-local only`
   - `shared (may affect multiple sites)`
5. State validation commands that will run before finishing.

## Decision Rules

- Prefer site-local edits by default.
- Edit `crates/daisy_rsx/src/marketing` only when at least two sites need the same change.
- Keep shared component APIs backwards compatible unless a coordinated migration is requested.
- Preserve existing site tone and structure unless a redesign is explicitly requested.

## Architecture Navigation

1. Read `references/architecture-map.md` for shared architecture.
2. Read `references/site-playbooks.md` for site-specific entry points.
3. If adapting ideas from generic web prompts, read `references/prompt-adaptation.md`.

## Maintain Existing Site Workflow

1. Run preflight checklist.
2. Identify smallest site-local change first.
3. If shared change is necessary, list affected sites and compatibility approach.
4. Implement edits.
5. Run validation:
   - `scripts/validate_site.sh <site>`
6. Report:
   - changed files
   - whether shared surface changed
   - validation result

## Create New Site Workflow (Default Baseline: bionic-gpt)

1. Run preflight checklist with mode `create-new-site`.
2. Scaffold from baseline:
   - `scripts/new_site_from_baseline.sh <new-site-slug>`
3. Follow `references/new-site-checklist.md`.
4. Use starter templates from `assets/starter-from-bionic-gpt/`.
5. Implement requested content/style changes mainly in the new site crate.
6. Validate:
   - `scripts/validate_site.sh <new-site-slug>`

## Validation Standard

Minimum required before completion:

1. Site-specific build/generation passes.
2. Internal link and local asset references in generated `dist/*.html` resolve.
3. Generated HTML has no obvious placeholder copy (`TODO`, `lorem ipsum`, `coming soon`).

Use:

```bash
/workspace/.codex/skills/marketing-site-builder/scripts/validate_site.sh <site>
```

## Resource Index

- Architecture and targeting: `references/architecture-map.md`
- Existing site playbooks: `references/site-playbooks.md`
- Prompt adaptation for RSX/SSG: `references/prompt-adaptation.md`
- New site checklist: `references/new-site-checklist.md`
- Scaffolding script: `scripts/new_site_from_baseline.sh`
- Validation script: `scripts/validate_site.sh`
- Optional shared-impact script: `scripts/check_shared_impact.sh`
- Starter templates: `assets/starter-from-bionic-gpt/`
