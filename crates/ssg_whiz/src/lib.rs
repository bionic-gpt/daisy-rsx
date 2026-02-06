use std::fs;
use std::io;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use axum::Router;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[derive(Clone, Debug)]
pub struct SiteConfig {
    pub dist_dir: PathBuf,
    pub run_server: bool,
    pub addr: SocketAddr,
    pub live_reload: bool,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            dist_dir: PathBuf::from("dist"),
            run_server: true,
            addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
            live_reload: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Page {
    pub path: String,
    pub html: String,
}

pub async fn generate_site(
    config: SiteConfig,
    pages: Vec<Page>,
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

fn write_pages(dist_dir: &Path, pages: Vec<Page>) -> io::Result<()> {
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
