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
            class: "mt-24 px-6 py-16",
            div {
                class: "mx-auto flex max-w-6xl flex-col items-center gap-6 px-6 text-center",
                h2 {
                    class: "max-w-4xl text-3xl font-bold leading-tight md:text-4xl",
                    "{config.title}"
                }
                img {
                    class: "w-full max-w-4xl rounded-box shadow-lg",
                    alt: "{config.image_alt}",
                    src: "{config.image}"
                }
                div {
                    class: "flex flex-col gap-4 sm:flex-row sm:justify-center",
                    a {
                        href: "{config.cta_url}",
                        class: "btn btn-primary",
                        "{config.cta_label}"
                    }
                }
            }
        }
    }
}
