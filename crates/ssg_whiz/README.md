# ssg_whiz

`ssg_whiz` is a somewhat experimental, opinionated static site generator for Rust + Dioxus.
It is designed for teams that want a consistent marketing/docs/blog layout with minimal wiring.

## Usage

You typically:

1. Define your `SiteConfig` (dist dir, links, meta, optional header).
2. Build your summaries (blog, docs, pages).
3. Provide a static page generator for bespoke pages.
4. Call the builder DSL.

### Typical Folder Layout

```
my-site/
├── assets/
├── content/
│   ├── blog/
│   │   └── hello-world.md
│   ├── docs/
│   │   └── getting-started.md
│   └── pages/
│       └── pricing.md
├── src/
│   ├── blog_summary.rs
│   ├── docs_summary.rs
│   ├── pages_summary.rs
│   ├── generator.rs
│   └── main.rs
└── Cargo.toml
```

### Example

```rust
use std::net::SocketAddr;

use ssg_whiz::{SiteBuilder, SiteConfig};
use ssg_whiz::summaries::DocumentSite;
use daisy_rsx::marketing::navigation::Section;
use ssg_whiz::summaries::{Category, PageSummary, Summary};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server: false,
        addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
        live_reload: false,
        navigation_links: my_ui::navigation_links(),
        footer_links: my_ui::footer_links(),
        site_meta: my_ui::site_meta(),
        site_header: None,
    };

    let blog_summary = Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Announcements".to_string(),
            pages: vec![PageSummary {
                date: "2026-02-06",
                title: "Hello, world",
                description: "First post from the new site.",
                folder: "hello-world",
                markdown: "hello-world.md",
                image: None,
                author: Some("Ada Lovelace"),
                author_image: None,
            }],
        }],
    };
    let docs_summary = Summary {
        source_folder: "docs",
        categories: vec![Category {
            name: "Guides".to_string(),
            pages: vec![PageSummary {
                date: "2026-02-01",
                title: "Getting Started",
                description: "Install and configure the product.",
                folder: "getting-started",
                markdown: "getting-started.md",
                image: None,
                author: None,
                author_image: None,
            }],
        }],
    };
    let pages_summary = Summary {
        source_folder: "pages",
        categories: vec![Category {
            name: "Marketing".to_string(),
            pages: vec![PageSummary {
                date: "2026-01-20",
                title: "Pricing",
                description: "Plan comparison and features.",
                folder: "pricing",
                markdown: "pricing.md",
                image: None,
                author: None,
                author_image: None,
            }],
        }],
    };

    SiteBuilder::new(config)
        .blog(blog_summary)
        .pages(pages_summary)
        .document(DocumentSite {
            summary: docs_summary,
            section: Section::Docs,
        })
        .static_pages(my_site::generate_static_pages)
        .build()
        .await?;

    Ok(())
}
```

### Notes

- `ssg_whiz` is opinionated: it copies `assets/` into `dist/` and copies `content/<summary>/`
  into `dist/<summary>/` automatically.
- The API is still evolving; expect changes as the DSL gets refined.
