#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Number,
    Email,
    Password,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Number => write!(f, "number"),
            InputType::Email => write!(f, "email"),
            InputType::Password => write!(f, "password"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for InputSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputSize::Default => write!(f, "input-sm"),
            InputSize::ExtraSmall => write!(f, "input-xs"),
            InputSize::Small => write!(f, "input-sm"),
            InputSize::Large => write!(f, "input-lg"),
            InputSize::Medium => write!(f, "input-md"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    input_type: Option<InputType>,
    input_size: Option<InputSize>,
    pub name: String,
    pub id: Option<String>,
    pub label_class: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
    pub help_text: Option<String>,
    pub placeholder: Option<String>,
    pub step: Option<String>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let input_type = props.input_type.unwrap_or_default();
    let input_size = props.input_size.unwrap_or_default();

    rsx!(
        match (props.label, props.required) {
            (Some(l), Some(_)) => rsx! {
                label { class: props.label_class, "{l} *" }
            },
            (Some(l), None) => rsx! {
                label { class: props.label_class, "{l}" }
            },
            (None, _) => rsx! {},
        }
        input {
            id: props.id,
            class: "input input-bordered {input_size}",
            value: props.value,
            required: props.required,
            disabled: props.disabled,
            readonly: props.readonly,
            name: "{props.name}",
            placeholder: props.placeholder,
            step: props.step,
            "type": "{input_type}",
        }
        if let Some(l) = props.help_text {
            label {
                span { class: "label-text-alt", "{l}" }
            }
        }
    )
}
