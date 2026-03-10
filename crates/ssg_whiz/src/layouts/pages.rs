use super::layout::Layout;
use crate::{page_permalink, summaries::PageSummary, Footer, FooterLinks, Section};
use dioxus::prelude::*;

#[component]
pub fn MarkdownPage(post: PageSummary, footer_links: FooterLinks) -> Element {
    let content = crate::markdown::markdown_to_html(post.markdown);
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            url: Some(page_permalink(post.folder)),
            section: Section::None,
            article {
                class: "markdown-page",
                div {
                    class: "markdown-page__content",
                    dangerous_inner_html: "{content}"
                }
            }
            Footer {
                links: footer_links
            }
        }
    }
}
