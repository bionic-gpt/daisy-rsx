#![allow(non_snake_case)]

use crate::{absolute_url, navigation_links, site_meta};
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
    let page_url = props
        .url
        .clone()
        .unwrap_or_else(|| meta.base_url.clone());
    let page_url = absolute_url(&page_url);
    let image = props.image.unwrap_or("/open-graph.png".to_string());
    let image_meta = absolute_url(&image);
    let image_lightbox_css = r#"
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
    "#;
    let image_lightbox_js = r#"
        (function () {
          if (window.__imageLightboxReady) return;
          window.__imageLightboxReady = true;

          const overlay = document.createElement("div");
          overlay.className = "image-lightbox";
          overlay.setAttribute("aria-hidden", "true");

          const overlayImg = document.createElement("img");
          overlayImg.alt = "";
          overlay.appendChild(overlayImg);
          document.body.appendChild(overlay);

          function closeLightbox() {
            overlay.classList.remove("is-open");
            overlay.setAttribute("aria-hidden", "true");
            overlayImg.removeAttribute("src");
            document.body.style.overflow = "";
          }

          function openLightbox(src, alt) {
            overlayImg.src = src;
            overlayImg.alt = alt || "";
            overlay.classList.add("is-open");
            overlay.setAttribute("aria-hidden", "false");
            document.body.style.overflow = "hidden";
          }

          document.addEventListener("click", function (event) {
            const target = event.target;
            if (!(target instanceof HTMLImageElement)) return;
            if (!target.closest("article, .prose")) return;
            if (target.closest(".image-lightbox")) return;
            if (!target.src) return;
            openLightbox(target.src, target.alt);
          });

          overlay.addEventListener("click", function () {
            closeLightbox();
          });

          document.addEventListener("keydown", function (event) {
            if (event.key === "Escape" && overlay.classList.contains("is-open")) {
              closeLightbox();
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

            link {
                rel: "stylesheet",
                href: "/tailwind.css",
                "type": "text/css"
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "/favicon.svg"
            }
            script {
                "async": "true",
                "data-goatcounter": "{meta.goatcounter}",
                src: "/goat-counter.js"

            }
            script {
                "async": "true",
                src: "/copy-paste.js"

            }
            script {
                "type": "module",
                src: "https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"
            }
            style { "{image_lightbox_css}" }
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
            script {
                src: "https://instant.page/5.2.0",
                type: "module",
                integrity: "sha384-jnZyxPjiipYXnSU0ygqeac2q7CVYMbh84q0uHVRRxEtvFPiQYbXWUorga2aqZJ0z"
            }
            script {
                dangerous_inner_html: image_lightbox_js
            }
        }
    )
}
