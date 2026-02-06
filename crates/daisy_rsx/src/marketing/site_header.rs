#![allow(non_snake_case)]

use dioxus::prelude::*;

pub type SiteHeader = Element;

pub fn SiteHeader(children: Element, class: Option<String>) -> Element {
    let class = class.unwrap_or_default();
    rsx!(
        div { class: "{class}",
            {children}
        }
    )
}
