use dioxus::prelude::*;
use daisy_rsx::marketing::hero::Hero;

pub fn page() -> String {
    dioxus_ssr::render_element(rsx!(
        main {
            class: "container mx-auto px-6 py-20",
            Hero {
                title: "Speed to decision is a weapons system".to_string(),
                subtitle: "Our adversaries are not waiting. AI agents for decision advantage.".to_string(),
                cta_label: None,
                cta_href: None,
            }
        }
    ))
}
