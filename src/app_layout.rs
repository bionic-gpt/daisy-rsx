#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AppLayoutProps {
    title: String,
    fav_icon_src: Option<String>,
    stylesheets: Vec<String>,
    js_href: Option<String>,
    header: Element,
    children: Element,
    sidebar: Element,
    sidebar_footer: Element,
    sidebar_header: Element,
}

pub fn AppLayout(props: AppLayoutProps) -> Element {
    rsx!(
        head {
            title {
                "{props.title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
            for href in &props.stylesheets {
                link {
                    rel: "stylesheet",
                    href: "{href}",
                    "type": "text/css"
                }
            }
            if let Some(js_href) = props.js_href {
                script {
                    "type": "module",
                    src: "{js_href}"
                }
            }
            if let Some(fav_icon_src) = props.fav_icon_src {
                link {
                    rel: "icon",
                    "type": "image/svg+xml",
                    href: "{fav_icon_src}"
                }
            }
        }
        body {
            div {
                class: "flex h-screen overflow-hidden",
                nav {
                    id: "sidebar",
                    class: "
                        border-r border-base-300
                        fixed
                        bg-base-200
                        inset-y-0
                        left-0
                        w-64
                        transform
                        -translate-x-full
                        transition-transform
                        duration-200
                        ease-in-out
                        flex
                        flex-col
                        lg:translate-x-0
                        lg:static
                        lg:inset-auto
                        lg:transform-none
                        z-20",
                    div {
                        class: "flex items-center p-4",
                        {props.sidebar_header}
                    }
                    div {
                        class: "flex-1 overflow-y-auto",
                        {props.sidebar}
                    }
                    div {
                        class: "p-4",
                        {props.sidebar_footer}
                    }
                }
                main {
                    id: "main-content",
                    class: "flex-1 flex flex-col",
                    header {
                        class: "flex items-center p-4 border-b border-base-300",
                        button {
                            id: "toggleButton",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "lucide lucide-panel-left",
                                rect {
                                    width: "18",
                                    height: "18",
                                    x: "3",
                                    y: "3",
                                    rx: "2",
                                }
                                path {
                                    d: "M9 3v18",
                                }
                            }
                        }
                        {props.header}
                    }
                    section {
                        class: "flex-1 overflow-y-auto",
                        {props.children}
                    }
                }
            }
        }
    )
}