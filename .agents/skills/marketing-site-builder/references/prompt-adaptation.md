# Prompt Adaptation

Adapt generic landing-page prompts (React/Vite/GSAP style) to this Rust RSX + SSG architecture.

## Translate Stack Assumptions

Map this:

- React app scaffolding
- Tailwind/JS animation requirements
- monolithic `App.jsx`

To this:

- RSX components in `crates/<site>/src/pages/**`
- reusable components in `crates/daisy_rsx/src/marketing/**`
- page/layout composition via `ssg_whiz`
- generated static output in `crates/<site>/dist`

## Preserve Intent, Not Literal Instructions

When prompt demands cinematic or high-fidelity outcomes, preserve:

- hierarchy and visual rhythm
- section sequencing
- conversion clarity (hero, proof, features, CTA)
- motion intent where existing architecture supports it

Do not force incompatible requirements (for example, required GSAP hooks) into this codebase.

## Mapping Guide

- "build new app" -> add/edit `src/pages` RSX and supporting assets/content.
- "new section component" -> implement in site-local page first; extract to `daisy_rsx` only if shared.
- "global design system" -> enforce with shared classes/tokens used by the target site.
- "interactive card" -> use current RSX patterns and CSS capabilities in the project.

## Reuse Priority

1. Existing site-local sections.
2. Existing `daisy_rsx::marketing` components.
3. New site-local component.
4. New shared component (only with clear multi-site need).

## Quality Bar

- Keep page composition intentional and non-generic.
- Preserve existing brand voice per site.
- Avoid introducing placeholder imagery/copy in final output.
