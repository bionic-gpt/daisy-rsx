#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TabContainerProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TabContainer(props: TabContainerProps) -> Element {
    rsx!(
        div {
            role: "tablist",
            class: "tabs tabs-border {props.class.clone().unwrap_or_default()}",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TabPanelProps {
    name: String,
    checked: Option<bool>,
    tab_name: String,
    children: Element,
}

#[component]
pub fn TabPanel(props: TabPanelProps) -> Element {
    rsx!(
        input {
            checked: props.checked,
            "type": "radio",
            class: "tab",
            "aria-label": props.tab_name,
            name: props.name,
        }
        div { role: "tabpanel", class: "tab-content", {props.children} }
    )
}
