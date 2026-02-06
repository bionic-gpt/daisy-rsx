use super::layout::Layout;
use crate::summaries::PageSummary;
use daisy_rsx::marketing::{footer::{Footer, FooterLinks}, navigation::Section};
use dioxus::prelude::*;

#[component]
pub fn MarkdownPage(post: PageSummary, footer_links: FooterLinks) -> Element {
    let content = crate::markdown::markdown_to_html(post.markdown);
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            section: Section::None,
            article {
                class: "mx-auto max-w-2xl p-5",
                div {
                    class: "prose",
                    dangerous_inner_html: "{content}"
                }
            }
            Footer {
                links: footer_links
            }
        }
    }
}
