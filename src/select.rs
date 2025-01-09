#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SelectSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for SelectSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectSize::Default => write!(f, ""),
            SelectSize::Small => write!(f, "select-sm"),
            SelectSize::ExtraSmall => write!(f, "select-xs"),
            SelectSize::Large => write!(f, "select-lg"),
            SelectSize::Medium => write!(f, "select-md"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    children: Element,
    select_size: Option<SelectSize>,
    pub name: String,
    pub id: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
    pub label_class: Option<String>,
    pub help_text: Option<String>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub multiple: Option<bool>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let select_size = props.select_size.unwrap_or_default();
    let value = props.value.unwrap_or_default();
    let disabled = props.disabled.filter(|&d| d);

    rsx!(
        match props.label {
            Some(l) => rsx! {
                label { class: props.label_class, "{l}" }
            },
            None => rsx! {},
        }
        select {
            id: props.id,
            required: props.required,
            disabled,
            multiple: props.multiple,
            class: "select select-bordered {select_size}",
            value: "{value}",
            name: "{props.name}",
            {props.children}
        }
        match props.help_text {
            Some(l) => rsx! {
                label { class: "label-text-alt",
                    span { "{l}" }
                }
            },
            None => rsx! {},
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct OptionProps {
    children: Element,
    pub value: String,
    pub selected_value: Option<String>,
}

#[component]
pub fn SelectOption(props: OptionProps) -> Element {
    rsx!(
        option {
            value: props.value.clone(),
            selected: props.selected_value.as_ref().map_or(false, |s| s == &props.value),
            {props.children}
        }
    )
}

#[test]
fn test_select_option() {
    let props = OptionProps {
        value: "test".to_string(),
        selected_value: Some("test".to_string()),
        children: rsx!( "Hello" ),
    };

    let expected = r#"<option value="test" selected=true>Hello</option>"#;
    let result = dioxus_ssr::render_element(SelectOption(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

#[test]
fn test_select() {
    let props = SelectProps {
        children: rsx! {
            SelectOption {
                value: "test".to_string(),
                selected_value: Some("test".to_string()),
                children: rsx! { "Hello" },
            }
            SelectOption {
                value: "test2".to_string(),
                selected_value: Some("test".to_string()),
                children: rsx! { "Hello2" },
            }
        },
        select_size: Some(SelectSize::Large),
        name: "test".to_string(),
        id: Some("test".to_string()),
        value: Some("test".to_string()),
        label: Some("test".to_string()),
        label_class: Some("test".to_string()),
        help_text: Some("test".to_string()),
        required: Some(true),
        disabled: Some(false),
        multiple: Some(false),
    };

    let expected = r#"<label class="test">test</label><select id="test" required=true class="select select-bordered select-lg" value="test" name="test"><option value="test" selected=true>Hello</option><option value="test2">Hello2</option></select><label class="label-text-alt"><span>test</span></label>"#;
    let result = dioxus_ssr::render_element(Select(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
