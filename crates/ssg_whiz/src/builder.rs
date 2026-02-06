use std::error::Error;
use std::future::Future;
use std::io;
use std::path::Path;
use std::pin::Pin;

use crate::{
    generate_website, set_navigation_links, set_site_header, set_site_meta, SiteConfig, SitePage,
    WebsiteInput,
};
use crate::summaries::{BlogSummary, DocumentSite, PagesSummary, Summary};

#[derive(Clone, Debug)]
pub struct SiteBuilder {
    config: SiteConfig,
    blog: Option<BlogSummary>,
    pages: Option<PagesSummary>,
    documents: Vec<DocumentSite>,
    static_pages: Option<StaticPagesFn>,
}

type StaticPagesFuture = Pin<Box<dyn Future<Output = Vec<SitePage>> + Send>>;
type StaticPagesFn = Box<dyn FnOnce() -> StaticPagesFuture + Send>;

impl SiteBuilder {
    pub fn new(config: SiteConfig) -> Self {
        Self {
            config,
            blog: None,
            pages: None,
            documents: Vec::new(),
            static_pages: None,
        }
    }

    pub fn blog(mut self, summary: BlogSummary) -> Self {
        self.blog = Some(summary);
        self
    }

    pub fn pages(mut self, summary: PagesSummary) -> Self {
        self.pages = Some(summary);
        self
    }

    pub fn document(mut self, document: DocumentSite) -> Self {
        self.documents.push(document);
        self
    }

    pub fn documents(mut self, documents: Vec<DocumentSite>) -> Self {
        self.documents.extend(documents);
        self
    }

    pub fn static_pages<F, Fut>(mut self, static_pages: F) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Vec<SitePage>> + Send + 'static,
    {
        let boxed: StaticPagesFn = Box::new(move || Box::pin(static_pages()));
        self.static_pages = Some(boxed);
        self
    }

    pub async fn build(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let static_pages = require(self.static_pages, "static pages")?;

        set_navigation_links(self.config.navigation_links.clone());
        set_site_meta(self.config.site_meta.clone());
        set_site_header(self.config.site_header);

        let dist_dir = self.config.dist_dir.clone();
        std::fs::create_dir_all(&dist_dir)?;

        copy_assets_dir(&dist_dir)?;

        if let Some(summary) = &self.blog {
            copy_summary_assets(&dist_dir, summary)?;
        }
        if let Some(summary) = &self.pages {
            copy_summary_assets(&dist_dir, summary)?;
        }
        for document in &self.documents {
            copy_summary_assets(&dist_dir, &document.summary)?;
        }

        let input = WebsiteInput {
            blog: require(self.blog, "blog summary")?,
            documents: self.documents,
            pages: require(self.pages, "pages summary")?,
            static_pages: static_pages().await,
        };

        generate_website(self.config, input).await
    }
}

fn require<T>(
    value: Option<T>,
    name: &'static str,
) -> Result<T, Box<dyn Error + Send + Sync>> {
    value.ok_or_else(|| format!("SiteBuilder missing {name}").into())
}

fn copy_assets_dir(dist_dir: &Path) -> io::Result<()> {
    let src = Path::new("assets");
    if src.exists() {
        copy_folder(src, dist_dir)?;
    }
    Ok(())
}

fn copy_summary_assets(dist_dir: &Path, summary: &Summary) -> io::Result<()> {
    let src = Path::new("content").join(summary.source_folder);
    if src.exists() {
        let dst = dist_dir.join(summary.source_folder);
        copy_folder(&src, &dst)?;
    }
    Ok(())
}

fn copy_folder(src: &Path, dst: &Path) -> io::Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_folder(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
