use dioxus::prelude::*;

use super::layout::Layout;
use crate::{
    page_permalink,
    summaries::{Category, PageSummary, Summary},
};
use daisy_rsx::marketing::navigation::Section;

#[component]
pub fn Document(
    summary: Summary,
    category: Category,
    doc: PageSummary,
    current_section: Section,
) -> Element {
    rsx! {
        Layout {
            title: "{doc.title}",
            description: "{doc.description}",
            url: Some(page_permalink(doc.folder)),
            section: current_section,
            mobile_menu: rsx! (MobileMenu {
                summary: summary.clone()
            }),
            main {
                class: "docs-page",

                div {
                    class: "docs-page__layout",
                    LeftNav {
                        summary: summary.clone(),
                        active_folder: doc.folder,
                        scroll_key: summary.source_folder,
                    }
                    Content {
                        doc
                    }
                }
                // Preserve sidebar scroll between navigations so the left nav
                // stays at the same position after clicking a link.
                script {
                    dangerous_inner_html: format!(r#"
                        (function() {{
                            const nav = document.querySelector('[data-scroll-key="{key}"]');
                            if (!nav) return;
                            const storageKey = "left-nav-scroll-{key}";
                            const saved = sessionStorage.getItem(storageKey);
                            if (saved) {{
                                nav.scrollTop = parseInt(saved, 10) || 0;
                            }}
                            nav.addEventListener("scroll", function() {{
                                sessionStorage.setItem(storageKey, nav.scrollTop.toString());
                            }}, {{ passive: true }});
                        }})();
                    "#, key = summary.source_folder)
                }
            }
        }
    }
}

#[component]
fn MobileMenu(summary: Summary) -> Element {
    rsx! {
        for category in &summary.categories {
            ul {
                for page in &category.pages {
                    li {
                        a {
                            href: "/{page.folder}",
                            "{page.title}",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LeftNav(summary: Summary, active_folder: &'static str, scroll_key: &'static str) -> Element {
    rsx! {
        div {
            class: "docs-sidebar",
            "data-scroll-key": scroll_key,
            nav {
                class: "docs-sidebar__nav",
                for category in &summary.categories {
                    p {
                        class: format!(
                            "docs-sidebar__category-title{}",
                            if category.name.contains("Coming Soon") {
                                " docs-sidebar__category-title--disabled"
                            } else {
                                ""
                            }
                        ),
                        "{category.name}"
                    }
                    ul {
                        class: "docs-sidebar__page-list",
                        for page in &category.pages {
                            li {
                                class: "docs-sidebar__page-item",
                                a {
                                    class: format!(
                                        "docs-sidebar__page-link{}{}",
                                        if page.folder == active_folder && !category.name.contains("Coming Soon") {
                                            " docs-sidebar__page-link--active"
                                        } else {
                                            ""
                                        },
                                        if category.name.contains("Coming Soon") {
                                            " docs-sidebar__page-link--disabled"
                                        } else {
                                            ""
                                        }
                                    ),
                                    href: "/{page.folder}",
                                    "hx-boost": if category.name.contains("Coming Soon") { "false" } else { "true" },
                                    tabindex: if category.name.contains("Coming Soon") { "-1" } else { "0" },
                                    "{page.title}"
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}

#[component]
fn Content(doc: PageSummary) -> Element {
    let content = crate::markdown::markdown_to_html(doc.markdown);
    rsx! {
        section {
            class: "docs-content",
            div {
                class: "docs-content__inner",
                article {
                    class: "docs-content__article",
                    div {
                        class: "docs-content__body",
                        dangerous_inner_html: "{content}"
                    }
                }
            }
        }
    }
}
