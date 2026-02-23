use dioxus::prelude::*;
use daisy_rsx::marketing::navigation::Section;
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "Decision | Command Velocity Platform".to_string(),
            description: "Command-grade decision velocity for operations that cannot wait.".to_string(),
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
                            span { class: "dl-kicker", "MIDNIGHT LUXE // DECISION OPS" }
                            h1 {
                                class: "dl-headline",
                                "Command theater,"
                                i { "precision in motion." }
                            }
                            p {
                                class: "dl-lead",
                                "Decision transforms fragmented war-room signals into one continuous operational instrument. Every brief, branch, and recommendation arrives in tempo with your command intent."
                            }
                            a {
                                class: "dl-btn",
                                href: "/blog",
                                "Enter the Briefing Room"
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
                                p { "Mission diagnostics reprioritize every 3 seconds so teams see where attention should move next." }
                                div {
                                    id: "diag-stage",
                                    class: "diag-stage",
                                    div { class: "diag-item", small { "Priority" } strong { "Supply chain latency" } }
                                    div { class: "diag-item", small { "Priority" } strong { "Denied-area ISR drift" } }
                                    div { class: "diag-item", small { "Priority" } strong { "Comms degradation" } }
                                }
                            }

                            article {
                                class: "dl-card",
                                h3 { "Telemetry Typewriter" }
                                p { "Live command feed types itself character-by-character with a persistent operations cursor." }
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
                                p { "The scheduler traces a decision cadence: select day, commit plan, save mission cycle." }
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
                                    div { class: "save-btn", id: "save-btn", "Save Protocol" }
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
                            p { "Most operations software optimizes for dashboard readability and retrospective reporting." }
                            h2 {
                                class: "contrast",
                                "We optimize for"
                                em { " irreversible minutes." }
                            }
                            p {
                                "Decision keeps leaders in command while compressing observe-orient-decide-act loops into a disciplined, auditable tempo."
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
                                    span { class: "protocol-step", "STEP 01" }
                                    h3 { class: "protocol-title", "Ingest" em { " Theater Signals" } }
                                    p { class: "protocol-desc", "Decision fuses ISR, HUMINT, logistics, and mission objectives into a single operational graph that can be trusted under pressure." }
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
                                    span { class: "protocol-step", "STEP 02" }
                                    h3 { class: "protocol-title", "Scan" em { " Decision Paths" } }
                                    p { class: "protocol-desc", "Courses of action are scored against doctrine, constraints, and confidence, then surfaced with transparent tradeoffs for command approval." }
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
                                    span { class: "protocol-step", "STEP 03" }
                                    h3 { class: "protocol-title", "Commit" em { " and Adapt" } }
                                    p { class: "protocol-desc", "Every recommendation becomes an auditable pulse in the command timeline, enabling faster adaptation without surrendering authority." }
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
                        h2 { "Ready to run command at luxury speed?" }
                        p { "Stand up your first mission-specific Decision workflow and watch planning latency collapse while control stays with leadership." }
                        a {
                            class: "dl-btn",
                            href: "/blog",
                            "Start Mission Setup"
                        }
                    }
                }

                footer {
                    class: "dl-footer",
                    div {
                        class: "dl-footer-grid",
                        div {
                            h4 { "Decision" }
                            p { class: "dl-lead", style: "font-size:0.92rem;max-width:30ch;", "A command-grade decision platform for teams operating where the cost of delay is strategic." }
                            div { class: "status", span { class: "pulse-dot" } "System Operational" }
                        }
                        nav {
                            h4 { "Navigate" }
                            a { href: "/", "Home" }
                            a { href: "/blog", "Briefings" }
                            a { href: "/#protocol", "Protocol" }
                        }
                        nav {
                            h4 { "Programs" }
                            a { href: "/blog", "Scenario Library" }
                            a { href: "/blog", "Operational Notes" }
                            a { href: "/blog", "Release Signals" }
                        }
                        nav {
                            h4 { "Legal" }
                            a { href: "/", "Privacy" }
                            a { href: "/", "Terms" }
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
