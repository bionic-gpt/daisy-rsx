# New Site Checklist

## 1. Scaffold

Run:

```bash
/workspace/.codex/skills/marketing-site-builder/scripts/new_site_from_baseline.sh <new-site-slug>
```

This copies `crates/bionic-gpt` into `crates/<new-site-slug>` and rewrites core crate identifiers.

## 2. Confirm Workspace Wiring

- `Cargo.toml` workspace members include `crates/<new-site-slug>`.
- `crates/<new-site-slug>/Cargo.toml` package name is `<new-site-slug>`.
- `crates/<new-site-slug>/src/main.rs` references the crate module with underscore form (`<new_site_slug>::...`).

## 3. Replace Baseline Branding

Prioritize these files:

- `crates/<new-site-slug>/src/ui_links.rs`
- `crates/<new-site-slug>/src/pages/home.rs`
- `crates/<new-site-slug>/src/pages/pricing.rs`
- `crates/<new-site-slug>/src/pages/contact.rs`
- `crates/<new-site-slug>/assets/logo.svg` and social sharing assets
- `crates/<new-site-slug>/content/pages/privacy.md`
- `crates/<new-site-slug>/content/pages/terms.md`

## 4. Keep Shared Surfaces Stable

- Avoid editing `crates/ssg_whiz` or `crates/daisy_rsx/src/marketing` unless explicitly needed.
- Prefer site-local overrides while bootstrapping.

## 5. Validate

Run:

```bash
/workspace/.codex/skills/marketing-site-builder/scripts/validate_site.sh <new-site-slug>
```

## 6. Final Review

- No placeholder text remains.
- Navigation/footer links are coherent for new brand.
- Dist pages and assets resolve locally.
