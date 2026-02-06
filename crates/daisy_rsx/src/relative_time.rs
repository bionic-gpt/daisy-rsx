#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RelativeTimeFormat {
    Datetime,
    #[default]
    Relative,
    Duration,
    Auto,
    Micro,
    Elapsed,
}

impl Display for RelativeTimeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeTimeFormat::Datetime => write!(f, "datetime"),
            RelativeTimeFormat::Relative => write!(f, "relative"),
            RelativeTimeFormat::Duration => write!(f, "duration"),
            RelativeTimeFormat::Auto => write!(f, "auto"),
            RelativeTimeFormat::Micro => write!(f, "micro"),
            RelativeTimeFormat::Elapsed => write!(f, "elapsed"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RelativeTimeProps {
    format: Option<RelativeTimeFormat>,
    datetime: String,
}

#[component]
pub fn RelativeTime(props: RelativeTimeProps) -> Element {
    let format = props.format.unwrap_or_default();

    rsx!(
        relative-time { datetime: props.datetime, format: format.to_string() }
    )
}

#[test]
fn test_relative_time() {
    let props = RelativeTimeProps {
        datetime: "2024-01-01".to_string(),
        format: Some(RelativeTimeFormat::Datetime),
    };

    let expected = r#"<relative-time datetime="2024-01-01" format="datetime"></relative-time>"#;
    let result = dioxus_ssr::render_element(RelativeTime(props));
    // println!("{}", result);
    assert_eq!(expected, result);
}
