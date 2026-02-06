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
│       └── terms.md
├── src/
│   ├── blog_summary.rs
│   ├── docs_summary.rs
│   ├── pages/
│   │   └── index.rs
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
                title: "Terms",
                description: "Terms of service.",
                folder: "terms",
                markdown: "terms.md",
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

### Example `src/pages/index.rs`

This is a small, component-driven page. It uses `daisy_rsx` components and returns HTML
via Dioxus SSR.

```rust
use dioxus::prelude::*;
use daisy_rsx::marketing::hero::Hero;

pub fn page() -> String {
    dioxus_ssr::render_element(rsx!(
        main {
            class: "container mx-auto px-6 py-16",
            Hero {
                title: "Welcome to Whiz".to_string(),
                subtitle: "An opinionated static site generator.".to_string(),
                cta_label: Some("Get Started".to_string()),
                cta_href: Some("/docs".to_string()),
            }
        }
    ))
}
```

### Example `src/generator.rs`

The generator collects your component-built pages and returns a list of `SitePage`
values. `SiteBuilder` writes them to `dist/` and wires them into the site alongside
your markdown summaries.

```rust
use ssg_whiz::SitePage;

use crate::pages;

fn output_page(path: &str, html: String) -> SitePage {
    SitePage {
        path: path.to_string(),
        html,
    }
}

pub async fn generate_static_pages() -> Vec<SitePage> {
    vec![
        output_page("", pages::index::page()),
    ]
}
```

### Notes

- `ssg_whiz` is opinionated: it copies `assets/` into `dist/` and copies `content/<summary>/`
  into `dist/<summary>/` automatically.
- The API is still evolving; expect changes as the DSL gets refined.
