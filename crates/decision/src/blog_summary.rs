use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Updates".to_string(),
            pages: vec![PageSummary {
                date: "2026-02-06",
                title: "Hello world",
                description: "First post from Decision.",
                folder: "hello-world",
                markdown: "hello-world.md",
                image: None,
                author: None,
                author_image: None,
            }],
        }],
    }
}
