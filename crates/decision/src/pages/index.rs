use dioxus::prelude::*;
use daisy_rsx::marketing::navigation::Section;
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "Decision Advantage | Command-Ready Judgment".to_string(),
            description: "Decision Advantage is a hardened decision layer that transforms fragmented operational data into command-ready judgment for senior leadership.".to_string(),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "decision-luxe",

                style {
                    dangerous_inner_html: include_str!("index_luxe.css")
                }

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
                                "Command-Ready"
                                i { "Judgment." }
                            }
                            p {
                                class: "dl-lead",
                                "Decision Advantage turns fragmented operational data into clear, defensible decisions-at the speed senior leadership requires."
                            }
                            a {
                                class: "dl-btn",
                                href: "/blog",
                                "Schedule a Demo"
                            }
                        }
                    }

                    section {
                        id: "artifacts",
                        class: "dl-artifacts dl-shell",
                        h2 { class: "dl-headline" , style: "font-size:clamp(1.7rem,4.7vw,3.5rem);", "Interactive" i { "Functional Artifacts" } }
                        p { class: "dl-lead", "Operational ideas should behave like software, not static slides. These live artifacts preview how Decision drives tempo and control." }
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
                                em { " decision integrity under pressure." }
                            }
                            p {
                                "Decision Advantage is designed for senior leaders operating under time compression, ambiguity, and consequence. It does not replace existing systems-it hardens them into a coherent decision layer that supports judgment, not automation theater."
                            }
                        }
                    }

                    section {
                        id: "protocol",
                        class: "dl-protocol-wrap dl-shell",
                        h2 { class: "dl-headline", style: "font-size:clamp(1.8rem,4.8vw,3.7rem);margin:1rem 1rem 2rem;", "Protocol" i { "Stacking Archive" } }
                        div {
                            class: "protocol-stack",

                            article {
                                class: "protocol-card",
                                id: "protocol-card-1",
                                div {
                                    span { class: "protocol-step", "PROTOCOL 01" }
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
                                    span { class: "protocol-step", "PROTOCOL 02" }
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
                                    span { class: "protocol-step", "PROTOCOL 03" }
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
                        h2 { "See Decision Advantage in Action." }
                        p { "A strategic demonstration tailored to senior command decision-making." }
                        a {
                            class: "dl-btn",
                            href: "/blog",
                            "Schedule a Demo"
                        }
                    }
                }

                footer {
                    class: "dl-footer",
                    div {
                        class: "dl-footer-grid",
                        div {
                            h4 { "Decision Advantage" }
                            p { class: "dl-lead", style: "font-size:0.92rem;max-width:30ch;", "Decision Advantage - The hardened decision layer." }
                            div { class: "status", span { class: "pulse-dot" } "System Operational" }
                        }
                        nav {
                            h4 { "Platform" }
                            a { href: "/#hero", "Overview" }
                            a { href: "/#protocol", "Architecture" }
                        }
                        nav {
                            h4 { "Protocol" }
                            a { href: "/#protocol", "Method" }
                            a { href: "/#manifesto", "Security" }
                        }
                        nav {
                            h4 { "Contact" }
                            a { href: "/blog", "Schedule Demo" }
                            a { href: "/blog", "Inquiries" }
                        }
                    }
                }

                script {
                    dangerous_inner_html: include_str!("index_luxe.js")
                }
            }
        }
    );

    ssg_whiz::render(page)
}
