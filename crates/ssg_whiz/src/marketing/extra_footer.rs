use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtraFooterConfig {
    pub title: String,
    pub image: String,
    pub image_alt: String,
    pub cta_label: String,
    pub cta_url: String,
}

#[component]
pub fn ExtraFooter(config: ExtraFooterConfig) -> Element {
    rsx! {
        section {
            class: "site-extra-footer",
            div {
                class: "site-extra-footer__inner",
                h2 {
                    class: "site-extra-footer__title",
                    "{config.title}"
                }
                img {
                    class: "site-extra-footer__image",
                    alt: "{config.image_alt}",
                    src: "{config.image}"
                }
                div {
                    class: "site-extra-footer__actions",
                    a {
                        href: "{config.cta_url}",
                        class: "site-extra-footer__button",
                        "{config.cta_label}"
                    }
                }
            }
        }
    }
}
