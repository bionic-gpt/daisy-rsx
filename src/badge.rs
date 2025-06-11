#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BadgeStyle {
    #[default]
    None,
    Outline,
    Dash,
    Soft,
    Ghost,
}

impl Display for BadgeStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeStyle::None => write!(f, ""),
            BadgeStyle::Outline => write!(f, "badge-outline"),
            BadgeStyle::Dash => write!(f, "badge-dash"),
            BadgeStyle::Soft => write!(f, "badge-soft"),
            BadgeStyle::Ghost => write!(f, "badge-ghost"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BadgeColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for BadgeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeColor::Default => write!(f, ""),
            BadgeColor::Neutral => write!(f, "badge-neutral"),
            BadgeColor::Primary => write!(f, "badge-primary"),
            BadgeColor::Secondary => write!(f, "badge-secondary"),
            BadgeColor::Accent => write!(f, "badge-accent"),
            BadgeColor::Info => write!(f, "badge-info"),
            BadgeColor::Success => write!(f, "badge-success"),
            BadgeColor::Warning => write!(f, "badge-warning"),
            BadgeColor::Error => write!(f, "badge-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BadgeSize {
    #[default]
    Md,
    Xs,
    Sm,
    Lg,
    Xl,
}

impl Display for BadgeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadgeSize::Md => write!(f, "badge-md"),
            BadgeSize::Xs => write!(f, "badge-xs"),
            BadgeSize::Sm => write!(f, "badge-sm"),
            BadgeSize::Lg => write!(f, "badge-lg"),
            BadgeSize::Xl => write!(f, "badge-xl"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    children: Element,
    class: Option<String>,
    badge_style: Option<BadgeStyle>,
    badge_color: Option<BadgeColor>,
    badge_size: Option<BadgeSize>,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let badge_style = props.badge_style.unwrap_or_default();
    let badge_color = props.badge_color.unwrap_or_default();
    let badge_size = props.badge_size.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    rsx!(
        span { class: "badge {badge_style} {badge_color} {badge_size} {class}", {props.children} }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge() {
        let props = BadgeProps {
            children: rsx!("Hello"),
            class: Some("custom".to_string()),
            badge_style: Some(BadgeStyle::Outline),
            badge_color: Some(BadgeColor::Primary),
            badge_size: Some(BadgeSize::Lg),
        };
        let expected =
            r#"<span class="badge badge-outline badge-primary badge-lg custom">Hello</span>"#;
        let result = dioxus_ssr::render_element(Badge(props));
        assert_eq!(result, expected);
    }
}
