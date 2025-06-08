#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Default,
    Primary,
    Outline,
    Danger,
}

impl Display for ButtonScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonScheme::Default => write!(f, "btn-default"),
            ButtonScheme::Primary => write!(f, "btn-primary"),
            ButtonScheme::Outline => write!(f, "btn-outline"),
            ButtonScheme::Danger => write!(f, "btn-warning"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    #[default]
    Button,
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonType::Submit => write!(f, "submit"),
            ButtonType::Reset => write!(f, "reset"),
            ButtonType::Button => write!(f, "button"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonSize::Default => write!(f, "btn-sm"),
            ButtonSize::ExtraSmall => write!(f, "btn-xs"),
            ButtonSize::Small => write!(f, "btn-sm"),
            ButtonSize::Medium => write!(f, "btn-md"),
            ButtonSize::Large => write!(f, "btn-lg"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    children: Element,
    id: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
    button_type: Option<ButtonType>,
    button_size: Option<ButtonSize>,
    button_scheme: Option<ButtonScheme>,
    popover_target: Option<String>,
    popover_target_action: Option<String>,
    disabled_text: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_scheme = props.button_scheme.unwrap_or_default();
    let button_type = props.button_type.unwrap_or_default();
    let button_size = props.button_size.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);

    rsx!(
        button {
            class: "btn {class} {button_scheme} {button_size}",
            id: props.id,
            disabled,
            popovertarget: props.popover_target,
            popovertargetaction: props.popover_target_action,
            "type": "{button_type}",
            "data-disabled-text": props.disabled_text,
            if let Some(img_src) = props.prefix_image_src {
                img { src: "{img_src}", class: "mr-2", width: "12" }
            }
            {props.children}
            if let Some(img_src) = props.suffix_image_src {
                img { src: "{img_src}", class: "ml-2", width: "12" }
            }
        }
    )
}

#[test]
fn test_button() {
    let props = ButtonProps {
        children: rsx!( "Hello" ),
        class: Some("test".to_string()),
        button_scheme: Some(ButtonScheme::Primary),
        button_size: Some(ButtonSize::Large),
        button_type: Some(ButtonType::Button),
        id: Some("id".to_string()),
        disabled: Some(false),
        prefix_image_src: None,
        suffix_image_src: None,
        drawer_trigger: None,
        modal_trigger: None,
        disabled_text: None,
    };

    let expected =
        r#"<button class="btn test btn-primary btn-lg" id="id" type="button">Hello</button>"#;
    let result = dioxus_ssr::render_element(Button(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

// test button with images
#[test]
fn test_button_with_images() {
    let props = ButtonProps {
        children: rsx!( "Hello" ),
        class: Some("test".to_string()),
        button_scheme: Some(ButtonScheme::Primary),
        button_size: Some(ButtonSize::Large),
        button_type: Some(ButtonType::Button),
        id: Some("id".to_string()),
        disabled: Some(false),
        prefix_image_src: Some("prefix.png".to_string()),
        suffix_image_src: Some("suffix.png".to_string()),
        drawer_trigger: None,
        modal_trigger: None,
        disabled_text: None,
    };

    let expected = r#"<button class="btn test btn-primary btn-lg" id="id" type="button"><img src="prefix.png" class="mr-2" width="12"/>Hello<img src="suffix.png" class="ml-2" width="12"/></button>"#;
    let result = dioxus_ssr::render_element(Button(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
