use dioxus::prelude::*;
use daisy_rsx::marketing::{
    footer::Footer,
    landing_page::MarketingLandingPage,
    landing_spec::{
        BenefitItem, BenefitsBlock, FaqBlock, FaqItem, FeatureBlock, HeroBlock, HeroVariant,
        ImageFeatureBlock, LandingPageSpec, LogoItem, PricingOrCtaBlock, ProofBlock, QuadFeatureBlock,
        QuadItem, SmallImageFeatureBlock, TestimonialItem, TestimonialsBlock,
    },
    navigation::Section,
    theme::decision_theme,
};
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let spec = LandingPageSpec {
        hero: HeroBlock {
            variant: HeroVariant::SplitVideo {
                video_src: "/european_flag.mp4".to_string(),
            },
            title: "Speed to decision is a weapons system".to_string(),
            subtitle: "Plan, simulate, and adapt in minutes instead of hours.".to_string(),
            cta_label: Some("See Demo".to_string()),
            cta_href: Some("/contact".to_string()),
        },
        proof: Some(ProofBlock {
            title: "Trusted by allied innovation teams".to_string(),
            logos: vec![
                LogoItem {
                    src: "/customer-logos/logo-1.svg".to_string(),
                    alt: "Customer logo one".to_string(),
                },
                LogoItem {
                    src: "/customer-logos/logo-2.svg".to_string(),
                    alt: "Customer logo two".to_string(),
                },
                LogoItem {
                    src: "/customer-logos/logo-3.svg".to_string(),
                    alt: "Customer logo three".to_string(),
                },
            ],
        }),
        feature_blocks: vec![
            FeatureBlock::SmallImage(SmallImageFeatureBlock {
                title: "Operational Tempo".to_string(),
                sub_title: "From fragmented intel to coordinated action".to_string(),
                text: "Operational teams lose tempo when intelligence, planning, and execution tools are disconnected across domains and classifications. Decision Advantage summarizes context, proposes COAs, and keeps commanders in the loop with explainable outputs.".to_string(),
                image: "https://placehold.co/1024x768/0A2A66/FFFFFF?text=Joint+Ops+Picture".to_string(),
                flip: true,
            }),
            FeatureBlock::SmallImage(SmallImageFeatureBlock {
                title: "Mission Planning".to_string(),
                sub_title: "Generate COAs with transparent assumptions".to_string(),
                text: "AI agents continuously evaluate constraints, resources, and adversary posture to propose actionable courses of action with confidence scoring.".to_string(),
                image: "https://placehold.co/960x640/123D8D/FFFFFF?text=COA+Generator".to_string(),
                flip: false,
            }),
            FeatureBlock::SmallImage(SmallImageFeatureBlock {
                title: "Cross-Domain Data".to_string(),
                sub_title: "Fuse ISR, cyber, logistics, and HUMINT".to_string(),
                text: "Unify structured and unstructured data into a single operational view that is queryable by commanders and staff in natural language.".to_string(),
                image: "https://placehold.co/960x640/0F347C/FFFFFF?text=Data+Fusion".to_string(),
                flip: true,
            }),
            FeatureBlock::Image(ImageFeatureBlock {
                title: "Command Timeline & Scenario Replay".to_string(),
                sub_title: "Understand why decisions were made and how outcomes evolved".to_string(),
                image: "https://placehold.co/1280x720/1B4B9B/FFFFFF?text=Timeline+Replay".to_string(),
            }),
            FeatureBlock::Quad(QuadFeatureBlock {
                title: "Core Capabilities".to_string(),
                sub_title: "Built for contested, data-dense environments".to_string(),
                text: "Decision Advantage turns information overload into prioritized, explainable actions for military operators.".to_string(),
                items: [
                    QuadItem {
                        title: "Human-on-the-loop".to_string(),
                        text: "Operators approve every high-impact action with full provenance.".to_string(),
                    },
                    QuadItem {
                        title: "Secure by Design".to_string(),
                        text: "Deploy in controlled environments with auditable model and data access.".to_string(),
                    },
                    QuadItem {
                        title: "Continuous Adaptation".to_string(),
                        text: "Agents re-plan as mission context and threat indicators change.".to_string(),
                    },
                    QuadItem {
                        title: "After-Action Intelligence".to_string(),
                        text: "Replay decisions, capture lessons learned, and improve doctrine.".to_string(),
                    },
                ],
            }),
            FeatureBlock::Benefits(BenefitsBlock {
                title: "Outcomes".to_string(),
                subtitle: "Operational benefits you can measure".to_string(),
                items: [
                    BenefitItem {
                        title: "Faster OODA loops".to_string(),
                        description: "Shorten observe-orient-decide-act cycles with realtime, context-aware recommendations.".to_string(),
                    },
                    BenefitItem {
                        title: "Higher decision quality".to_string(),
                        description: "Improve consistency with explainable AI support and validated planning templates.".to_string(),
                    },
                    BenefitItem {
                        title: "Mission assurance".to_string(),
                        description: "Maintain traceability, policy alignment, and post-mission audit records.".to_string(),
                    },
                ],
            }),
        ],
        problem_solution: None,
        process: None,
        pricing_or_cta: PricingOrCtaBlock::Cta {
            title: "Operational advantage starts with faster decisions".to_string(),
            subtitle: "See how Decision Advantage fits your mission workflows.".to_string(),
            button_label: "Request a Demo".to_string(),
            button_href: "/contact".to_string(),
        },
        testimonials: Some(TestimonialsBlock {
            title: "Testimonials".to_string(),
            items: [
                TestimonialItem {
                    text: "Decision Advantage reduced our planning cycle by over 40% while preserving command authority and discipline.".to_string(),
                    job: "Joint Task Force Operations Lead".to_string(),
                    person: "A. Commander".to_string(),
                    image: "https://placehold.co/96x96/001741/FFFFFF?text=AC".to_string(),
                },
                TestimonialItem {
                    text: "We now run scenario branches in parallel and brief leadership with evidence-backed COAs in near real time.".to_string(),
                    job: "Defense Analytics Director".to_string(),
                    person: "R. Analyst".to_string(),
                    image: "https://placehold.co/96x96/0A2A66/FFFFFF?text=RA".to_string(),
                },
            ],
        }),
        faq: Some(FaqBlock {
            title: "Frequently asked questions".to_string(),
            items: vec![
                FaqItem {
                    question: "Does this replace commanders?".to_string(),
                    answer: "No. The platform is designed for human command authority, with AI providing recommendations and decision support.".to_string(),
                },
                FaqItem {
                    question: "Can it run in sovereign or on-prem environments?".to_string(),
                    answer: "Yes. Decision Advantage can be deployed in controlled environments to meet operational and sovereignty constraints.".to_string(),
                },
                FaqItem {
                    question: "How do we trust model outputs?".to_string(),
                    answer: "Every recommendation includes traceable sources, assumptions, and confidence signals for operator validation.".to_string(),
                },
            ],
        }),
    };

    let page = rsx!(
        Layout {
            title: "Decision Advantage".to_string(),
            description: "Agentic military AI for faster, auditable operational decision advantage.".to_string(),
            mobile_menu: None,
            section: Section::Home,
            main {
                MarketingLandingPage {
                    theme: decision_theme(),
                    spec: spec,
                }
            }
            Footer {
                links: crate::ui_links::footer_links()
            }
        }
    );

    ssg_whiz::render(page)
}
