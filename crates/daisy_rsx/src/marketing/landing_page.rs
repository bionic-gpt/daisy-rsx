use dioxus::prelude::*;

use crate::marketing::{
    benefits::Benefits,
    customer_logos::{CustomerLogo, CustomersConfigured},
    faq_accordian::{Faq, FaqText},
    features::{Feature, Features},
    hero::Hero,
    image_feature::ImageFeature,
    image_hero::ImageHero,
    landing_spec::{
        FeatureBlock, HeroVariant, LandingPageSpec, PricingOrCtaBlock,
    },
    problem_solution::ProblemSolution,
    quad_feature::QuadFeature,
    small_image_feature::SmallImageFeature,
    split_video_hero::SplitVideoHero,
    testamonials::Testamonials,
    theme::MarketingTheme,
    video_hero::VideoHero,
};

#[component]
pub fn MarketingLandingPage(theme: MarketingTheme, spec: LandingPageSpec) -> Element {
    rsx! {
        div {
            class: format!("{} {}", theme.section_spacing.page_container, theme.section_spacing.section_gap),

            match &spec.hero.variant {
                HeroVariant::Basic => rsx! {
                    Hero {
                        title: spec.hero.title.clone(),
                        subtitle: spec.hero.subtitle.clone(),
                        cta_label: spec.hero.cta_label.clone(),
                        cta_href: spec.hero.cta_href.clone(),
                        class: None,
                    }
                },
                HeroVariant::Image { image } => rsx! {
                    ImageHero {
                        title: spec.hero.title.clone(),
                        subtitle: spec.hero.subtitle.clone(),
                        image: image.clone(),
                        cta_label: spec.hero.cta_label.clone(),
                        cta_href: spec.hero.cta_href.clone(),
                        class: None,
                    }
                },
                HeroVariant::Video { video_id, claim } => rsx! {
                    VideoHero {
                        title: spec.hero.title.clone(),
                        subtitle: spec.hero.subtitle.clone(),
                        video_id: video_id.clone(),
                        claim: claim.clone(),
                        cta_label: spec.hero.cta_label.clone(),
                        cta_href: spec.hero.cta_href.clone(),
                        class: None,
                    }
                },
                HeroVariant::SplitVideo { video_src } => rsx! {
                    SplitVideoHero {
                        title: spec.hero.title.clone(),
                        subtitle: spec.hero.subtitle.clone(),
                        video_src: video_src.clone(),
                        cta_label: spec.hero.cta_label.clone(),
                        cta_href: spec.hero.cta_href.clone(),
                        class: None,
                    }
                },
            }

            if let Some(proof) = &spec.proof {
                CustomersConfigured {
                    title: proof.title.clone(),
                    logos: proof.logos.iter().map(|logo| CustomerLogo {
                        src: logo.src.clone(),
                        alt: logo.alt.clone(),
                    }).collect(),
                    class: None,
                }
            }

            for block in &spec.feature_blocks {
                match block {
                    FeatureBlock::SmallImage(feature) => rsx! {
                        SmallImageFeature {
                            title: feature.title.clone(),
                            sub_title: feature.sub_title.clone(),
                            text: feature.text.clone(),
                            image: feature.image.clone(),
                            flip: feature.flip,
                            class: None,
                        }
                    },
                    FeatureBlock::Image(feature) => rsx! {
                        ImageFeature {
                            title: feature.title.clone(),
                            sub_title: feature.sub_title.clone(),
                            image: feature.image.clone(),
                        }
                    },
                    FeatureBlock::Quad(feature) => rsx! {
                        QuadFeature {
                            title: feature.title.clone(),
                            sub_title: feature.sub_title.clone(),
                            text: feature.text.clone(),
                            title1: feature.items[0].title.clone(),
                            text1: feature.items[0].text.clone(),
                            title2: feature.items[1].title.clone(),
                            text2: feature.items[1].text.clone(),
                            title3: feature.items[2].title.clone(),
                            text3: feature.items[2].text.clone(),
                            title4: feature.items[3].title.clone(),
                            text4: feature.items[3].text.clone(),
                        }
                    },
                    FeatureBlock::Benefits(feature) => rsx! {
                        Benefits {
                            title: feature.title.clone(),
                            subtitle: feature.subtitle.clone(),
                            benefit1: feature.items[0].title.clone(),
                            benefit1_desc: feature.items[0].description.clone(),
                            benefit2: feature.items[1].title.clone(),
                            benefit2_desc: feature.items[1].description.clone(),
                            benefit3: feature.items[2].title.clone(),
                            benefit3_desc: feature.items[2].description.clone(),
                            class: None,
                        }
                    },
                }
            }

            if let Some(problem_solution) = &spec.problem_solution {
                ProblemSolution {
                    image: problem_solution.image.clone(),
                    title: problem_solution.title.clone(),
                    problem: problem_solution.problem.clone(),
                    solution: problem_solution.solution.clone(),
                    class: None,
                }
            }

            if let Some(process) = &spec.process {
                Features {
                    title: process.title.clone(),
                    description: process.description.clone(),
                    features: process.items.iter().map(|item| Feature {
                        title: item.title.clone(),
                        description: item.description.clone(),
                        icon: item.icon.clone(),
                    }).collect(),
                    class: None,
                }
            }

            match &spec.pricing_or_cta {
                PricingOrCtaBlock::Cta { title, subtitle, button_label, button_href } => rsx! {
                    section {
                        class: "text-center rounded-2xl p-8 md:p-12 bg-base-200",
                        h2 {
                            class: "text-3xl font-bold",
                            "{title}"
                        }
                        p {
                            class: "mt-4 text-lg opacity-80",
                            "{subtitle}"
                        }
                        div {
                            class: "mt-8",
                            a {
                                class: theme.buttons.primary_class.clone(),
                                href: button_href.clone(),
                                "{button_label}"
                            }
                        }
                    }
                },
            }

            if let Some(testimonials) = &spec.testimonials {
                Testamonials {
                    text1: testimonials.items[0].text.clone(),
                    job1: testimonials.items[0].job.clone(),
                    person1: testimonials.items[0].person.clone(),
                    img1: testimonials.items[0].image.clone(),
                    text2: testimonials.items[1].text.clone(),
                    job2: testimonials.items[1].job.clone(),
                    person2: testimonials.items[1].person.clone(),
                    img2: testimonials.items[1].image.clone(),
                    class: None,
                }
            }

            if let Some(faq) = &spec.faq {
                section {
                    h1 {
                        class: "text-3xl font-medium font-display title-font mb-12 text-center",
                        "{faq.title}"
                    }
                    Faq {
                        questions: faq.items.iter().map(|item| FaqText {
                            question: item.question.clone(),
                            answer: item.answer.clone(),
                        }).collect(),
                        class: None,
                    }
                }
            }
        }
    }
}
