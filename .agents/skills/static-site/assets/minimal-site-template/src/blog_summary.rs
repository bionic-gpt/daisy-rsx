use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Updates".to_string(),
            pages: vec![PageSummary {
                date: "2026-03-09",
                title: "Hello from __SITE_TITLE__",
                description: "The first post for your new static site.",
                folder: "blog/hello-world",
                markdown: include_str!("../content/blog/hello-world/index.md"),
                image: None,
                author: None,
                author_image: None,
            }],
        }],
    }
}
