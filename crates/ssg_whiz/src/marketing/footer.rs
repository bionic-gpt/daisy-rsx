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
            class: if flush { "site-footer site-footer--flush" } else { "site-footer" },
            div {
                class: "site-footer__inner",
                nav {
                    class: "site-footer__section",
                    h6 {
                        class: "site-footer__title",
                        "Resources"
                    }
                    a {
                        href: links.blog.clone(),
                        class: "site-footer__link",
                        "Blog"
                    }
                    a {
                        href: links.pricing.clone(),
                        class: "site-footer__link",
                        "Pricing"
                    }
                }
                nav {
                    class: "site-footer__section",
                    h6 {
                        class: "site-footer__title",
                        "Company"
                    }
                    if let Some(about) = links.about.clone() {
                        a {
                            class: "site-footer__link",
                            href: about,
                            "About Us"
                        }
                    } else {
                        a {
                            class: "site-footer__link",
                            "About Us"
                        }
                    }
                    a {
                        href: links.contact.clone(),
                        class: "site-footer__link",
                        "Contact"
                    }
                }
                nav {
                    class: "site-footer__section",
                    h6 {
                        class: "site-footer__title",
                        "Legal"
                    }
                    a {
                        href: links.terms.clone(),
                        class: "site-footer__link",
                        "Terms of Use"
                    }
                    a {
                        href: links.privacy.clone(),
                        class: "site-footer__link",
                        "Privacy Policy"
                    }
                }
            }
        }
    }
}
