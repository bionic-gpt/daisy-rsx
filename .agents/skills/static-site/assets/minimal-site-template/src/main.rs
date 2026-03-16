pub mod blog_summary;
pub mod docs_summary;
pub mod generator;
pub mod pages;
pub mod pages_summary;
pub mod ui_links;

use std::net::SocketAddr;

use ssg_whiz::{DocumentSite, ScriptAsset, Section, SiteAssets, SiteBuilder, SiteConfig};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server: std::env::var("DO_NOT_RUN_SERVER").is_err(),
        addr: SocketAddr::from(([0, 0, 0, 0], __SITE_PORT__)),
        live_reload: true,
        navigation_links: ui_links::navigation_links(),
        footer_links: ui_links::footer_links(),
        site_meta: ui_links::site_meta(),
        site_header: None,
        site_assets: SiteAssets {
            stylesheets: vec!["/tailwind.css".to_string()],
            head_scripts: vec![
                ScriptAsset::new("/goat-counter.js"),
                ScriptAsset::new("/copy-paste.js"),
                ScriptAsset {
                    src: "https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"
                        .to_string(),
                    script_type: Some("module".to_string()),
                    async_load: false,
                    integrity: None,
                    data_goatcounter: None,
                },
            ],
            body_scripts: vec![ScriptAsset {
                src: "https://instant.page/5.2.0".to_string(),
                script_type: Some("module".to_string()),
                async_load: false,
                integrity: Some(
                    "sha384-jnZyxPjiipYXnSU0ygqeac2q7CVYMbh84q0uHVRRxEtvFPiQYbXWUorga2aqZJ0z"
                        .to_string(),
                ),
                data_goatcounter: None,
            }],
            head_inline_scripts: vec![],
            body_inline_scripts: vec![],
        },
        ..SiteConfig::default()
    };

    SiteBuilder::new(config)
        .blog(blog_summary::summary())
        .pages(pages_summary::summary())
        .document(DocumentSite {
            summary: docs_summary::summary(),
            section: Section::Docs,
        })
        .static_pages(generator::generate_static_pages)
        .build()
        .await
        .expect("Failed to generate website");
}
