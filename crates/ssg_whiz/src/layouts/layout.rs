#![allow(non_snake_case)]

use crate::{absolute_url, navigation_links, site_assets, site_meta, Navigation, Section};
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
        }
    )
}
