pub mod blog_summary;
pub mod generator;
pub mod pages;
pub mod pages_summary;
pub mod ui_links;

use std::net::SocketAddr;

use ssg_whiz::{SiteBuilder, SiteConfig};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server: std::env::var("DO_NOT_RUN_SERVER").is_err(),
        addr: SocketAddr::from(([0, 0, 0, 0], 8082)),
        live_reload: true,
        navigation_links: ui_links::navigation_links(),
        footer_links: ui_links::footer_links(),
        site_meta: ui_links::site_meta(),
        site_header: None,
    };

    SiteBuilder::new(config)
        .blog(blog_summary::summary())
        .pages(pages_summary::summary())
        .static_pages(generator::generate_static_pages)
        .build()
        .await
        .expect("Failed to generate website");
}
