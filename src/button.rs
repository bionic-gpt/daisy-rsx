#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl Display for ButtonScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonScheme::Neutral => write!(f, "btn-neutral"),
            ButtonScheme::Primary => write!(f, "btn-primary"),
            ButtonScheme::Secondary => write!(f, "btn-secondary"),
            ButtonScheme::Accent => write!(f, "btn-accent"),
            ButtonScheme::Info => write!(f, "btn-info"),
            ButtonScheme::Success => write!(f, "btn-success"),
            ButtonScheme::Warning => write!(f, "btn-warning"),
            ButtonScheme::Error => write!(f, "btn-error"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    Link,
    #[default]
    Button,
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonType::Submit => write!(f, "submit"),
            ButtonType::Reset => write!(f, "reset"),
            ButtonType::Button => write!(f, "button"),
            ButtonType::Link => write!(f, "button"),
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

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonShape {
    #[default]
    None,
    Circle,
    Square,
}

impl Display for ButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonShape::None => write!(f, ""),
            ButtonShape::Circle => write!(f, "btn-circle"),
            ButtonShape::Square => write!(f, "btn-square"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    #[default]
    None,
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

impl Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonStyle::None => write!(f, ""),
            ButtonStyle::Outline => write!(f, "btn-outline"),
            ButtonStyle::Dash => write!(f, "btn-dash"),
            ButtonStyle::Soft => write!(f, "btn-soft"),
            ButtonStyle::Ghost => write!(f, "btn-ghost"),
            ButtonStyle::Link => write!(f, "btn-link"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    children: Element,
    id: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    href: Option<String>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
    button_type: Option<ButtonType>,
    button_size: Option<ButtonSize>,
    button_scheme: Option<ButtonScheme>,
    popover_target: Option<String>,
    popover_target_action: Option<String>,
    disabled_text: Option<String>,
    button_shape: Option<ButtonShape>,
    button_style: Option<ButtonStyle>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_scheme = props.button_scheme.unwrap_or_default();
    let button_type = props.button_type.unwrap_or_default();
    let button_size = props.button_size.unwrap_or_default();
    let button_shape = props.button_shape.unwrap_or_default();
    let button_style = props.button_style.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled.filter(|&x| x);


    if props.button_type == Some(ButtonType::Link) {
        rsx!(
            a {
                class: "btn {class} {button_scheme} {button_size} {button_shape} {button_style}",
                id: props.id,
                href: props.href,
                if let Some(img_src) = props.prefix_image_src {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "12"
                        }
                },
                {props.children},
                if let Some(img_src) = props.suffix_image_src {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "12"
                        }
                }
            }
        )
    } else {
        rsx!(
            button {
                class: "btn {class} {button_scheme} {button_size} {button_shape} {button_style}",
                id: props.id,
                disabled,
                // We wanted to use popover but doesnt seem to woek with daisy modals
                "data-modal-target": props.popover_target,
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
}

#[test]
fn test_button() {
    let props = ButtonProps {
        children: rsx!( "Hello" ),
        class: Some("test".to_string()),
        href: None,
        button_scheme: Some(ButtonScheme::Primary),
        button_size: Some(ButtonSize::Large),
        button_type: Some(ButtonType::Button),
        button_shape: None,
        button_style: None,
        id: Some("id".to_string()),
        disabled: Some(false),
        prefix_image_src: None,
        suffix_image_src: None,
        disabled_text: None,
        popover_target: None,
        popover_target_action: None,
    };

    let expected =
        r#"<button class="btn test btn-primary btn-lg  " id="id" type="button">Hello</button>"#;
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
        href: None,
        button_scheme: Some(ButtonScheme::Primary),
        button_size: Some(ButtonSize::Large),
        button_type: Some(ButtonType::Button),
        button_shape: None,
        button_style: None,
        id: Some("id".to_string()),
        disabled: Some(false),
        prefix_image_src: Some("prefix.png".to_string()),
        suffix_image_src: Some("suffix.png".to_string()),
        disabled_text: None,
        popover_target: None,
        popover_target_action: None,
    };

    let expected = r#"<button class="btn test btn-primary btn-lg  " id="id" type="button"><img src="prefix.png" class="mr-2" width="12"/>Hello<img src="suffix.png" class="ml-2" width="12"/></button>"#;
    let result = dioxus_ssr::render_element(Button(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

// test all button schemes
#[test]
fn test_all_button_schemes() {
    let schemes = [
        (ButtonScheme::Neutral, "btn-neutral"),
        (ButtonScheme::Primary, "btn-primary"),
        (ButtonScheme::Secondary, "btn-secondary"),
        (ButtonScheme::Accent, "btn-accent"),
        (ButtonScheme::Info, "btn-info"),
        (ButtonScheme::Success, "btn-success"),
        (ButtonScheme::Warning, "btn-warning"),
        (ButtonScheme::Error, "btn-error"),
    ];

    for (scheme, expected_class) in schemes {
        let props = ButtonProps {
            children: rsx!( "Test" ),
            class: None,
            href: None,
            button_scheme: Some(scheme),
            button_size: None,
            button_type: None,
            button_shape: None,
            button_style: None,
            id: None,
            disabled: None,
            prefix_image_src: None,
            suffix_image_src: None,
            disabled_text: None,
            popover_target: None,
            popover_target_action: None,
        };

        let result = dioxus_ssr::render_element(Button(props));
        assert!(result.contains(expected_class),
                "Expected '{}' to contain '{}', but got: {}",
                result, expected_class, result);
    }
}

// test default button scheme
#[test]
fn test_default_button_scheme() {
    let props = ButtonProps {
        children: rsx!( "Default" ),
        class: None,
        href: None,
        button_scheme: None, // Should use default (Neutral)
        button_size: None,
        button_type: None,
        button_shape: None,
        button_style: None,
        id: None,
        disabled: None,
        prefix_image_src: None,
        suffix_image_src: None,
        disabled_text: None,
        popover_target: None,
        popover_target_action: None,
    };

    let result = dioxus_ssr::render_element(Button(props));
    assert!(result.contains("btn-neutral"),
            "Expected default scheme to be 'btn-neutral', but got: {}", result);
}
