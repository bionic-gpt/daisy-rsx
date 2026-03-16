use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FooterLinks {
    pub blog: String,
    pub pricing: String,
    pub contact: String,
    pub terms: String,
    pub privacy: String,
    pub about: Option<String>,
}

#[component]
pub fn Footer(margin_top: Option<String>, links: FooterLinks) -> Element {
    let flush = margin_top.as_deref() == Some("mt-0");

    rsx! {
        footer {
            class: if flush {
                "mt-0 bg-base-200 px-6 py-10 text-base-content"
            } else {
                "mt-24 bg-base-200 px-6 py-10 text-base-content"
            },
            div {
                class: "mx-auto flex max-w-5xl flex-col gap-8 md:flex-row md:justify-between",
                nav {
                    class: "flex flex-col gap-2",
                    h6 {
                        class: "mb-2 text-sm font-bold uppercase tracking-[0.08em]",
                        "Resources"
                    }
                    a {
                        href: links.blog.clone(),
                        class: "link link-hover",
                        "Blog"
                    }
                    a {
                        href: links.pricing.clone(),
                        class: "link link-hover",
                        "Pricing"
                    }
                }
                nav {
                    class: "flex flex-col gap-2",
                    h6 {
                        class: "mb-2 text-sm font-bold uppercase tracking-[0.08em]",
                        "Company"
                    }
                    if let Some(about) = links.about.clone() {
                        a {
                            class: "link link-hover",
                            href: about,
                            "About Us"
                        }
                    } else {
                        a {
                            class: "link link-hover",
                            "About Us"
                        }
                    }
                    a {
                        href: links.contact.clone(),
                        class: "link link-hover",
                        "Contact"
                    }
                }
                nav {
                    class: "flex flex-col gap-2",
                    h6 {
                        class: "mb-2 text-sm font-bold uppercase tracking-[0.08em]",
                        "Legal"
                    }
                    a {
                        href: links.terms.clone(),
                        class: "link link-hover",
                        "Terms of Use"
                    }
                    a {
                        href: links.privacy.clone(),
                        class: "link link-hover",
                        "Privacy Policy"
                    }
                }
            }
        }
    }
}
