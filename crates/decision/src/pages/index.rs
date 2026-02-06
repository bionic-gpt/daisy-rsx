use dioxus::prelude::*;
use daisy_rsx::marketing::footer::Footer;
use daisy_rsx::marketing::hero::Hero;
use daisy_rsx::marketing::navigation::Section;
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "Decision".to_string(),
            description: "Speed to decision is a weapons system.".to_string(),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "container mx-auto px-6 py-20",
                Hero {
                    title: "Speed to decision is a weapons system".to_string(),
                    subtitle: "Our adversaries are not waiting. AI agents for decision advantage.".to_string(),
                    cta_label: None,
                    cta_href: None,
                }
            }
            Footer {
                links: crate::ui_links::footer_links()
            }
        }
    );

    ssg_whiz::render(page)
}
