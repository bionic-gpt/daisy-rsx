#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextAreaSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for TextAreaSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAreaSize::Default => write!(f, "textarea-sm"),
            TextAreaSize::Small => write!(f, "textarea-sm"),
            TextAreaSize::ExtraSmall => write!(f, "textarea-xs"),
            TextAreaSize::Large => write!(f, "textarea-lg"),
            TextAreaSize::Medium => write!(f, "textarea-md"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    children: Element,
    area_size: Option<TextAreaSize>,
    pub name: String,
    pub id: Option<String>,
    pub class: Option<String>,
    pub rows: Option<String>,
    pub label_class: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
    pub help_text: Option<String>,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

#[component]
pub fn TextArea(props: Props) -> Element {
    let input_size = props.area_size.unwrap_or_default();
    let class = format!("{} {}", props.class.unwrap_or_default(), input_size);
    let value = props.value.unwrap_or_default();
    let placeholder = props.placeholder.unwrap_or_default();
    let label_class = props.label_class.unwrap_or_default();

    let disabled = props.disabled.unwrap_or(false);

    rsx!(
        match props.label {
            Some(l) => rsx! {
                label { class: "{label_class}", "{l}" }
            },
            None => rsx! {},
        }
        textarea {
            id: props.id,
            class: "textarea textarea-bordered textarea-sm {class}",
            value: "{value}",
            name: "{props.name}",
            placeholder: "{placeholder}",
            required: props.required,
            disabled,
            readonly: props.readonly,
            rows: props.rows,
            {props.children}
        }
        match props.help_text {
            Some(l) => rsx! {
                span { class: "note mb-3", "{l}" }
            },
            None => rsx! {},
        }
    )
}

#[test]
fn test_text_area() {
    let props = Props {
        children: rsx! { "Hello" },
        area_size: Some(TextAreaSize::Default),
        name: "name".to_string(),
        id: Some("id".to_string()),
        class: Some("class".to_string()),
        rows: Some("rows".to_string()),
        label_class: Some("label_class".to_string()),
        value: Some("value".to_string()),
        label: Some("label".to_string()),
        help_text: Some("help_text".to_string()),
        placeholder: Some("placeholder".to_string()),
        required: Some(true),
        disabled: Some(false),
        readonly: Some(false),
    };

    let expected = r#"<label class="label_class">label</label><textarea id="id" class="textarea textarea-bordered textarea-sm class textarea-sm" value="value" name="name" placeholder="placeholder" required=true rows="rows">Hello</textarea><span class="note mb-3">help_text</span>"#;
    let result = dioxus_ssr::render_element(TextArea(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
