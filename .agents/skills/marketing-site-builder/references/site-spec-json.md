# Page Spec JSON

Use this JSON contract for landing page redesign and new landing page builds.

## Rules

- Provide one JSON object.
- Fill all required keys.
- Animation behavior must be explicit under `animations`.
- Copy in JSON is treated as final source text.

## Required Schema (shape)

```json
{
  "site": {
    "slug": "decision",
    "page": "home",
    "theme": "midnight-luxe"
  },
  "seo": {
    "title": "...",
    "meta_description": "...",
    "og_headline": "...",
    "og_description": "..."
  },
  "nav": {
    "labels": ["...", "...", "...", "..."],
    "cta": "..."
  },
  "hero": {
    "kicker": "...",
    "headline": "...",
    "subheadline": "...",
    "cta": "..."
  },
  "artifacts": {
    "intro_title": "...",
    "intro_text": "...",
    "diagnostic_shuffler": {
      "title": "...",
      "descriptor": "...",
      "items": ["...", "...", "..."]
    },
    "telemetry_typewriter": {
      "title": "...",
      "descriptor": "...",
      "feeds": ["...", "...", "..."]
    },
    "cursor_protocol_scheduler": {
      "title": "...",
      "descriptor": "...",
      "helper": "..."
    }
  },
  "manifesto": {
    "contrast": "...",
    "focus": "...",
    "support": "..."
  },
  "protocol": {
    "title": "...",
    "subtitle": "...",
    "steps": [
      { "label": "...", "headline": "...", "description": "..." },
      { "label": "...", "headline": "...", "description": "..." },
      { "label": "...", "headline": "...", "description": "..." }
    ]
  },
  "cta": {
    "headline": "...",
    "subheadline": "...",
    "label": "..."
  },
  "footer": {
    "brand": "...",
    "status": "...",
    "nav": {
      "col1": { "title": "...", "links": ["...", "..."] },
      "col2": { "title": "...", "links": ["...", "..."] },
      "col3": { "title": "...", "links": ["...", "..."] }
    }
  },
  "animations": {
    "nav": {
      "morph_on_scroll": true,
      "threshold": 0.35,
      "hover_lift_px": 1,
      "cta_magnetic_scale": 1.03
    },
    "hero": {
      "entrance": "fade-up",
      "duration_ms": 900,
      "translate_y_px": 40
    },
    "diagnostic_shuffler": {
      "interval_ms": 3000,
      "rotation": "move-last-to-front",
      "easing": "cubic-bezier(0.34,1.56,0.64,1)"
    },
    "telemetry_typewriter": {
      "char_interval_ms": 30,
      "line_pause_ms": 900,
      "cursor_blink_ms": 1000
    },
    "scheduler": {
      "step_interval_ms": 2200,
      "activate_delay_ms": 140,
      "move_to_save_delay_ms": 500
    },
    "manifesto": {
      "parallax": true,
      "max_offset_px": 26
    },
    "protocol_stack": {
      "sticky_top_px": 88,
      "dimmed_scale": 0.9,
      "dimmed_blur_px": 20,
      "dimmed_opacity": 0.5
    },
    "reduced_motion": {
      "disable_nonessential": true
    }
  }
}
```

## Implementation Contract

- `site.theme` selects visual tokens.
- `animations` values drive JS/CSS constants directly.
- If any required key is missing, ask one question at a time to fill gaps before coding.
- Keep all rendered copy aligned with JSON values.
