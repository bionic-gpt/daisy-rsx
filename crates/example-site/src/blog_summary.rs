use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Updates".to_string(),
            pages: vec![
                PageSummary {
                    date: "2026-09-02",
                    title: "Second Example Article",
                    description: "A second starter post for checking the blog index layout.",
                    folder: "blog/second-example-article",
                    markdown: include_str!("../content/blog/second-example-article/index.md"),
                    image: Some("/blog/second-example-article/header.svg"),
                    open_graph_image: Some("/blog/second-example-article/open-graph.svg"),
                    author_image: Some("/blog-authors/ian.png"),
                    author: Some("Ian"),
                },
                PageSummary {
                    date: "2026-03-09",
                    title: "Hello from Example Site",
                    description: "The first post for your new static site.",
                    folder: "blog/hello-world",
                    markdown: include_str!("../content/blog/hello-world/index.md"),
                    image: Some("/blog/hello-world/header.png"),
                    open_graph_image: Some("/blog/hello-world/open-graph.svg"),
                    author_image: Some("/blog-authors/ian.png"),
                    author: Some("Ian"),
                },
            ],
        }],
    }
}
