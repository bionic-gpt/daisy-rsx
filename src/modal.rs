#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    trigger_id: String,
    children: Element,
    submit_action: Option<String>,
    class: Option<String>,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    rsx!(
        if let Some(action) = &props.submit_action {
            form { action: "{action}", method: "post",
                dialog {
                    class: "modal {props.class.clone().unwrap_or_default()}",
                    id: "{props.trigger_id}",
                    popover: true,
                    {props.children}
                }
            }
        } else {
            dialog {
                class: "modal {props.class.clone().unwrap_or_default()}",
                id: "{props.trigger_id}",
                popover: true,
                {props.children}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn ModalBody(props: ModalBodyProps) -> Element {
    rsx!(
        div { class: "modal-box {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalActionProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn ModalAction(props: ModalActionProps) -> Element {
    rsx!(
        div { class: "modal-action {props.class.clone().unwrap_or_default()}", {props.children} }
    )
}

#[test]
fn test_modal() {
    let props = ModalProps {
        children: rsx!( "Hello" ),
        class: Some("test".to_string()),
        submit_action: Some("test".to_string()),
        trigger_id: "id".to_string(),
    };

    let expected = r#"<form action="test" method="post"><dialog class="modal test" id="id" popover=true>Hello</dialog></form>"#;
    let result = dioxus_ssr::render_element(Modal(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}

#[test]
fn test_modal_without_submit_action() {
    let props = ModalProps {
        children: rsx!( "Hello" ),
        class: Some("test".to_string()),
        submit_action: None,
        trigger_id: "id".to_string(),
    };

    let expected = r#"<dialog class="modal test" id="id" popover=true>Hello</dialog>"#;
    let result = dioxus_ssr::render_element(Modal(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
