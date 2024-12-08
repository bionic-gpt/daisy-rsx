#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    trigger_id: String,
    label: String,
    children: Element,
    submit_action: Option<String>,
}

#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    if let Some(action) = &props.submit_action {
        rsx!(
            form {
                action: "{action}",
                method: "post",
                div {
                    div {
                        class: "side-drawer flex flex-col",
                        id: props.trigger_id,
                        div {
                            class: "drawer__overlay",
                            tabindex: "-1"
                        }
                        div {
                            class: "drawer__panel",
                            header {
                                class: "drawer__header",
                                h4 {
                                    class: "drawer__title",
                                    "{props.label}"
                                }
                                a {
                                    href: "#",
                                    class: "drawer__close",
                                    "X"
                                }
                            }
                            {props.children}
                        }
                    }
                }
            }
        )
    } else {
        rsx!(
            div {
                div {
                    class: "side-drawer flex flex-col",
                    id: props.trigger_id,
                    div {
                        class: "drawer__overlay",
                        tabindex: "-1"
                    }
                    div {
                        class: "drawer__panel",
                        header {
                            class: "drawer__header",
                            h4 {
                                class: "drawer__title",
                                "{props.label}"
                            }
                            a {
                                href: "#",
                                class: "drawer__close",
                                "X"
                            }
                        }
                        {props.children}
                    }
                }
            }
        )
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DrawerFooterProps {
    children: Element,
}

#[component]
pub fn DrawerFooter(props: DrawerFooterProps) -> Element {
    rsx!(
        div {
            class: "drawer__footer",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct DrawerBodyProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn DrawerBody(props: DrawerBodyProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("drawer__body {}", class);
    rsx!(
        div {
            class: "{class}",
            {props.children}
        }
    )
}
