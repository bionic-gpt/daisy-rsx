use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![Category {
            name: "Getting Started".to_string(),
            pages: vec![
                PageSummary {
                    date: "2026-03-16",
                    title: "Getting Started",
                    description: "A starter documentation section for your new static site.",
                    folder: "docs/getting-started",
                    markdown: include_str!("../content/docs/getting-started/index.md"),
                    image: None,
                    author: None,
                    author_image: None,
                },
                PageSummary {
                    date: "2026-03-17",
                    title: "Configuration",
                    description: "Configure the generated example site.",
                    folder: "docs/configuration",
                    markdown: include_str!("../content/docs/configuration/index.md"),
                    image: None,
                    author: None,
                    author_image: None,
                },
            ],
        }],
    }
}
