use super::layout::Layout;
use crate::{
    ExtraFooter, Footer, FooterLinks, Section, extra_footer, page_permalink,
    summaries::{PageSummary, Summary},
};
use dioxus::prelude::*;

fn image_variant_path(path: &str, width: u32, height: u32) -> String {
    if let Some((base, ext)) = path.rsplit_once('.') {
        format!(
            "/processed/{base}-{width}x{height}.{ext}",
            base = base.trim_start_matches('/')
        )
    } else {
        path.to_string()
    }
}

fn supports_resized_variants(path: &str) -> bool {
    path.rsplit_once('.')
        .map(|(_, ext)| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "webp"
            )
        })
        .unwrap_or(false)
}

#[component]
pub fn BlogPost(post: PageSummary, footer_links: FooterLinks) -> Element {
    let content = crate::markdown::markdown_to_html(post.markdown);
    let extra_footer = extra_footer();
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            url: Some(page_permalink(post.folder)),
            image: post.image.map(|image| image.to_string()),
            section: Section::Blog,
            article {
                class: "mt-24 mx-auto max-w-prose px-5 lg:prose-xl",
                div {
                    class: "",
                    h1 {
                        class: "text-4xl font-extrabold leading-tight md:text-5xl",
                        "{post.title}"
                    }
                    div {
                        class: "my-8 flex flex-row",
                        if let Some(author_image) = post.author_image {
                            img {
                                class: "shrink-0 rounded-full",
                                width: "44",
                                height: "44",
                                src: author_image,
                                alt: "Author"
                            }
                        }
                        div {
                            class: "flex flex-col pl-2",
                            if let Some(author) = post.author {
                                strong {
                                    class: "text-base font-semibold",
                                    "{author}"
                                }
                            }
                            small {
                                class: "text-base opacity-70",
                                "{post.date}"
                            }
                        }
                    }
                    div {
                        class: "mb-4 flex items-center justify-between border-y border-base-300 py-2",
                        small {
                            class: "block text-sm font-medium uppercase tracking-wide opacity-70",
                            "Share"
                        }
                        div {
                            class: "flex flex-row gap-2",
                            a {
                                class: "btn btn-ghost btn-sm",
                                href: "https://twitter.com/intent/tweet?url={page_permalink(post.folder)}",
                                img {
                                    width: "16",
                                    height: "16",
                                    src: "/social-sharing/x-twitter.svg"
                                }
                            }
                            a {
                                class: "btn btn-ghost btn-sm",
                                href: "https://www.linkedin.com/sharing/share-offsite/?url={page_permalink(post.folder)}",
                                img {
                                    width: "16",
                                    height: "16",
                                    src: "/social-sharing/linkedin.svg"
                                }
                            }
                        }
                    }
                    if let Some(image) = post.image {
                        img {
                            class: "mb-8 block aspect-video w-full rounded-box object-cover shadow-md",
                            src: image,
                            alt: "{post.title}"
                        }
                    }
                    div {
                        class: "prose prose-slate max-w-none prose-pre:overflow-x-auto prose-pre:rounded-xl prose-pre:bg-slate-100 prose-code:font-mono prose-img:max-w-full",
                        dangerous_inner_html: "{content}"
                    }
                }
            }
            if let Some(config) = extra_footer {
                ExtraFooter {
                    config
                }
            }
            Footer {
                margin_top: "mt-0",
                links: footer_links.clone()
            }
        }
    }
}

#[component]
pub fn BlogList(summary: Summary, footer_links: FooterLinks) -> Element {
    let hero_title = "Latest Insights".to_string();
    let hero_subtitle =
        "Ideas, implementation notes, and product updates from the team.".to_string();

    rsx! {
        Layout {
            title: "Blog",
            description: "Blog",
            mobile_menu: None,
            section: Section::Blog,
            section {
                class: "mx-auto mt-32 mb-12 max-w-5xl px-5 text-center",
                h1 {
                    class: "text-4xl font-extrabold md:text-5xl",
                    "{hero_title}"
                }
                h2 {
                    class: "mt-4 text-lg font-medium opacity-70 md:text-xl",
                    "{hero_subtitle}"
                }
            }
            section {
                class: "mx-auto max-w-6xl px-5 pb-8",
                div {
                    class: "grid gap-6 md:grid-cols-2",
                    for category in summary.categories {
                        for page in category.pages {
                            article {
                                class: "card mx-auto w-full max-w-sm bg-base-100 ring-1 ring-base-300",
                                figure {
                                    a {
                                        href: "/{page.folder}",
                                        if let Some(image) = page.image {
                                            img {
                                                class: "aspect-video w-full object-cover",
                                                src: if supports_resized_variants(image) {
                                                    image_variant_path(image, 384, 216)
                                                } else {
                                                    image.to_string()
                                                },
                                                srcset: if supports_resized_variants(image) {
                                                    format!(
                                                        "{} 1x, {} 2x",
                                                        image_variant_path(image, 384, 216),
                                                        image_variant_path(image, 768, 432)
                                                    )
                                                } else {
                                                    String::new()
                                                },
                                                sizes: "(min-width: 768px) 384px, 100vw",
                                                width: "384",
                                                height: "216",
                                                loading: "lazy",
                                                alt: "{page.title}",
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "card-body gap-3",
                                    h3 {
                                        class: "card-title text-balance text-xl",
                                        a {
                                            href: "/{page.folder}",
                                            "{page.title}"
                                        }
                                    }
                                    p {
                                        class: "text-sm font-semibold uppercase tracking-wide opacity-60",
                                        "{page.date}"
                                    }
                                    p {
                                        class: "line-clamp-3 opacity-80",
                                        "{page.description}"
                                    }
                                    div {
                                        class: "card-actions justify-end pt-2",
                                        a {
                                            class: "btn btn-outline btn-sm",
                                            href: "/{page.folder}",
                                            "Read More"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Footer {
                links: footer_links
            }
        }
    }
}
