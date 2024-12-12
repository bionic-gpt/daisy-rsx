use dioxus::prelude::*;

#[component]
pub fn Hero(title: String, subtitle: String,
    cta: String,
    cta_link: String,) -> Element {
    rsx! {
        section {
            div {
                class: "flex justify-center text-center",
                div {
                    class: "max-w-lg",
                    h1 {
                        class: "text-5xl font-bold",
                        "{title}"
                    }
                    p {
                        class: "py-6",
                        "{subtitle}"
                    }
                    div {
                        class: "flex gap-2 justify-center",
                        a {
                            class: "btn btn-primary",
                            href: cta_link,
                            "{cta}"
                        }
                    }
                }
            }
        }
    }
}
