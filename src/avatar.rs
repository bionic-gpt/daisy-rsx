#![allow(non_snake_case)]
#![allow(unused_braces)]
use dioxus::prelude::*;

/// DaisyUI color pairs for letter avatars. Each tuple contains the background
/// color variable and its matching foreground color.
const AVATAR_COLORS: [(&str, &str); 8] = [
    (
        "hsl(var(--color-primary))",
        "hsl(var(--color-primary-content))",
    ),
    (
        "hsl(var(--color-secondary))",
        "hsl(var(--color-secondary-content))",
    ),
    (
        "hsl(var(--color-accent))",
        "hsl(var(--color-accent-content))",
    ),
    (
        "hsl(var(--color-neutral))",
        "hsl(var(--color-neutral-content))",
    ),
    ("hsl(var(--color-info))", "hsl(var(--color-info-content))"),
    (
        "hsl(var(--color-success))",
        "hsl(var(--color-success-content))",
    ),
    (
        "hsl(var(--color-warning))",
        "hsl(var(--color-warning-content))",
    ),
    (
        "hsl(var(--color-error))",
        "hsl(var(--color-error-content))",
    ),
];

fn letter_colors(ch: char) -> (&'static str, &'static str) {
    let idx = (ch as usize) % AVATAR_COLORS.len();
    AVATAR_COLORS[idx]
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AvatarType {
    Team,
    #[default]
    User,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AvatarSize {
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl AvatarSize {
    pub fn to_string(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            AvatarSize::Small => ("24", "24", "w-8 h-8"),
            AvatarSize::Medium => ("48", "48", "w-16 h-16"),
            AvatarSize::Large => ("96", "96", "w-20 w-20"),
            AvatarSize::ExtraLarge => ("128", "128", "w-32 h-32"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    avatar_size: Option<AvatarSize>,
    avatar_type: Option<AvatarType>,
    name: Option<String>,
    _email: Option<String>,
    image_src: Option<String>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let avatar_size = props.avatar_size.unwrap_or_default();

    let avatar_size = avatar_size.to_string();

    // Get the first character of the name, or "?" if the name is empty
    let the_name = props
        .name
        .as_deref()
        .and_then(|n| n.chars().next())
        .map_or("?".to_string(), |c| c.to_string());

    let first_char = the_name.chars().next().unwrap_or('?');
    let (bg_color, text_color) = letter_colors(first_char);

    if let Some(image) = props.image_src {
        rsx!(
            div { class: "avatar",
                div { class: "rounded {avatar_size.2}",
                    img {
                        width: avatar_size.0,
                        height: avatar_size.1,
                        src: image,
                    }
                }
            }
        )
    } else {
        match props.avatar_type {
            Some(AvatarType::User) => rsx!(
                div { class: "avatar",
                    div { class: "rounded {avatar_size.2}",
                        svg {
                            "aria-hidden": true,
                            xmlns: "http://www.w3.org/2000/svg",
                            height: avatar_size.0,
                            width: avatar_size.1,
                            "viewbox": "0 0 27 27",
                            rect {
                                fill: "rgb(125, 73, 193)",
                                height: "27",
                                rx: "12",
                                width: "27",
                                x: "0",
                                y: "0",
                            }
                            g { fill: "#fff", opacity: ".5",
                                circle { cx: "13.5", cy: "30", r: "13" }
                                circle { cx: "13.5", cy: "11", r: "5" }
                            }
                        }
                    }
                }
            ),
            Some(_) => rsx!(
                div { class: "avatar",
                    div { class: "rounded {avatar_size.2}",
                        svg {
                            "aria-hidden": true,
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 50 50",
                            height: avatar_size.0,
                            width: avatar_size.1,
                            rect {
                                fill: bg_color,
                                height: "100%",
                                width: "100%",
                            }
                            text {
                                fill: text_color,
                                "font-size": "26",
                                "font-weight": "500",
                                x: "50%",
                                y: "55%",
                                "dominant-baseline": "middle",
                                "text-anchor": "middle",
                                {the_name}
                            }
                        }
                    }
                }
            ),
            None => rsx!(
                div { class: "avatar",
                    div { class: "rounded {avatar_size.2}",
                        svg {
                            "aria-hidden": true,
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 50 50",
                            height: avatar_size.0,
                            width: avatar_size.1,
                            rect {
                                fill: bg_color,
                                height: "100%",
                                width: "100%",
                            }
                            text {
                                fill: text_color,
                                "font-size": "26",
                                "font-weight": "500",
                                x: "50%",
                                y: "55%",
                                "dominant-baseline": "middle",
                                "text-anchor": "middle",
                                {the_name}
                            }
                        }
                    }
                }
            ),
        }
    }
}
