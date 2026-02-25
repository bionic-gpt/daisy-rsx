use std::error::Error;
use std::future::Future;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Command;
use std::time::SystemTime;

use image::{ImageFormat, ImageReader, imageops::FilterType};

use crate::{
    generate_website, set_navigation_links, set_site_header, set_site_meta, SiteConfig, SitePage,
    WebsiteInput,
};
use crate::summaries::{BlogSummary, DocumentSite, PagesSummary, Summary};

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
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Vec<SitePage>> + Send + 'static,
    {
        let boxed: StaticPagesFn = Box::new(move || Box::pin(static_pages()));
        self.static_pages = Some(boxed);
        self
    }

    pub async fn build(self) -> Result<(), Box<dyn Error + Send + Sync>> {
        tracing::info!("site build: starting");
        let static_pages = require(self.static_pages, "static pages")?;

        set_navigation_links(self.config.navigation_links.clone());
        set_site_meta(self.config.site_meta.clone());
        set_site_header(self.config.site_header);

        let dist_dir = self.config.dist_dir.clone();
        std::fs::create_dir_all(&dist_dir)?;
        tracing::info!("site build: dist dir ready at {}", dist_dir.display());

        copy_assets_dir(&dist_dir)?;
        tracing::info!("site build: copied shared assets");

        if let Some(summary) = &self.blog {
            copy_summary_assets(&dist_dir, summary)?;
        }
        if let Some(summary) = &self.pages {
            copy_summary_assets(&dist_dir, summary)?;
        }
        for document in &self.documents {
            copy_summary_assets(&dist_dir, &document.summary)?;
        }
        tracing::info!("site build: copied summary content");

        let input = WebsiteInput {
            blog: require(self.blog, "blog summary")?,
            documents: self.documents,
            pages: require(self.pages, "pages summary")?,
            static_pages: static_pages().await,
        };

        let result = generate_website(self.config, input).await;
        if result.is_ok() {
            tracing::info!("site build: complete");
        }
        result
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
        tracing::info!(
            "site build: copying summary assets from {}",
            src.display()
        );
        let dst = dist_dir.join(summary.source_folder);
        if summary.source_folder == "blog" {
            let stats = copy_blog_folder_with_resizing(&src, &dst)?;
            tracing::info!(
                "site build: blog image variants regenerated: {}, skipped: {}",
                stats.regenerated,
                stats.skipped
            );
        } else {
            copy_folder(&src, &dst)?;
        }
        tracing::info!(
            "site build: finished copying summary assets to {}",
            dst.display()
        );
        let rendered = render_d2_diagrams(&dst)?;
        if rendered > 0 {
            tracing::info!(
                "site build: rendered {} d2 diagram(s) under {}",
                rendered,
                dst.display()
            );
        } else {
            tracing::info!("site build: no d2 diagrams found under {}", dst.display());
        }
    }
    Ok(())
}

#[derive(Default)]
struct BlogImageStats {
    regenerated: usize,
    skipped: usize,
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

fn copy_blog_folder_with_resizing(src: &Path, dst: &Path) -> io::Result<BlogImageStats> {
    let mut stats = BlogImageStats::default();
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            let nested = copy_blog_folder_with_resizing(&src_path, &dst_path)?;
            stats.regenerated += nested.regenerated;
            stats.skipped += nested.skipped;
        } else if is_resizable_blog_image(&src_path) {
            if copy_and_generate_blog_variants(&src_path, &dst_path)? {
                tracing::info!("site build: resized blog image {}", src_path.display());
                stats.regenerated += 1;
            } else {
                stats.skipped += 1;
            }
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(stats)
}

fn is_resizable_blog_image(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("png") | Some("jpg") | Some("jpeg") | Some("webp")
    )
}

fn image_format_for(path: &Path) -> io::Result<ImageFormat> {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => Ok(ImageFormat::Png),
        Some("jpg") | Some("jpeg") => Ok(ImageFormat::Jpeg),
        Some("webp") => Ok(ImageFormat::WebP),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported image format for {}", path.display()),
        )),
    }
}

fn copy_and_generate_blog_variants(src_path: &Path, dst_path: &Path) -> io::Result<bool> {
    let src_modified = std::fs::metadata(src_path)?.modified()?;
    let original_stale = file_is_stale(dst_path, src_modified);
    let mut stale_variants = Vec::new();

    for (width, height) in [(384u32, 216u32), (768u32, 432u32)] {
        let variant_path = image_variant_path(dst_path, width, height)?;
        let stale = file_is_stale(&variant_path, src_modified);
        stale_variants.push((variant_path, width, height, stale));
    }

    let any_variant_stale = stale_variants.iter().any(|(_, _, _, stale)| *stale);
    if !original_stale && !any_variant_stale {
        return Ok(false);
    }

    if original_stale {
        std::fs::copy(src_path, dst_path)?;
    }

    if any_variant_stale {
        let image = ImageReader::open(src_path)?
            .decode()
            .map_err(|err| io::Error::other(format!("failed to decode {}: {err}", src_path.display())))?;

        for (variant_path, width, height, stale) in stale_variants {
            if !stale {
                continue;
            }
            let resized = image.resize_to_fill(width, height, FilterType::Lanczos3);
            let format = image_format_for(&variant_path)?;
            resized.save_with_format(&variant_path, format).map_err(|err| {
                io::Error::other(format!("failed to save {}: {err}", variant_path.display()))
            })?;
        }
    }

    Ok(true)
}

fn file_is_stale(path: &Path, src_modified: SystemTime) -> bool {
    let modified = match std::fs::metadata(path).and_then(|m| m.modified()) {
        Ok(modified) => modified,
        Err(_) => return true,
    };

    modified < src_modified
}

fn image_variant_path(path: &Path, width: u32, height: u32) -> io::Result<PathBuf> {
    let stem = path
        .file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing file stem"))?;
    let ext = path
        .extension()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing extension"))?;

    let mut filename = stem.to_os_string();
    filename.push(format!("-{width}x{height}"));
    filename.push(".");
    filename.push(ext);

    Ok(path.with_file_name(filename))
}

fn render_d2_diagrams(root: &Path) -> io::Result<usize> {
    let mut rendered = 0usize;
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            rendered += render_d2_diagrams(&path)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("d2") {
            render_d2_diagram(&path)?;
            rendered += 1;
        }
    }
    Ok(rendered)
}

fn render_d2_diagram(path: &Path) -> io::Result<()> {
    let output = path.with_extension("svg");
    tracing::info!(
        "site build: rendering d2 {} -> {}",
        path.display(),
        output.display()
    );
    let result = Command::new("d2").arg(path).arg(&output).output();
    let result = match result {
        Ok(result) => result,
        Err(err) => {
            // Default: keep site generation moving even when d2 is not installed.
            // Set SSG_WHIZ_REQUIRE_D2=1 to enforce hard failure (recommended in CI).
            let require_d2 = std::env::var("SSG_WHIZ_REQUIRE_D2")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            if require_d2 {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "failed to run d2 for {}: {}. Install d2 to render diagrams at build time.",
                        path.display(),
                        err
                    ),
                ));
            }

            tracing::warn!(
                "site build: skipping d2 render for {} because d2 is unavailable: {}",
                path.display(),
                err
            );
            return Ok(());
        }
    };

    if result.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&result.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&result.stdout).trim().to_string();
    let detail = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        "d2 exited with a non-zero status".to_string()
    };

    Err(io::Error::other(format!(
        "d2 failed for {}: {}",
        path.display(),
        detail
    )))
}
