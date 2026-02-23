#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LandingPageSpec {
    pub hero: HeroBlock,
    pub proof: Option<ProofBlock>,
    pub feature_blocks: Vec<FeatureBlock>,
    pub problem_solution: Option<ProblemSolutionBlock>,
    pub process: Option<ProcessBlock>,
    pub pricing_or_cta: PricingOrCtaBlock,
    pub testimonials: Option<TestimonialsBlock>,
    pub faq: Option<FaqBlock>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HeroBlock {
    pub variant: HeroVariant,
    pub title: String,
    pub subtitle: String,
    pub cta_label: Option<String>,
    pub cta_href: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HeroVariant {
    Basic,
    Image { image: String },
    Video { video_id: String, claim: String },
    SplitVideo { video_src: String },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProofBlock {
    pub title: String,
    pub logos: Vec<LogoItem>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogoItem {
    pub src: String,
    pub alt: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FeatureBlock {
    SmallImage(SmallImageFeatureBlock),
    Image(ImageFeatureBlock),
    Quad(QuadFeatureBlock),
    Benefits(BenefitsBlock),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SmallImageFeatureBlock {
    pub title: String,
    pub sub_title: String,
    pub text: String,
    pub image: String,
    pub flip: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImageFeatureBlock {
    pub title: String,
    pub sub_title: String,
    pub image: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuadItem {
    pub title: String,
    pub text: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct QuadFeatureBlock {
    pub title: String,
    pub sub_title: String,
    pub text: String,
    pub items: [QuadItem; 4],
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BenefitsBlock {
    pub title: String,
    pub subtitle: String,
    pub items: [BenefitItem; 3],
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BenefitItem {
    pub title: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProblemSolutionBlock {
    pub title: String,
    pub problem: String,
    pub solution: String,
    pub image: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProcessBlock {
    pub title: String,
    pub description: String,
    pub items: Vec<ProcessItem>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProcessItem {
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PricingOrCtaBlock {
    Cta {
        title: String,
        subtitle: String,
        button_label: String,
        button_href: String,
    },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestimonialsBlock {
    pub title: String,
    pub items: [TestimonialItem; 2],
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestimonialItem {
    pub text: String,
    pub job: String,
    pub person: String,
    pub image: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FaqBlock {
    pub title: String,
    pub items: Vec<FaqItem>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FaqItem {
    pub question: String,
    pub answer: String,
}
