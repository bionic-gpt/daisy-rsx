use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "pages",
        categories: vec![Category {
            name: "Getting Started".to_string(),
            pages: vec![PageSummary {
                date: "2026-03-16",
                title: "Getting Started",
                description: "A starter documentation page for your new static site.",
                folder: "getting-started",
                markdown: include_str!("../content/pages/getting-started/index.md"),
                image: None,
                author: None,
                author_image: None,
            }],
        }],
    }
}
