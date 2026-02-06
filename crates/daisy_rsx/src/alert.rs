#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AlertColor {
    #[default]
    Default,
    Warn,
    Info,
    Error,
    Success,
}

impl AlertColor {
    pub fn to_string(&self) -> &'static str {
        match self {
            AlertColor::Default => "alert alert-info",
            AlertColor::Info => "alert alert-info",
            AlertColor::Warn => "alert alert-warning",
            AlertColor::Error => "alert alert-error",
            AlertColor::Success => "alert alert-success",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    children: Element,
    class: Option<String>,
    alert_color: Option<AlertColor>,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let alert_color = props.alert_color.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    let class = format!("{} {}", alert_color.to_string(), class);

    rsx!(
        div { class: "{class}", {props.children} }
    )
}
