use std::net::SocketAddr;

use ssg_whiz::{SiteBuilder, SiteConfig};

use bionic_gpt::{
    architect_course_summary, blog_summary, docs_summary, generator, pages_summary,
    ui_links::{footer_links, navigation_links},
    site_header::site_header,
};
use daisy_rsx::marketing::navigation::Section;
use ssg_whiz::summaries::DocumentSite;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let docs_summary = docs_summary::summary();
    let architect_summary = architect_course_summary::summary();
    let blog_summary = blog_summary::summary();
    let pages_summary = pages_summary::summary();

    let run_server = std::env::var("DO_NOT_RUN_SERVER").is_err();
    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server,
        addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
        live_reload: true,
        navigation_links: navigation_links(),
        footer_links: footer_links(),
        site_meta: bionic_gpt::ui_links::site_meta(),
        site_header: Some(site_header),
    };

    SiteBuilder::new(config)
        .blog(blog_summary)
        .pages(pages_summary)
        .documents(vec![
            DocumentSite {
                summary: docs_summary,
                section: Section::Docs,
            },
            DocumentSite {
                summary: architect_summary,
                section: Section::ArchitectCourse,
            },
        ])
        .static_pages(generator::generate_static_pages)
        .build()
        .await
        .expect("Failed to generate website");
}
