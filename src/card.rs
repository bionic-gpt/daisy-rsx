#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    class: Option<String>,
    children: Element,
    popover_target: Option<String>,
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
            "data-target": props.popover_target,
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
    rsx!(
        div { class: "card-header flex items-center {props.class.clone().unwrap_or_default()}",
            h3 { class: "card-title overflow-hidden", "{props.title}" }
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
    rsx!(
        div { class: "card-body {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}
