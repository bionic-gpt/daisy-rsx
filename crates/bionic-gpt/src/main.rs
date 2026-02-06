use std::{fs, net::SocketAddr, path::Path};

use ssg_whiz::{generate_site, Page, SiteConfig};

use bionic_gpt::{
    architect_course_summary, blog_summary, docs_summary, generator, pages_summary,
};
use daisy_rsx::marketing::navigation::Section;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    fs::create_dir_all("dist").expect("Couldn't create dist folder");

    let docs_summary = docs_summary::summary();
    let architect_summary = architect_course_summary::summary();
    let blog_summary = blog_summary::summary();
    let pages_summary = pages_summary::summary();

    copy_summary_assets(&docs_summary);
    copy_summary_assets(&architect_summary);
    copy_summary_assets(&blog_summary);

    let mut pages: Vec<Page> = Vec::new();
    pages.extend(generator::generate_marketing().await);
    pages.extend(generator::generate_product().await);
    pages.extend(generator::generate_solutions().await);
    pages.extend(generator::generate_docs(docs_summary, Section::Docs));
    pages.extend(generator::generate_docs(
        architect_summary,
        Section::ArchitectCourse,
    ));
    pages.extend(generator::generate(blog_summary.clone()));
    pages.extend(generator::generate_pages(pages_summary).await);
    pages.extend(generator::generate_blog_list(blog_summary).await);

    let src = Path::new("assets");
    let dst = Path::new("dist");
    generator::copy_folder(src, dst).expect("Couldn't copy folder");

    let run_server = std::env::var("DO_NOT_RUN_SERVER").is_err();
    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server,
        addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
        live_reload: true,
    };

    generate_site(config, pages)
        .await
        .expect("Failed to generate site");
}

fn copy_summary_assets(summary: &generator::Summary) {
    let src = Path::new("content").join(summary.source_folder);
    let dst = Path::new("dist").join(summary.source_folder);
    generator::copy_folder(&src, &dst).expect("Couldn't copy content folder");
}
