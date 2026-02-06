#![allow(non_snake_case)]
use super::button::{Button, ButtonScheme};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BlankSlateProps {
    heading: String,
    visual: String,
    description: String,
    primary_action: Option<(String, String)>,
    primary_action_drawer: Option<(String, String)>,
    secondary_action: Option<(String, String)>,
}

#[component]
pub fn BlankSlate(props: BlankSlateProps) -> Element {
    rsx!(
        div {
            class: "mt-4 flex flex-col justify-center items-center",
            img {
                class: "mb-4 svg-icon",
                src: "{props.visual}",
                width: "100px",
                height: "100px"
            }
            h2 {
                class: "text-center mb-4  max-w-prose",
                "{props.heading}"
            }
            p {
                class: "mb-4  max-w-prose text-center",
                "{props.description}"
            }
            match &props.primary_action {
                Some(pa) => rsx!(
                    div {
                        a {
                            href: "{pa.1}",
                            span {
                                class: "Button-label",
                                "{pa.0}"
                            }
                        }
                    }
                 ),
                None => rsx!()
            }
            match props.primary_action_drawer {
                Some(pa) => rsx!(
                    div {
                        Button {
                            button_scheme: ButtonScheme::Primary,
                            popover_target: "{pa.1}",
                            "{pa.0}"
                        }
                    }
                 ),
                None => rsx!()
            }
            match props.secondary_action {
                Some(pa) => rsx!(
                    div {
                        a {
                            href: "{pa.1}",
                            "{pa.0}"
                        }
                    }
                 ),
                None => rsx!()
            }
        }
    )
}
