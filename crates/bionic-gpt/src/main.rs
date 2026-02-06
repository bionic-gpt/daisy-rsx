use std::{fs, net::SocketAddr, path::Path};

use ssg_whiz::{
    generate_website, set_navigation_links, SiteConfig, WebsiteInput,
};

use bionic_gpt::{
    architect_course_summary, blog_summary, docs_summary, generator, pages_summary,
    ui_links::{footer_links, navigation_links},
};
use daisy_rsx::marketing::navigation::Section;
use ssg_whiz::summaries::DocumentSite;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    fs::create_dir_all("dist").expect("Couldn't create dist folder");
    set_navigation_links(navigation_links());

    let docs_summary = docs_summary::summary();
    let architect_summary = architect_course_summary::summary();
    let blog_summary = blog_summary::summary();
    let pages_summary = pages_summary::summary();

    copy_summary_assets(&docs_summary);
    copy_summary_assets(&architect_summary);
    copy_summary_assets(&blog_summary);

    let static_pages = generator::generate_static_pages().await;

    let src = Path::new("assets");
    let dst = Path::new("dist");
    generator::copy_folder(src, dst).expect("Couldn't copy folder");

    let run_server = std::env::var("DO_NOT_RUN_SERVER").is_err();
    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server,
        addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
        live_reload: true,
        navigation_links: navigation_links(),
        footer_links: footer_links(),
    };

    let input = WebsiteInput {
        blog: blog_summary,
        documents: vec![
            DocumentSite {
                summary: docs_summary,
                section: Section::Docs,
            },
            DocumentSite {
                summary: architect_summary,
                section: Section::ArchitectCourse,
            },
        ],
        pages: pages_summary,
        static_pages,
    };

    generate_website(config, input)
        .await
        .expect("Failed to generate website");
}

fn copy_summary_assets(summary: &ssg_whiz::summaries::Summary) {
    let src = Path::new("content").join(summary.source_folder);
    let dst = Path::new("dist").join(summary.source_folder);
    generator::copy_folder(&src, &dst).expect("Couldn't copy content folder");
}
