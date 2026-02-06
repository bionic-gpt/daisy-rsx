#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RangeColor {
    #[default]
    Default,
    Warn,
    Info,
    Error,
    Success,
}

impl Display for RangeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeColor::Default => write!(f, ""),
            RangeColor::Info => write!(f, "range-info"),
            RangeColor::Warn => write!(f, "range-warning"),
            RangeColor::Error => write!(f, "range-error"),
            RangeColor::Success => write!(f, "range-success"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RangeSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for RangeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeSize::Default => write!(f, "range-sm"),
            RangeSize::ExtraSmall => write!(f, "range-xs"),
            RangeSize::Small => write!(f, "range-sm"),
            RangeSize::Large => write!(f, "range-lg"),
            RangeSize::Medium => write!(f, "range-md"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RangeProps {
    children: Element,
    class: Option<String>,
    min: i32,
    max: i32,
    value: i32,
    name: String,
    label: Option<String>,
    label_class: Option<String>,
    help_text: Option<String>,
    range_color: Option<RangeColor>,
    step: Option<i32>,
}

#[component]
pub fn Range(props: RangeProps) -> Element {
    let range_color = props.range_color.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    rsx!(
        match props.label {
            Some(l) => rsx! {
                label { class: props.label_class, "{l}" }
            },
            None => rsx! {},
        }
        input {
            "type": "range",
            min: "{props.min}",
            max: "{props.max}",
            value: "{props.value}",
            step: props.step,
            class: "range {range_color} {class}",
            name: props.name,
            {props.children}
        }
        match props.help_text {
            Some(l) => rsx! {
                label {
                    span { class: "label-text-alt", "{l}" }
                }
            },
            None => rsx! {},
        }
    )
}

#[test]
fn test_range() {
    let props = RangeProps {
        children: rsx!( "Hello" ),
        class: Some("test".to_string()),
        range_color: Some(RangeColor::Info),
        min: 0,
        max: 100,
        value: 50,
        step: Some(10),
        name: "test".to_string(),
        label: Some("test".to_string()),
        label_class: Some("test".to_string()),
        help_text: Some("test".to_string()),
    };

    let expected = r#"<label class="test">test</label><input type="range" min="0" max="100" value="50" step=10 class="range range-info test" name="test">Hello</input><label><span class="label-text-alt">test</span></label>"#;
    let result = dioxus_ssr::render_element(Range(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

#[test]
fn test_range_default() {
    let props = RangeProps {
        children: rsx!( "Hello" ),
        class: None,
        range_color: None,
        min: 0,
        max: 100,
        value: 50,
        step: None,
        name: "test".to_string(),
        label: None,
        label_class: None,
        help_text: None,
    };

    let expected = r#"<input type="range" min="0" max="100" value="50" class="range  " name="test">Hello</input>"#;
    let result = dioxus_ssr::render_element(Range(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
