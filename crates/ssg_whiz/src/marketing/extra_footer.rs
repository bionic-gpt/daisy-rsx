use dioxus::prelude::*;

pub type ExtraFooterFactory = fn() -> Element;

#[derive(Clone, Debug)]
pub enum ExtraFooterSlot {
    BuiltIn(ExtraFooterConfig),
    Custom(ExtraFooterFactory),
}

impl PartialEq for ExtraFooterSlot {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BuiltIn(left), Self::BuiltIn(right)) => left == right,
            (Self::Custom(left), Self::Custom(right)) => std::ptr::fn_addr_eq(*left, *right),
            _ => false,
        }
    }
}

impl Eq for ExtraFooterSlot {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtraFooterConfig {
    pub title: String,
    pub image: String,
    pub image_alt: String,
    pub cta_label: String,
    pub cta_url: String,
}

#[component]
pub fn RenderExtraFooter(slot: ExtraFooterSlot) -> Element {
    match slot {
        ExtraFooterSlot::BuiltIn(config) => rsx! {
            ExtraFooter {
                config
            }
        },
        ExtraFooterSlot::Custom(factory) => factory(),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn custom_footer() -> Element {
        rsx! {
            section {
                "Custom extra footer"
            }
        }
    }

    fn built_in_config() -> ExtraFooterConfig {
        ExtraFooterConfig {
            title: "Built-in extra footer".to_string(),
            image: "/footer.png".to_string(),
            image_alt: "Footer image".to_string(),
            cta_label: "Read more".to_string(),
            cta_url: "/read-more".to_string(),
        }
    }

    #[test]
    fn renders_built_in_extra_footer_slot() {
        let html = dioxus_ssr::render_element(rsx! {
            RenderExtraFooter {
                slot: ExtraFooterSlot::BuiltIn(built_in_config())
            }
        });

        assert!(html.contains("Built-in extra footer"));
        assert!(html.contains("Footer image"));
        assert!(html.contains("Read more"));
        assert!(html.contains("/read-more"));
    }

    #[test]
    fn renders_custom_extra_footer_slot() {
        let html = dioxus_ssr::render_element(rsx! {
            RenderExtraFooter {
                slot: ExtraFooterSlot::Custom(custom_footer)
            }
        });

        assert!(html.contains("Custom extra footer"));
        assert!(!html.contains("btn btn-primary"));
    }

    #[test]
    fn renders_nothing_without_extra_footer_slot() {
        let slot: Option<ExtraFooterSlot> = None;
        let html = dioxus_ssr::render_element(rsx! {
            if let Some(slot) = slot {
                RenderExtraFooter {
                    slot
                }
            }
        });

        assert!(html.is_empty());
    }
}
