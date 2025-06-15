#![allow(non_snake_case)]
#![allow(unused_braces)]

use std::fmt::Display;
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimelineDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl Display for TimelineDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimelineDirection::Horizontal => write!(f, "timeline-horizontal"),
            TimelineDirection::Vertical => write!(f, "timeline-vertical"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    children: Element,
    class: Option<String>,
    direction: Option<TimelineDirection>,
    compact: Option<bool>,
    snap_icon: Option<bool>,
}

#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    let direction = props.direction.unwrap_or_default();
    let compact = if props.compact.unwrap_or(false) { "timeline-compact" } else { "" };
    let snap_icon = if props.snap_icon.unwrap_or(false) { "timeline-snap-icon" } else { "" };
    let class = props.class.unwrap_or_default();

    rsx!(
        ul { class: "timeline {direction} {compact} {snap_icon} {class}",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TimelineItemProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    rsx!(
        li { class: "{props.class.clone().unwrap_or_default()}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TimelinePartProps {
    children: Element,
    class: Option<String>,
    boxed: Option<bool>,
}

#[component]
pub fn TimelineStart(props: TimelinePartProps) -> Element {
    let boxed = if props.boxed.unwrap_or(false) { "timeline-box" } else { "" };
    rsx!(
        div { class: "timeline-start {boxed} {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}

#[component]
pub fn TimelineMiddle(props: TimelinePartProps) -> Element {
    rsx!(
        div { class: "timeline-middle {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}

#[component]
pub fn TimelineEnd(props: TimelinePartProps) -> Element {
    let boxed = if props.boxed.unwrap_or(false) { "timeline-box" } else { "" };
    rsx!(
        div { class: "timeline-end {boxed} {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}
