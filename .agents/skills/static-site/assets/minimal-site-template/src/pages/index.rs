use daisy_rsx::marketing::{footer::Footer, navigation::Section};
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "__SITE_TITLE__".to_string(),
            description: "A starter static site built with ssg_whiz.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "min-h-screen bg-base-100 text-base-content",

                section {
                    id: "hero",
                    class: "hero py-24",
                    div {
                        class: "hero-content text-center",
                        div {
                            class: "max-w-3xl",
                            span { class: "badge badge-outline mb-6", "__SITE_TITLE__" }
                            h1 { class: "text-5xl font-bold", "A minimal site scaffold that already runs." }
                            p {
                                class: "py-6 text-lg opacity-80",
                                "You have a homepage, a blog, static assets, custom JavaScript hooks, and a Cloudflare build script. Replace this copy and ship."
                            }
                            div {
                                class: "flex flex-wrap justify-center gap-3",
                                a { class: "btn btn-primary", href: "/blog", "Read the starter post" }
                                a { class: "btn btn-outline", href: "https://github.com/bionic-gpt/daisy-rsx", "View components" }
                            }
                        }
                    }
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }

                script {
                    dangerous_inner_html: include_str!("index.js")
                }
            }
        }
    );

    ssg_whiz::render(page)
}
