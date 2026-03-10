use dioxus::prelude::*;

pub const EXTRA_FOOTER_TITLE: &str =
    "The all-in-one agentic AI platform for regulated teams—secure, open, and extensible end to end.";

#[component]
pub fn ExtraFooter(title: String, image: String, cta: String, cta_url: String) -> Element {
    rsx! {
        section {
            class: "site-extra-footer",
            div {
                class: "site-extra-footer__inner",
                h2 {
                    class: "site-extra-footer__title",
                    "{title}"
                }
                img {
                    class: "site-extra-footer__image",
                    alt: "Product Screenshot",
                    src: "{image}"
                }
                div {
                    class: "site-extra-footer__actions",
                    a {
                        href: "{cta_url}",
                        class: "site-extra-footer__button",
                        "{cta}"
                    }
                }
            }
        }
    }
}
