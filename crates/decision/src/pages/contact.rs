use dioxus::prelude::*;

use daisy_rsx::marketing::{
    footer::Footer,
    navigation::Section,
};
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx! {
        Layout {
            title: "Contact Decision Advantage".to_string(),
            description: "Schedule a strategic demonstration with the Decision Advantage team.".to_string(),
            mobile_menu: None,
            section: Section::Contact,

            main {
                class: "decision-luxe",
                section {
                    class: "dl-wrap",
                    div {
                        class: "dl-shell",
                        style: "padding:2rem; margin-top:7.5rem;",
                        h1 {
                            class: "dl-headline",
                            style: "font-size:clamp(1.9rem,4.8vw,3.7rem);",
                            "Schedule a"
                            i { "Strategic Demo" }
                        }
                        p {
                            class: "dl-lead",
                            style: "margin-top:0.9rem;max-width:72ch;",
                            "Meet with the Decision Advantage team to review your operational context, current system landscape, and command decision objectives."
                        }

                        div {
                            class: "dl-grid3",
                            style: "margin-top:1.6rem;",
                            article {
                                class: "dl-card",
                                h3 { "Command Workflow Review" }
                                p { "Map where decision latency appears across leadership and staff processes." }
                            }
                            article {
                                class: "dl-card",
                                h3 { "Integration Readiness" }
                                p { "Assess how Decision connects into existing data and operational systems without disruption." }
                            }
                            article {
                                class: "dl-card",
                                h3 { "Security & Assurance" }
                                p { "Review controls, assurance posture, and governance fit for high-consequence operations." }
                            }
                        }

                        div {
                            style: "margin-top:1.8rem; display:flex; gap:0.75rem; flex-wrap:wrap;",
                            a {
                                class: "dl-btn",
                                href: "https://calendly.com/bionicgpt",
                                "Book a Demo"
                            }
                            a {
                                class: "dl-btn",
                                href: "mailto:team@decision.example.com",
                                "Email the Team"
                            }
                        }
                    }
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    };

    ssg_whiz::render(page)
}
