#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimeLineProps {
    condensed: Option<bool>,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TimeLine(props: TimeLineProps) -> Element {
    let condensed = match props.condensed {
        Some(true) => "timeline-condensed",
        _ => "",
    };

    rsx!(
        div { class: "{condensed} timeline-item {props.class.clone().unwrap_or_default()}",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TimeLineBadgeProps {
    image_src: String,
    class: Option<String>,
}

#[component]
pub fn TimeLineBadge(props: TimeLineBadgeProps) -> Element {
    rsx!(
        div { class: "timeline-badge {props.class.clone().unwrap_or_default()}",
            img { src: "{props.image_src}", width: "16" }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TimeLineBodyProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn TimeLineBody(props: TimeLineBodyProps) -> Element {
    rsx!(
        div { class: "timeline-body {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}
