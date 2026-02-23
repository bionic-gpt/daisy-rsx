use dioxus::prelude::*;
use daisy_rsx::marketing::{footer::Footer, navigation::Section};
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "Decision Advantage | Command-Ready Judgment".to_string(),
            description: "Decision Advantage is an agentic decision-support layer that turns fragmented operational data into command-ready judgment for senior leadership.".to_string(),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "decision-luxe",

                section {
                    id: "hero",
                    class: "dl-wrap",

                    div {
                        class: "dl-hero",
                        div {
                            class: "dl-hero-content",
                            span { class: "dl-kicker", "Decision Advantage" }
                            h1 {
                                class: "dl-headline",
                                "Agentic Decision Support for"
                                i { "Command Judgment." }
                            }
                            p {
                                class: "dl-lead",
                                "A hardened decision layer that coordinates agentic analysis across existing systems to deliver clear, defensible judgment at command tempo."
                            }
                            a {
                                class: "dl-btn",
                                href: "/contact",
                                "Schedule a Demo"
                            }
                        }
                    }

                    section {
                        id: "artifacts",
                        class: "dl-artifacts dl-shell",
                        h2 { class: "dl-headline" , style: "font-size:clamp(1.7rem,4.7vw,3.5rem);", "Operational Signals" i { "in Motion" } }
                        p { class: "dl-lead", "These live artifacts represent how Decision Advantage continuously stabilizes inputs, timing, and context into usable command judgment." }
                        div {
                            class: "dl-grid3",

                            article {
                                class: "dl-card",
                                h3 { "Diagnostic Shuffler" }
                                p { "Surfaces the signals that matter by continuously re-framing inputs across operational contexts." }
                                div {
                                    id: "diag-stage",
                                    class: "diag-stage",
                                    div { class: "diag-item", small { "Priority" } strong { "Operational Constraints" } }
                                    div { class: "diag-item", small { "Priority" } strong { "Confidence Gaps" } }
                                    div { class: "diag-item", small { "Priority" } strong { "Decision Risk" } }
                                }
                            }

                            article {
                                class: "dl-card",
                                h3 { "Telemetry Typewriter" }
                                p { "Streams live operational context into a readable decision feed-without dashboards or noise." }
                                div {
                                    class: "telemetry",
                                    div { class: "live", span { class: "pulse-dot" } "Live Feed" }
                                    div {
                                        code {
                                            id: "type-line",
                                            ""
                                            span { class: "cursor" }
                                        }
                                    }
                                }
                            }

                            article {
                                class: "dl-card",
                                h3 { "Cursor Protocol Scheduler" }
                                p { "Coordinates decision timing across staff, systems, and command rhythms." }
                                div {
                                    id: "scheduler",
                                    class: "scheduler",
                                    div { id: "schedule-cursor", class: "schedule-cursor" }
                                    div {
                                        class: "schedule-grid",
                                        div { class: "day-cell", "S" }
                                        div { class: "day-cell", "M" }
                                        div { class: "day-cell", "T" }
                                        div { class: "day-cell", "W" }
                                        div { class: "day-cell", "T" }
                                        div { class: "day-cell", "F" }
                                        div { class: "day-cell", "S" }
                                    }
                                    div { class: "save-btn", id: "save-btn", "Right decision. Right moment." }
                                }
                            }
                        }
                    }

                    section {
                        id: "manifesto",
                        class: "dl-manifesto",
                        div { class: "dl-parallax", id: "parallax" }
                        div {
                            class: "dl-manifesto-content",
                            p { "Most systems focus on collecting more data, adding more dashboards, and accelerating raw output." }
                            h2 {
                                class: "contrast",
                                "We focus on"
                                em { " decision integrity under pressure" }
                            }
                            p {
                                "Decision Advantage exists to support senior leaders operating under time compression and consequence. It does not replace systems or issue recommendations-it coordinates agentic decision support that strengthens judgment when clarity matters most."
                            }
                        }
                    }

                    section {
                        id: "protocol",
                        class: "dl-protocol-wrap dl-shell",
                        h2 { class: "dl-headline", style: "font-size:clamp(1.8rem,4.8vw,3.7rem);margin:1rem auto 0.5rem;text-align:center;", "Operational Decision" i { "Protocol" } }
                        p { class: "dl-lead", style: "margin:0 auto 1.6rem;text-align:center;", "A disciplined approach to stabilizing judgment without disrupting command systems." }
                        div {
                            class: "protocol-stack",

                            article {
                                class: "protocol-card",
                                id: "protocol-card-1",
                                div {
                                    span { class: "protocol-step", "Step 01" }
                                    h3 { class: "protocol-title", "Ingest" em { " Without Disruption" } }
                                    p { class: "protocol-desc", "Connects to existing operational systems without re-architecture. No workflow breakage. No retraining." }
                                }
                                div {
                                    class: "viz",
                                    div { class: "rotor" }
                                }
                            }

                            article {
                                class: "protocol-card",
                                id: "protocol-card-2",
                                div {
                                    span { class: "protocol-step", "Step 02" }
                                    h3 { class: "protocol-title", "Stabilize" em { " the Decision Frame" } }
                                    p { class: "protocol-desc", "Normalizes fragmented inputs into a single, defensible context senior leaders can trust." }
                                }
                                div {
                                    class: "viz",
                                    div { class: "scan-grid" }
                                    div { class: "scan-line" }
                                }
                            }

                            article {
                                class: "protocol-card",
                                id: "protocol-card-3",
                                div {
                                    span { class: "protocol-step", "Step 03" }
                                    h3 { class: "protocol-title", "Deliver" em { " Command Judgment" } }
                                    p { class: "protocol-desc", "Outputs clear decision posture-not recommendations-aligned to leadership intent and timing." }
                                }
                                div {
                                    class: "viz",
                                    svg {
                                        class: "wave",
                                        view_box: "0 0 320 110",
                                        path {
                                            d: "M0 56 C22 56, 24 30, 46 30 C68 30, 70 80, 95 80 C120 80, 122 52, 144 52 C166 52, 168 92, 190 92 C212 92, 216 40, 238 40 C260 40, 262 70, 285 70 C305 70, 306 56, 320 56"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section {
                        class: "dl-cta",
                        h2 { "Experience Decision Advantage." }
                        p { "Schedule a strategic demonstration focused on senior command decision-making." }
                        a {
                            class: "dl-btn",
                            href: "/contact",
                            "Schedule a Demo"
                        }
                    }
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }

                script {
                    dangerous_inner_html: include_str!("index_luxe.js")
                }
            }
        }
    );

    ssg_whiz::render(page)
}
