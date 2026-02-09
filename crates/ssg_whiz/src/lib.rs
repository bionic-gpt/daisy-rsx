use std::fs;
use std::io;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use axum::Router;
use daisy_rsx::marketing::{
    footer::FooterLinks,
    navigation::{NavigationModel, Section},
    site_header::SiteHeader,
};
use dioxus::prelude::*;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use layouts::{BlogList, BlogPost, Document, MarkdownPage};
use summaries::{BlogSummary, DocumentSite, PagesSummary, Summary};

pub mod layouts;
pub mod markdown;
pub mod summaries;
pub mod builder;

pub use builder::SiteBuilder;

static NAV_LINKS: OnceLock<NavigationModel> = OnceLock::new();
static SITE_META: OnceLock<SiteMeta> = OnceLock::new();
static SITE_HEADER_FACTORY: OnceLock<Option<SiteHeaderFactory>> = OnceLock::new();

pub fn set_navigation_links(links: NavigationModel) {
    let _ = NAV_LINKS.set(links);
}

pub(crate) fn navigation_links() -> &'static NavigationModel {
    NAV_LINKS
        .get()
        .expect("ssg_whiz navigation links not set")
}

#[derive(Clone, Debug)]
pub struct SiteMeta {
    pub base_url: String,
    pub site_name: String,
    pub brand_name: String,
    pub goatcounter: String,
}

pub type SiteHeaderFactory = fn() -> SiteHeader;

pub fn set_site_meta(meta: SiteMeta) {
    let _ = SITE_META.set(meta);
}

pub(crate) fn site_meta() -> &'static SiteMeta {
    SITE_META.get().expect("ssg_whiz site meta not set")
}

pub fn set_site_header(factory: Option<SiteHeaderFactory>) {
    let _ = SITE_HEADER_FACTORY.set(factory);
}

pub(crate) fn site_header_factory() -> Option<SiteHeaderFactory> {
    SITE_HEADER_FACTORY.get().cloned().unwrap_or(None)
}

pub fn absolute_url(value: &str) -> String {
    let meta = site_meta();
    let base = meta.base_url.trim_end_matches('/');
    if value.starts_with("http://") || value.starts_with("https://") {
        value.to_string()
    } else if value.starts_with('/') {
        format!("{base}{value}")
    } else {
        let trimmed = value.trim_start_matches('/');
        format!("{base}/{trimmed}")
    }
}

pub fn page_permalink(folder: &str) -> String {
    absolute_url(folder)
}

#[derive(Clone, Debug)]
pub struct SiteConfig {
    pub dist_dir: PathBuf,
    pub run_server: bool,
    pub addr: SocketAddr,
    pub live_reload: bool,
    pub navigation_links: NavigationModel,
    pub footer_links: FooterLinks,
    pub site_meta: SiteMeta,
    pub site_header: Option<SiteHeaderFactory>,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            dist_dir: PathBuf::from("dist"),
            run_server: true,
            addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
            live_reload: true,
            navigation_links: NavigationModel {
                home: "/".to_string(),
                logo_src: None,
                logo_alt: None,
                desktop_left: vec![],
                desktop_right: vec![],
                mobile: vec![],
            },
            footer_links: FooterLinks {
                blog: "/blog".to_string(),
                pricing: "/pricing".to_string(),
                contact: "/contact".to_string(),
                terms: "/terms".to_string(),
                privacy: "/privacy".to_string(),
                about: None,
            },
            site_meta: SiteMeta {
                base_url: "https://bionic-gpt.com".to_string(),
                site_name: "Bionic GPT".to_string(),
                brand_name: "Bionic".to_string(),
                goatcounter: "https://bionicgpt.goatcounter.com/count".to_string(),
            },
            site_header: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SitePage {
    pub path: String,
    pub html: String,
}

#[derive(Clone, Debug)]
pub struct WebsiteInput {
    pub blog: BlogSummary,
    pub documents: Vec<DocumentSite>,
    pub pages: PagesSummary,
    pub static_pages: Vec<SitePage>,
}

pub async fn generate_website(
    config: SiteConfig,
    input: WebsiteInput,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    set_navigation_links(config.navigation_links.clone());
    set_site_meta(config.site_meta.clone());
    set_site_header(config.site_header);

    let mut pages = input.static_pages;
    pages.extend(render_blog_posts(&input.blog, config.footer_links.clone()));
    pages.push(render_blog_list(&input.blog, config.footer_links.clone()));

    for doc_site in input.documents {
        pages.extend(render_document_site(&doc_site.summary, doc_site.section));
    }

    pages.extend(render_pages_summary(&input.pages, config.footer_links.clone()));

    generate_site(config, pages).await
}

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

pub async fn generate_site(
    config: SiteConfig,
    pages: Vec<SitePage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fs::create_dir_all(&config.dist_dir)?;

    write_pages(&config.dist_dir, pages)?;

    if config.run_server {
        let app = if config.live_reload {
            Router::new()
                .fallback_service(ServeDir::new(&config.dist_dir))
                .layer(LiveReloadLayer::new())
        } else {
            Router::new().fallback_service(ServeDir::new(&config.dist_dir))
        };

        let listener = tokio::net::TcpListener::bind(&config.addr).await?;
        tracing::info!("listening on http://{}", &config.addr);
        axum::serve(listener, app.into_make_service()).await?;
    }

    Ok(())
}

fn write_pages(dist_dir: &Path, pages: Vec<SitePage>) -> io::Result<()> {
    for page in pages {
        let dir = normalized_page_dir(dist_dir, &page.path);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("index.html"), page.html)?;
    }

    Ok(())
}

fn normalized_page_dir(dist_dir: &Path, path: &str) -> PathBuf {
    let trimmed = path.trim_matches('/');
    if trimmed.is_empty() {
        dist_dir.to_path_buf()
    } else {
        dist_dir.join(trimmed)
    }
}

fn render_blog_posts(summary: &BlogSummary, footer_links: FooterLinks) -> Vec<SitePage> {
    let mut pages = Vec::new();

    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                BlogPost {
                    post: *page,
                    footer_links: footer_links.clone()
                }
            };

            let html = render(page_ele);
            pages.push(SitePage {
                path: page.folder.to_string(),
                html,
            });
        }
    }

    pages
}

fn render_blog_list(summary: &BlogSummary, footer_links: FooterLinks) -> SitePage {
    let page_ele = rsx! {
        BlogList {
            summary: summary.clone(),
            footer_links
        }
    };

    let html = render(page_ele);
    SitePage {
        path: "blog".to_string(),
        html,
    }
}

fn render_document_site(summary: &Summary, section: Section) -> Vec<SitePage> {
    let mut pages = Vec::new();

    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                Document {
                    summary: summary.clone(),
                    category: category.clone(),
                    doc: *page,
                    current_section: section.clone(),
                }
            };

            let html = render(page_ele);
            pages.push(SitePage {
                path: page.folder.to_string(),
                html,
            });
        }
    }

    pages
}

fn render_pages_summary(summary: &PagesSummary, footer_links: FooterLinks) -> Vec<SitePage> {
    let mut pages = Vec::new();

    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                MarkdownPage {
                    post: *page,
                    footer_links: footer_links.clone()
                }
            };

            let html = render(page_ele);
            pages.push(SitePage {
                path: page.folder.to_string(),
                html,
            });
        }
    }

    pages
}
