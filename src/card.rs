#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BoxProps {
    class: Option<String>,
    children: Element,
    drawer_trigger: Option<String>,
    modal_trigger: Option<String>,
}

pub fn Box(props: BoxProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("card {}", class);

    rsx!(
        div {
            class: "{class}",
            "data-drawer-target": props.drawer_trigger,
            "data-modal-target": props.modal_trigger,
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct BoxHeadersProps {
    class: Option<String>,
    title: String,
    children: Element,
}

pub fn BoxHeader(props: BoxHeadersProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("card-header flex items-center {}", class);

    rsx!(
        div {
            class: "{class}",
            h3 {
                class: "card-title overflow-hidden",
                "{props.title}"
            }
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct BoxBodyProps {
    class: Option<String>,
    children: Element,
}

pub fn BoxBody(props: BoxBodyProps) -> Element {
    let class = if let Some(class) = props.class {
        format!("card-body {}", class)
    } else {
        "card-body".to_string()
    };

    rsx!(
        div {
            class: "{class}",
            {props.children}
        }
    )
}
