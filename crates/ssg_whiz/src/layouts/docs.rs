use dioxus::prelude::*;

use super::layout::Layout;
use crate::{
    Section, page_permalink,
    summaries::{Category, PageSummary, Summary},
};

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
                class: "flex-1",

                div {
                    class: "relative flex flex-row",
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
            ul { class: "menu menu-sm gap-1",
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
            class: "fixed top-2.5 bottom-0 left-[-100%] z-40 hidden h-[calc(100vh-108px)] w-[420px] shrink-0 overflow-y-auto border-r border-base-300 bg-base-100 lg:sticky lg:left-0 lg:block",
            "data-scroll-key": scroll_key,
            nav {
                class: "px-5 pt-12 pb-5",
                for category in &summary.categories {
                    p {
                        class: format!(
                            "mb-2 font-semibold{}",
                            if category.name.contains("Coming Soon") {
                                " opacity-60"
                            } else {
                                ""
                            }
                        ),
                        "{category.name}"
                    }
                    ul {
                        class: "menu mb-6 p-0",
                        for page in &category.pages {
                            li {
                                a {
                                    class: format!(
                                        "{}{}",
                                        if page.folder == active_folder && !category.name.contains("Coming Soon") {
                                            "active"
                                        } else {
                                            ""
                                        },
                                        if category.name.contains("Coming Soon") {
                                            " pointer-events-none cursor-not-allowed opacity-50"
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
            class: "h-[calc(100vh-108px)] w-full px-5 pt-12 pb-5 lg:overflow-y-auto",
            div {
                class: "mb-12",
                article {
                    class: "mx-auto max-w-3xl",
                    div {
                        class: "prose prose-slate max-w-none prose-pre:overflow-x-auto prose-pre:rounded-xl prose-pre:bg-slate-100 prose-code:font-mono prose-img:max-w-full",
                        dangerous_inner_html: "{content}"
                    }
                }
            }
        }
    }
}
