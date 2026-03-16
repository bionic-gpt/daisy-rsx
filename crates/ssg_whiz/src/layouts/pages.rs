use super::layout::Layout;
use crate::{Footer, FooterLinks, Section, page_permalink, summaries::PageSummary};
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
                    class: "prose prose-slate max-w-none prose-pre:overflow-x-auto prose-pre:rounded-xl prose-pre:bg-slate-100 prose-code:font-mono prose-img:max-w-full",
                    dangerous_inner_html: "{content}"
                }
            }
            Footer {
                links: footer_links
            }
        }
    }
}
