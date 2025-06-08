#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ToolTipColor {
    #[default]
    Default,
    Warn,
    Info,
    Error,
    Success,
}

impl Display for ToolTipColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolTipColor::Default => write!(f, ""),
            ToolTipColor::Info => write!(f, "tooltip-info"),
            ToolTipColor::Warn => write!(f, "tooltip-warning"),
            ToolTipColor::Error => write!(f, "tooltip-error"),
            ToolTipColor::Success => write!(f, "tooltip-success"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToolTipProps {
    text: String,
    children: Element,
    class: Option<String>,
    alert_color: Option<ToolTipColor>,
}

#[component]
pub fn ToolTip(props: ToolTipProps) -> Element {
    let alert_color = props.alert_color.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    rsx!(
        div { class: "tooltip {alert_color} {class}", "data-tip": props.text, {props.children} }
    )
}
