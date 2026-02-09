use dioxus::prelude::*;
use daisy_rsx::marketing::{
    benefits::Benefits,
    faq_accordian::{Faq, FaqText},
    footer::Footer,
    image_feature::ImageFeature,
    navigation::Section,
    quad_feature::QuadFeature,
    small_image_feature::SmallImageFeature,
    split_video_hero::SplitVideoHero,
    testamonials::Testamonials,
};
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "Decision Advantage".to_string(),
            description: "Agentic military AI for faster, auditable operational decision advantage.".to_string(),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "mx-auto w-full max-w-5xl px-6 pt-32 pb-20 lg:pt-40 grid gap-y-28 lg:gap-y-36",
                SplitVideoHero {
                    title: "Speed to decision is a weapons system".to_string(),
                    subtitle: "Plan, simulate, and adapt in minutes instead of hours.".to_string(),
                    video_src: "/european_flag.mp4".to_string(),
                    cta_label: Some("See Demo".to_string()),
                    cta_href: Some("/contact".to_string()),
                }

                QuadFeature {
                    title: "Core Capabilities".to_string(),
                    sub_title: "Built for contested, data-dense environments".to_string(),
                    text: "Decision Advantage turns information overload into prioritized, explainable actions for military operators.".to_string(),
                    title1: "Human-on-the-loop".to_string(),
                    text1: "Operators approve every high-impact action with full provenance.".to_string(),
                    title2: "Secure by Design".to_string(),
                    text2: "Deploy in controlled environments with auditable model and data access.".to_string(),
                    title3: "Continuous Adaptation".to_string(),
                    text3: "Agents re-plan as mission context and threat indicators change.".to_string(),
                    title4: "After-Action Intelligence".to_string(),
                    text4: "Replay decisions, capture lessons learned, and improve doctrine.".to_string(),
                }

                SmallImageFeature {
                    title: "Operational Tempo".to_string(),
                    sub_title: "From fragmented intel to coordinated action".to_string(),
                    text: "Operational teams lose tempo when intelligence, planning, and execution tools are disconnected across domains and classifications. Decision Advantage summarizes context, proposes COAs, and keeps commanders in the loop with explainable outputs.".to_string(),
                    image: "https://placehold.co/1024x768/0A2A66/FFFFFF?text=Joint+Ops+Picture".to_string(),
                    flip: true,
                }

                SmallImageFeature {
                    title: "Mission Planning".to_string(),
                    sub_title: "Generate COAs with transparent assumptions".to_string(),
                    text: "AI agents continuously evaluate constraints, resources, and adversary posture to propose actionable courses of action with confidence scoring.".to_string(),
                    image: "https://placehold.co/960x640/123D8D/FFFFFF?text=COA+Generator".to_string(),
                    flip: false,
                }

                SmallImageFeature {
                    title: "Cross-Domain Data".to_string(),
                    sub_title: "Fuse ISR, cyber, logistics, and HUMINT".to_string(),
                    text: "Unify structured and unstructured data into a single operational view that is queryable by commanders and staff in natural language.".to_string(),
                    image: "https://placehold.co/960x640/0F347C/FFFFFF?text=Data+Fusion".to_string(),
                    flip: true,
                }

                ImageFeature {
                    title: "Command Timeline & Scenario Replay".to_string(),
                    sub_title: "Understand why decisions were made and how outcomes evolved".to_string(),
                    image: "https://placehold.co/1280x720/1B4B9B/FFFFFF?text=Timeline+Replay".to_string(),
                }

                Benefits {
                    title: "Outcomes".to_string(),
                    subtitle: "Operational benefits you can measure".to_string(),
                    benefit1: "Faster OODA loops".to_string(),
                    benefit1_desc: "Shorten observe-orient-decide-act cycles with realtime, context-aware recommendations.".to_string(),
                    benefit2: "Higher decision quality".to_string(),
                    benefit2_desc: "Improve consistency with explainable AI support and validated planning templates.".to_string(),
                    benefit3: "Mission assurance".to_string(),
                    benefit3_desc: "Maintain traceability, policy alignment, and post-mission audit records.".to_string(),
                }

                Testamonials {
                    text1: "Decision Advantage reduced our planning cycle by over 40% while preserving command authority and discipline.",
                    job1: "Joint Task Force Operations Lead",
                    person1: "A. Commander",
                    img1: "https://placehold.co/96x96/001741/FFFFFF?text=AC",
                    text2: "We now run scenario branches in parallel and brief leadership with evidence-backed COAs in near real time.",
                    job2: "Defense Analytics Director",
                    person2: "R. Analyst",
                    img2: "https://placehold.co/96x96/0A2A66/FFFFFF?text=RA",
                }

                Faq {
                    questions: vec![
                        FaqText {
                            question: "Does this replace commanders?".to_string(),
                            answer: "No. The platform is designed for human command authority, with AI providing recommendations and decision support.".to_string(),
                        },
                        FaqText {
                            question: "Can it run in sovereign or on-prem environments?".to_string(),
                            answer: "Yes. Decision Advantage can be deployed in controlled environments to meet operational and sovereignty constraints.".to_string(),
                        },
                        FaqText {
                            question: "How do we trust model outputs?".to_string(),
                            answer: "Every recommendation includes traceable sources, assumptions, and confidence signals for operator validation.".to_string(),
                        },
                    ],
                }
            }
            Footer {
                links: crate::ui_links::footer_links()
            }
        }
    );

    ssg_whiz::render(page)
}
