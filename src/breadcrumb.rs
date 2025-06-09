#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub text: String,
    pub href: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    items: Vec<BreadcrumbItem>,
    class: Option<String>,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx!(
        div {
            class: "breadcrumbs text-sm {class}",
            ul {
                for item in props.items {
                    li {
                        if let Some(href) = &item.href {
                            a { href: "{href}", "{item.text}" }
                        } else {
                            "{item.text}"
                        }
                    }
                }
            }
        }
    )
}

#[test]
fn test_breadcrumb_basic() {
    let items = vec![
        BreadcrumbItem {
            text: "Home".to_string(),
            href: Some("/".to_string()),
        },
        BreadcrumbItem {
            text: "Documents".to_string(),
            href: Some("/documents".to_string()),
        },
        BreadcrumbItem {
            text: "Add Document".to_string(),
            href: None,
        },
    ];

    let props = BreadcrumbProps {
        items,
        class: None,
    };

    let expected = r#"<div class="breadcrumbs text-sm "><ul><li><a href="/">Home</a></li><li><a href="/documents">Documents</a></li><li>Add Document</li></ul></div>"#;
    let result = dioxus_ssr::render_element(Breadcrumb(props));
    assert_eq!(result, expected);
}

#[test]
fn test_breadcrumb_with_custom_class() {
    let items = vec![
        BreadcrumbItem {
            text: "Home".to_string(),
            href: Some("/".to_string()),
        },
        BreadcrumbItem {
            text: "Current".to_string(),
            href: None,
        },
    ];

    let props = BreadcrumbProps {
        items,
        class: Some("my-custom-class".to_string()),
    };

    let expected = r#"<div class="breadcrumbs text-sm my-custom-class"><ul><li><a href="/">Home</a></li><li>Current</li></ul></div>"#;
    let result = dioxus_ssr::render_element(Breadcrumb(props));
    assert_eq!(result, expected);
}

#[test]
fn test_breadcrumb_empty() {
    let props = BreadcrumbProps {
        items: vec![],
        class: None,
    };

    let expected = r#"<div class="breadcrumbs text-sm "><ul></ul></div>"#;
    let result = dioxus_ssr::render_element(Breadcrumb(props));
    assert_eq!(result, expected);
}

#[test]
fn test_breadcrumb_only_links() {
    let items = vec![
        BreadcrumbItem {
            text: "Home".to_string(),
            href: Some("/".to_string()),
        },
        BreadcrumbItem {
            text: "About".to_string(),
            href: Some("/about".to_string()),
        },
    ];

    let props = BreadcrumbProps {
        items,
        class: None,
    };

    let expected = r#"<div class="breadcrumbs text-sm "><ul><li><a href="/">Home</a></li><li><a href="/about">About</a></li></ul></div>"#;
    let result = dioxus_ssr::render_element(Breadcrumb(props));
    assert_eq!(result, expected);
}

#[test]
fn test_breadcrumb_only_text() {
    let items = vec![
        BreadcrumbItem {
            text: "Step 1".to_string(),
            href: None,
        },
        BreadcrumbItem {
            text: "Step 2".to_string(),
            href: None,
        },
    ];

    let props = BreadcrumbProps {
        items,
        class: None,
    };

    let expected = r#"<div class="breadcrumbs text-sm "><ul><li>Step 1</li><li>Step 2</li></ul></div>"#;
    let result = dioxus_ssr::render_element(Breadcrumb(props));
    assert_eq!(result, expected);
}