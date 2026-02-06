#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FieldsetProps {
    legend: String,
    children: Element,
    class: Option<String>,
    legend_class: Option<String>,
    help_text: Option<String>,
}

#[component]
pub fn Fieldset(props: FieldsetProps) -> Element {
    let class = props.class.unwrap_or_default();
    let legend_class = props.legend_class.unwrap_or_default();

    rsx!(
        fieldset { class: "fieldset {class}",
            legend { class: "fieldset-legend {legend_class}", "{props.legend}" }
            {props.children}
            match props.help_text {
                Some(help) => rsx!( p { class: "label", "{help}" } ),
                None => rsx!(),
            }
        }
    )
}
