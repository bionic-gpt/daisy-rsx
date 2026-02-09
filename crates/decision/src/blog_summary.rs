use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Updates".to_string(),
            pages: vec![PageSummary {
                date: "2026-02-09",
                title: "Confidential Computing: A Practical Guide",
                description: "How confidential computing protects data in use with TEEs, attestation, and a practical adoption path for sensitive workloads.",
                folder: "blog/confidential-compute",
                markdown: include_str!("../content/blog/confidential-compute/index.md"),
                image: Some("/blog/confidential-compute/confidential-compute.png"),
                author: None,
                author_image: None,
            }],
        }],
    }
}
