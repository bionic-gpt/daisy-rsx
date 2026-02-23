#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SectionSpacing {
    pub section_gap: String,
    pub page_container: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SurfaceStyle {
    pub badge_class: String,
    pub card_class: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ButtonStyle {
    pub primary_class: String,
    pub secondary_class: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MarketingTheme {
    pub name: String,
    pub section_spacing: SectionSpacing,
    pub surface: SurfaceStyle,
    pub buttons: ButtonStyle,
    pub title_class: String,
    pub subtitle_class: String,
    pub motion: MotionTokens,
}

pub fn bionic_theme() -> MarketingTheme {
    MarketingTheme {
        name: "bionic".to_string(),
        section_spacing: SectionSpacing {
            section_gap: "grid gap-y-36".to_string(),
            page_container: "px-4 md:px-0 w-full lg:max-w-5xl mt-16 md:mt-36 mx-auto".to_string(),
        },
        surface: SurfaceStyle {
            badge_class: "badge badge-outline".to_string(),
            card_class: "bg-base-200".to_string(),
        },
        buttons: ButtonStyle {
            primary_class: "btn btn-primary".to_string(),
            secondary_class: "btn btn-secondary".to_string(),
        },
        title_class: "font-display text-2xl md:text-6xl font-bold".to_string(),
        subtitle_class: "py-6 text-base md:text-lg".to_string(),
        motion: MotionTokens {
            duration_fast_ms: 220,
            duration_base_ms: 520,
            duration_slow_ms: 760,
            easing_enter: "cubic-bezier(0.22, 1, 0.36, 1)".to_string(),
            easing_emphasis: "cubic-bezier(0.34, 1.56, 0.64, 1)".to_string(),
            stagger_ms: 90,
            travel_px: 20,
        },
    }
}

pub fn decision_theme() -> MarketingTheme {
    MarketingTheme {
        name: "decision".to_string(),
        section_spacing: SectionSpacing {
            section_gap: "grid gap-y-28 lg:gap-y-36".to_string(),
            page_container: "mx-auto w-full max-w-5xl px-6 pt-32 pb-20 lg:pt-40".to_string(),
        },
        surface: SurfaceStyle {
            badge_class: "badge badge-outline".to_string(),
            card_class: "bg-base-200".to_string(),
        },
        buttons: ButtonStyle {
            primary_class: "btn btn-primary".to_string(),
            secondary_class: "btn btn-secondary".to_string(),
        },
        title_class: "text-4xl sm:text-5xl font-bold".to_string(),
        subtitle_class: "mt-6 text-lg opacity-80".to_string(),
        motion: MotionTokens {
            duration_fast_ms: 240,
            duration_base_ms: 560,
            duration_slow_ms: 820,
            easing_enter: "cubic-bezier(0.22, 1, 0.36, 1)".to_string(),
            easing_emphasis: "cubic-bezier(0.34, 1.56, 0.64, 1)".to_string(),
            stagger_ms: 100,
            travel_px: 26,
        },
    }
}

pub fn deploy_theme() -> MarketingTheme {
    MarketingTheme {
        name: "deploy".to_string(),
        section_spacing: SectionSpacing {
            section_gap: "grid gap-y-28 lg:gap-y-32".to_string(),
            page_container: "mx-auto w-full max-w-6xl px-6 pt-24 pb-20 lg:pt-32".to_string(),
        },
        surface: SurfaceStyle {
            badge_class: "badge badge-outline".to_string(),
            card_class: "bg-base-200".to_string(),
        },
        buttons: ButtonStyle {
            primary_class: "btn btn-primary".to_string(),
            secondary_class: "btn btn-secondary".to_string(),
        },
        title_class: "font-display text-3xl md:text-5xl font-bold".to_string(),
        subtitle_class: "py-6 text-base md:text-lg opacity-80".to_string(),
        motion: MotionTokens {
            duration_fast_ms: 220,
            duration_base_ms: 500,
            duration_slow_ms: 740,
            easing_enter: "cubic-bezier(0.22, 1, 0.36, 1)".to_string(),
            easing_emphasis: "cubic-bezier(0.34, 1.56, 0.64, 1)".to_string(),
            stagger_ms: 80,
            travel_px: 18,
        },
    }
}
use crate::marketing::motion::MotionTokens;
