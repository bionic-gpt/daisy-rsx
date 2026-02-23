use dioxus::prelude::*;

#[component]
pub fn SplitVideoHero(
    title: String,
    subtitle: String,
    video_src: String,
    cta_label: Option<String>,
    cta_href: Option<String>,
    class: Option<String>,
) -> Element {
    let cta_label = cta_label.unwrap_or_else(|| "Book a Call".to_string());
    let class = class.unwrap_or_default();

    rsx! {
        section {
            class: format!("grid gap-10 items-center lg:grid-cols-2 {class}"),
            div {
                class: "text-center lg:text-left",
                h1 {
                    class: "text-4xl sm:text-5xl font-bold",
                    "{title}"
                }
                p {
                    class: "mt-6 text-lg opacity-80",
                    "{subtitle}"
                }
                if let Some(cta_href) = cta_href {
                    div {
                        class: "mt-8 flex justify-center lg:justify-start",
                        a {
                            class: "btn btn-primary",
                            href: cta_href,
                            "{cta_label}"
                        }
                    }
                }
            }
            div {
                class: "w-full",
                video {
                    class: "w-full rounded-2xl shadow-lg",
                    src: video_src,
                    autoplay: true,
                    loop: true,
                    muted: true,
                    playsinline: true,
                    controls: false
                }
            }
        }
    }
}
