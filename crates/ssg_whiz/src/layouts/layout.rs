#![allow(non_snake_case)]

use crate::{absolute_url, navigation_links, site_assets, site_meta};
use daisy_rsx::marketing::navigation::{Navigation, Section};
use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    title: String,
    description: String,
    image: Option<String>,
    url: Option<String>,
    children: Element,
    mobile_menu: Option<Element>,
    section: Section,
}

pub fn Layout(props: LayoutProps) -> Element {
    let meta = site_meta();
    let assets = site_assets();
    let page_url = props.url.clone().unwrap_or_else(|| meta.base_url.clone());
    let page_url = absolute_url(&page_url);
    let image = props.image.unwrap_or("/open-graph.png".to_string());
    let image_meta = absolute_url(&image);
    let lightbox_css = r#"
        .image-lightbox {
            position: fixed;
            inset: 0;
            display: none;
            align-items: center;
            justify-content: center;
            background: rgba(0, 0, 0, 0.92);
            z-index: 9999;
            cursor: zoom-out;
            padding: 2rem;
        }
        .image-lightbox.is-open {
            display: flex;
        }
        .image-lightbox img {
            max-width: min(96vw, 1800px);
            max-height: 92vh;
            width: auto;
            height: auto;
            object-fit: contain;
            box-shadow: 0 10px 35px rgba(0, 0, 0, 0.45);
            border-radius: 8px;
        }
        article img,
        .prose img {
            cursor: zoom-in;
        }
        .code-lightbox {
            position: fixed;
            inset: 0;
            display: none;
            align-items: center;
            justify-content: center;
            background: rgba(0, 0, 0, 0.92);
            z-index: 10000;
            padding: 1rem;
        }
        .code-lightbox.is-open {
            display: flex;
        }
        .code-lightbox__panel {
            width: min(96vw, 1900px);
            max-height: 94vh;
            display: flex;
            flex-direction: column;
            --code-lightbox-font-scale: 1;
        }
        .code-lightbox__toolbar {
            display: flex;
            justify-content: flex-end;
            gap: 0.5rem;
            margin-bottom: 0.5rem;
        }
        .code-lightbox__button {
            background: rgba(16, 24, 40, 0.92);
            color: #fff;
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 8px;
            padding: 0.35rem 0.75rem;
            font-weight: 600;
            cursor: pointer;
        }
        .code-lightbox__content {
            overflow: auto;
            border-radius: 10px;
            box-shadow: 0 10px 35px rgba(0, 0, 0, 0.45);
        }
        .code-lightbox__content pre {
            margin: 0;
            padding: 1.25rem;
            max-height: 86vh;
            overflow: auto;
            font-size: calc(clamp(15px, 1.35vw, 24px) * var(--code-lightbox-font-scale));
            line-height: 1.45;
        }
        article pre,
        .prose pre {
            cursor: zoom-in;
        }
    "#;
    let lightbox_js = r#"
        (function () {
          if (window.__contentLightboxReady) return;
          window.__contentLightboxReady = true;

          const imageOverlay = document.createElement("div");
          imageOverlay.className = "image-lightbox";
          imageOverlay.setAttribute("aria-hidden", "true");

          const overlayImg = document.createElement("img");
          overlayImg.alt = "";
          imageOverlay.appendChild(overlayImg);
          document.body.appendChild(imageOverlay);

          const codeOverlay = document.createElement("div");
          codeOverlay.className = "code-lightbox";
          codeOverlay.setAttribute("aria-hidden", "true");

          const codePanel = document.createElement("div");
          codePanel.className = "code-lightbox__panel";

          const codeToolbar = document.createElement("div");
          codeToolbar.className = "code-lightbox__toolbar";

          const zoomOutButton = document.createElement("button");
          zoomOutButton.type = "button";
          zoomOutButton.className = "code-lightbox__button";
          zoomOutButton.textContent = "A-";

          const zoomInButton = document.createElement("button");
          zoomInButton.type = "button";
          zoomInButton.className = "code-lightbox__button";
          zoomInButton.textContent = "A+";

          const closeButton = document.createElement("button");
          closeButton.type = "button";
          closeButton.className = "code-lightbox__button";
          closeButton.textContent = "Close";

          const codeContent = document.createElement("div");
          codeContent.className = "code-lightbox__content";

          codeToolbar.appendChild(zoomOutButton);
          codeToolbar.appendChild(zoomInButton);
          codeToolbar.appendChild(closeButton);
          codePanel.appendChild(codeToolbar);
          codePanel.appendChild(codeContent);
          codeOverlay.appendChild(codePanel);
          document.body.appendChild(codeOverlay);

          let codeZoom = 1;

          function setCodeZoom(nextZoom) {
            codeZoom = Math.max(0.75, Math.min(2.25, nextZoom));
            codePanel.style.setProperty("--code-lightbox-font-scale", String(codeZoom));
          }

          function closeImageLightbox() {
            imageOverlay.classList.remove("is-open");
            imageOverlay.setAttribute("aria-hidden", "true");
            overlayImg.removeAttribute("src");
            if (!codeOverlay.classList.contains("is-open")) {
              document.body.style.overflow = "";
            }
          }

          function openImageLightbox(src, alt) {
            overlayImg.src = src;
            overlayImg.alt = alt || "";
            imageOverlay.classList.add("is-open");
            imageOverlay.setAttribute("aria-hidden", "false");
            document.body.style.overflow = "hidden";
          }

          function closeCodeLightbox() {
            codeOverlay.classList.remove("is-open");
            codeOverlay.setAttribute("aria-hidden", "true");
            codeContent.replaceChildren();
            setCodeZoom(1);
            if (!imageOverlay.classList.contains("is-open")) {
              document.body.style.overflow = "";
            }
          }

          function openCodeLightbox(preNode) {
            const clone = preNode.cloneNode(true);
            clone.querySelectorAll(".code-copy-btn").forEach((node) => node.remove());
            codeContent.replaceChildren(clone);
            setCodeZoom(1.25);
            codeOverlay.classList.add("is-open");
            codeOverlay.setAttribute("aria-hidden", "false");
            document.body.style.overflow = "hidden";
          }

          zoomInButton.addEventListener("click", function (event) {
            event.stopPropagation();
            setCodeZoom(codeZoom + 0.15);
          });

          zoomOutButton.addEventListener("click", function (event) {
            event.stopPropagation();
            setCodeZoom(codeZoom - 0.15);
          });

          closeButton.addEventListener("click", function (event) {
            event.stopPropagation();
            closeCodeLightbox();
          });

          document.addEventListener("click", function (event) {
            const target = event.target;
            if (!(target instanceof Element)) return;

            if (target.closest(".code-copy-btn")) return;

            const pre = target.closest("pre");
            if (pre && pre.closest("article, .prose") && !pre.closest(".code-lightbox")) {
              openCodeLightbox(pre);
              return;
            }

            if (!(target instanceof HTMLImageElement)) return;
            if (!target.closest("article, .prose")) return;
            if (target.closest(".image-lightbox")) return;
            if (!target.src) return;
            openImageLightbox(target.src, target.alt);
          });

          imageOverlay.addEventListener("click", function () {
            closeImageLightbox();
          });

          codeOverlay.addEventListener("click", function (event) {
            if (event.target === codeOverlay) {
              closeCodeLightbox();
            }
          });

          codePanel.addEventListener("click", function (event) {
            event.stopPropagation();
          });

          document.addEventListener("keydown", function (event) {
            if (event.key === "Escape" && imageOverlay.classList.contains("is-open")) {
              closeImageLightbox();
            }
            if (event.key === "Escape" && codeOverlay.classList.contains("is-open")) {
              closeCodeLightbox();
            }
          });
        })();
    "#;
    rsx!(
        head {
            title {
                "{props.title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
            meta {
                name: "description",
                content: "{props.description}"
            }

            // The four required Open Graph tags for every page are og:title, og:type, og:image, and og:url.
            meta { property: "og:title", content: "{props.title}" }
            meta { property: "og:description", content: "{props.description}" }
            meta { property: "og:type", content: "article" }
            meta { property: "og:site_name", content: "{meta.site_name}" }
            meta { property: "og:url", content: "{page_url}" }
            meta { property: "og:image", content: "{image_meta}" }
            meta { property: "twitter:image", content: "{image_meta}" }

            for href in &assets.stylesheets {
                link {
                    rel: "stylesheet",
                    href: "{href}",
                    "type": "text/css"
                }
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "/favicon.svg"
            }
            for script_asset in &assets.head_scripts {
                script {
                    async: script_asset.async_load,
                    "data-goatcounter": "{script_asset.data_goatcounter.as_deref().unwrap_or(\"\")}",
                    integrity: "{script_asset.integrity.as_deref().unwrap_or(\"\")}",
                    r#type: "{script_asset.script_type.as_deref().unwrap_or(\"\")}",
                    src: "{script_asset.src}"
                }
            }
            style { "{lightbox_css}" }
        }
        body {
            //WebinarHeader {}
            Navigation {
                mobile_menu: props.mobile_menu,
                section: props.section,
                model: navigation_links().clone(),
                brand: Some(meta.brand_name.clone()),
                site_header: crate::site_header_factory().map(|factory| factory())
            }
            {props.children}
            for script_asset in &assets.body_scripts {
                script {
                    async: script_asset.async_load,
                    integrity: "{script_asset.integrity.as_deref().unwrap_or(\"\")}",
                    r#type: "{script_asset.script_type.as_deref().unwrap_or(\"\")}",
                    src: "{script_asset.src}"
                }
            }
            script {
                dangerous_inner_html: lightbox_js
            }
        }
    )
}
