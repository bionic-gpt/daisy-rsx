#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    class: Option<String>,
    children: Element,
    drawer_trigger: Option<String>,
    modal_trigger: Option<String>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
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
pub struct CardHeadersProps {
    class: Option<String>,
    title: String,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeadersProps) -> Element {
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
pub struct CardBodyProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
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
