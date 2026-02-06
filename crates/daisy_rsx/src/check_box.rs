#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxScheme {
    #[default]
    Default,
    Primary,
    Outline,
    Danger,
}

impl Display for CheckBoxScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckBoxScheme::Default => write!(f, "checkbox-default"),
            CheckBoxScheme::Primary => write!(f, "checkbox-primary"),
            CheckBoxScheme::Outline => write!(f, "checkbox-outline"),
            CheckBoxScheme::Danger => write!(f, "checkbox-warning"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for CheckBoxSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckBoxSize::Default => write!(f, "checkbox-sm"),
            CheckBoxSize::Small => write!(f, "checkbox-sm"),
            CheckBoxSize::ExtraSmall => write!(f, "checkbox-xs"),
            CheckBoxSize::Large => write!(f, "checkbox-lg"),
            CheckBoxSize::Medium => write!(f, "checkbox-md"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CheckBoxProps {
    children: Element,
    id: Option<String>,
    checked: Option<bool>,
    class: Option<String>,
    name: String,
    value: String,
    checkbox_size: Option<CheckBoxSize>,
    checkbox_scheme: Option<CheckBoxScheme>,
}

#[component]
pub fn CheckBox(props: CheckBoxProps) -> Element {
    let checkbox_scheme = props.checkbox_scheme.unwrap_or_default();
    let checkbox_size = props.checkbox_size.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    let checked = props
        .checked
        .and_then(|checked| checked.then_some("checked"));

    rsx!(
        input {
            "type": "checkbox",
            class: "checkbox {class} {checkbox_scheme} {checkbox_size}",
            id: props.id,
            name: props.name,
            value: props.value,
            checked,
            {props.children}
        }
    )
}

#[test]
fn test_check_box() {
    let props = CheckBoxProps {
        children: rsx!(),
        name: "name".to_string(),
        value: "value".to_string(),
        checked: Some(true),
        class: Some("custom".to_string()),
        checkbox_size: Some(CheckBoxSize::Large),
        checkbox_scheme: Some(CheckBoxScheme::Danger),
        id: Some("id".to_string()),
    };
    let expected = r#"<input type="checkbox" class="checkbox custom checkbox-warning checkbox-lg" id="id" name="name" value="value" checked="checked"></input>"#;
    let result = dioxus_ssr::render_element(CheckBox(props));
    // println!("{}", result);
    assert_eq!(result, expected);
}

#[test]
fn test_check_box_default() {
    let props = CheckBoxProps {
        children: rsx!(),
        name: "name".to_string(),
        value: "value".to_string(),
        checked: None,
        class: None,
        checkbox_size: None,
        checkbox_scheme: None,
        id: None,
    };
    let expected = r#"<input type="checkbox" class="checkbox  checkbox-default checkbox-sm" name="name" value="value"></input>"#;
    let result = dioxus_ssr::render_element(CheckBox(props));
    // println!("{}", result);
    assert_eq!(result, expected);
}

#[test]
fn test_check_box_checked_false() {
    let props = CheckBoxProps {
        children: rsx!(),
        name: "name".to_string(),
        value: "value".to_string(),
        checked: Some(false),
        class: None,
        checkbox_size: None,
        checkbox_scheme: None,
        id: None,
    };
    let expected = r#"<input type="checkbox" class="checkbox  checkbox-default checkbox-sm" name="name" value="value"></input>"#;
    let result = dioxus_ssr::render_element(CheckBox(props));
    // println!("{}", result);
    assert_eq!(result, expected);
}
