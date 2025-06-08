#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelRole {
    #[default]
    Default,
    Neutral,
    Danger,
    Warning,
    Success,
    Info,
    Highlight,
}

impl Display for LabelRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelRole::Neutral => write!(f, "badge-neutral"),
            LabelRole::Danger => write!(f, "badge-danger"),
            LabelRole::Warning => write!(f, "badge-warning"),
            LabelRole::Success => write!(f, "badge-success"),
            LabelRole::Info => write!(f, "badge-info"),
            LabelRole::Highlight => write!(f, "badge-highlight"),
            LabelRole::Default => write!(f, ""),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelSize {
    #[default]
    Small,
    Large,
}

impl Display for LabelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelSize::Small => write!(f, ""),
            LabelSize::Large => write!(f, "badge-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    children: Element,
    class: Option<String>,
    label_role: Option<LabelRole>,
    label_size: Option<LabelSize>,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    let label_role = props.label_role.unwrap_or_default();
    let label_size = props.label_size.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    rsx!(
        button { class: "badge {label_role} {label_size} {class}", {props.children} }
    )
}

#[test]
fn test_label() {
    let props = LabelProps {
        children: rsx!( "Hello" ),
        class: Some("test".to_string()),
        label_role: Some(LabelRole::Danger),
        label_size: Some(LabelSize::Large),
    };

    let expected = r#"<button class="badge badge-danger badge-lg test">Hello</button>"#;
    let result = dioxus_ssr::render_element(Label(props));
    // println!("{}", result);
    assert_eq!(result, expected);
}
