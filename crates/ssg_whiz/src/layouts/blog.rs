use super::layout::Layout;
use crate::{page_permalink, site_meta, summaries::{PageSummary, Summary}};
use daisy_rsx::marketing::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    footer::{Footer, FooterLinks},
    navigation::Section,
};
use dioxus::prelude::*;

fn image_variant_path(path: &str, width: u32, height: u32) -> String {
    if let Some((base, ext)) = path.rsplit_once('.') {
        format!("{base}-{width}x{height}.{ext}")
    } else {
        path.to_string()
    }
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
                class: "mt-32 mx-auto prose lg:prose-xl p-5",
                h1 {
                    "{post.title}"
                }
                div {
                    class: "not-prose flex flex-row mt-8 mb-4",
                    if let Some(author_image) = post.author_image {
                        img {
                            width: "44",
                            height: "44",
                            src: author_image,
                            alt: "Author"
                        }
                    }
                    div {
                        class: "not-prose flex flex-col pl-2",
                        if let Some(author) = post.author {
                            strong {
                                class: "text-base",
                                "{author}"
                            }
                        }
                        small {
                            class: "text-base",
                            "{post.date}"
                        }
                    }
                }
                div {
                    class: "not-prose flex justify-between items-center border-t border-b mb-4",
                    small {
                        class: "not-prose",
                        "Share"
                    }
                    div {
                        class: "not-prose flex flex-row gap-1",
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
                        class: "mb-8 object-cover h-96 w-full",
                        src: image,
                        alt: "{post.title}"
                    }
                }
                div {
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
            "The Bionic blog explores issues around LLMs in the enterprise".to_string()
        )
    };

    rsx! {
        Layout {
            title: "Blog",
            description: "Blog",
            mobile_menu: None,
            section: Section::Blog,
            section {
                class: "lg:max-w-5xl mx-auto text-center mb-12 mt-32",
                h1 {
                    class: "text-4xl font-extrabold",
                    "{hero_title}"
                }
                h2 {
                    class: "text-2xl font-bold",
                    "{hero_subtitle}"
                }
            }
            section {
                class: "lg:max-w-5xl mx-auto p-4",
                div {
                    div {
                        class: "md:grid grid-cols-2 gap-4",
                        for category in summary.categories {
                            for page in category.pages {
                                div {
                                    class: "border p-4",
                                    a {
                                        href: "/{page.folder}",
                                        if let Some(image) = page.image {
                                            img {
                                                class: "w-full aspect-[16/9] object-cover rounded-md",
                                                src: image_variant_path(image, 384, 216),
                                                srcset: "{image_variant_path(image, 384, 216)} 1x, {image_variant_path(image, 768, 432)} 2x",
                                                sizes: "(min-width: 768px) 384px, 100vw",
                                                width: "384",
                                                height: "216",
                                                loading: "lazy",
                                                alt: "{page.title}",
                                            }
                                        }
                                    }
                                    div {
                                        div {
                                            h3 {
                                                "{page.title}"
                                            }
                                            p {
                                                class: "subtitle",
                                                strong {
                                                    "{page.date}"
                                                }
                                            }
                                            p {
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
