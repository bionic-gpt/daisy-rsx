pub mod blog_summary;
pub mod docs_summary;
pub mod generator;
pub mod mcp_specs;
pub mod pages;
pub mod pages_summary;
pub mod ui_links;

pub use ssg_whiz::render;

use std::{fs, net::SocketAddr, path::Path};
use ssg_whiz::{generate_website, set_navigation_links, set_site_meta, WebsiteInput, SiteConfig};
use ssg_whiz::summaries::DocumentSite;

pub mod routes {
    pub const SIGN_IN_UP: &str = "https://app.deploy-mcp.com";

    pub mod blog {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/blog/")]
        pub struct Index {}
    }

    pub mod product {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/chat/")]
        pub struct Chat {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/assistants/")]
        pub struct Assistants {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/integrations/")]
        pub struct Integrations {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/automations/")]
        pub struct Automations {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/developers/")]
        pub struct Developers {}
    }

    pub mod solutions {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/solutions/education/")]
        pub struct Education {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/solutions/support/")]
        pub struct Support {}
    }

    pub mod marketing {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/")]
        pub struct Index {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/enterprise/")]
        pub struct Enterprise {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/terms/")]
        pub struct Terms {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/privacy/")]
        pub struct Privacy {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/pricing/")]
        pub struct Pricing {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/mcp-servers/")]
        pub struct McpServers {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/contact/")]
        pub struct Contact {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/services/")]
        pub struct ServicesPage {}
    }

    pub mod docs {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/docs/")]
        pub struct Index {}
    }

    pub mod mcp_servers {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/mcp-servers/:slug/")]
        pub struct Detail {
            pub slug: String,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    fs::create_dir_all("dist").expect("Couldn't create dist folder");
    set_navigation_links(ui_links::navigation_links());
    set_site_meta(ui_links::site_meta());

    let docs_summary = docs_summary::summary();
    let blog_summary = blog_summary::summary();
    let pages_summary = pages_summary::summary();

    copy_summary_assets(&docs_summary);
    copy_summary_assets(&blog_summary);
    copy_summary_assets(&pages_summary);

    let src = Path::new("assets");
    let dst = Path::new("dist");
    generator::copy_folder(src, dst).expect("Couldn't copy assets");

    let run_server = std::env::var("DO_NOT_RUN_SERVER").is_err();
    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server,
        addr: SocketAddr::from(([0, 0, 0, 0], 8081)),
        live_reload: true,
        navigation_links: ui_links::navigation_links(),
        footer_links: ui_links::footer_links(),
        site_meta: ui_links::site_meta(),
    };

    let input = WebsiteInput {
        blog: blog_summary,
        documents: vec![DocumentSite {
            summary: docs_summary,
            section: daisy_rsx::marketing::navigation::Section::Docs,
        }],
        pages: pages_summary,
        static_pages: generator::generate_static_pages().await,
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
