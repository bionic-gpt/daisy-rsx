use super::layout::Layout;
use crate::{
    page_permalink, site_meta,
    summaries::{PageSummary, Summary},
    ExtraFooter, Footer, FooterLinks, Section, EXTRA_FOOTER_TITLE,
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
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            url: Some(page_permalink(post.folder)),
            image: post.image.map(|image| image.to_string()),
            section: Section::Blog,
            article {
                class: "blog-post",
                h1 {
                    class: "blog-post__title",
                    "{post.title}"
                }
                div {
                    class: "blog-post__meta",
                    if let Some(author_image) = post.author_image {
                        img {
                            class: "blog-post__author-image",
                            width: "44",
                            height: "44",
                            src: author_image,
                            alt: "Author"
                        }
                    }
                    div {
                        class: "blog-post__meta-text",
                        if let Some(author) = post.author {
                            strong {
                                class: "blog-post__author",
                                "{author}"
                            }
                        }
                        small {
                            class: "blog-post__date",
                            "{post.date}"
                        }
                    }
                }
                div {
                    class: "blog-post__share",
                    small {
                        class: "blog-post__share-label",
                        "Share"
                    }
                    div {
                        class: "blog-post__share-links",
                        a {
                            href: "https://twitter.com/intent/tweet?url={page_permalink(post.folder)}",
                            img {
                                width: "16",
                                height: "16",
                                src: "/social-sharing/x-twitter.svg"
                            }
                        }
                        a {
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
                        class: "blog-post__hero-image",
                        src: image,
                        alt: "{post.title}"
                    }
                }
                div {
                    class: "blog-post__content",
                    dangerous_inner_html: "{content}"
                }
            }
            ExtraFooter {
                title: EXTRA_FOOTER_TITLE.to_string(),
                image: "/landing-page/bionic-console.png",
                cta: "Find out More",
                cta_url: "/"
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
    let meta = site_meta();
    let (hero_title, hero_subtitle) = if meta.brand_name == "Decision" {
        (
            "Decision Briefings".to_string(),
            "Operational insights, command judgment patterns, and secure decision-support practices.".to_string()
        )
    } else {
        (
            "Enterprise Generative AI".to_string(),
            "The Bionic blog explores issues around LLMs in the enterprise".to_string(),
        )
    };

    rsx! {
        Layout {
            title: "Blog",
            description: "Blog",
            mobile_menu: None,
            section: Section::Blog,
            section {
                class: "blog-list__hero",
                h1 {
                    class: "blog-list__title",
                    "{hero_title}"
                }
                h2 {
                    class: "blog-list__subtitle",
                    "{hero_subtitle}"
                }
            }
            section {
                class: "blog-list__content",
                div {
                    div {
                        class: "blog-list__grid",
                        for category in summary.categories {
                            for page in category.pages {
                                div {
                                    class: "blog-card",
                                    a {
                                        class: "blog-card__image-link",
                                        href: "/{page.folder}",
                                        if let Some(image) = page.image {
                                            img {
                                                class: "blog-card__image",
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
                                    div {
                                        class: "blog-card__body",
                                        div {
                                            class: "blog-card__content",
                                            h3 {
                                                class: "blog-card__title",
                                                "{page.title}"
                                            }
                                            p {
                                                class: "blog-card__date",
                                                strong {
                                                    "{page.date}"
                                                }
                                            }
                                            p {
                                                class: "blog-card__cta",
                                                a {
                                                    href: "/{page.folder}",
                                                    "Read More..."
                                                }
                                            }
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
