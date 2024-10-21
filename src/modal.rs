#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    trigger_id: String,
    children: Element,
    submit_action: Option<String>,
}

pub fn Modal(props: ModalProps) -> Element {
    rsx!(
        if let Some(action) = &props.submit_action {
            form {
                action: "{action}",
                method: "post",
                dialog {
                    class: "modal",
                    id: "{props.trigger_id}",
                    ModalBody {
                        {props.children}
                    }
                }
            }
        } else {
            dialog {
                class: "modal",
                id: "{props.trigger_id}",
                ModalBody {
                    {props.children}
                }
            }
        }
    )
}


#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    children: Element,
    class: Option<String>,
}

pub fn ModalBody(props: ModalBodyProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("modal-box {}", class);
    rsx!(
        div {
            class: "{class}",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalActionProps {
    children: Element,
    class: Option<String>,
}

pub fn ModalAction(props: ModalActionProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("modal-action {}", class);
    rsx!(
        div {
            class: "{class}",
            {props.children}
        }
    )
}
