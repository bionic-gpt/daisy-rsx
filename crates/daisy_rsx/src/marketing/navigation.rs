use crate::marketing::site_header::SiteHeader;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Section {
    None,
    Home,
    Enterprise,
    Partners,
    McpServers,
    Pricing,
    Blog,
    Docs,
    ArchitectCourse,
    Contact,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NavigationLink {
    pub href: String,
    pub label: String,
    pub section: Section,
    pub class: Option<String>,
    pub hx_boost: bool,
    pub badge_image: Option<String>,
    pub badge_alt: Option<String>,
}

impl NavigationLink {
    pub fn new(label: impl Into<String>, href: impl Into<String>, section: Section) -> Self {
        Self {
            href: href.into(),
            label: label.into(),
            section,
            class: None,
            hx_boost: true,
            badge_image: None,
            badge_alt: None,
        }
    }

    pub fn external(label: impl Into<String>, href: impl Into<String>, section: Section) -> Self {
        Self {
            href: href.into(),
            label: label.into(),
            section,
            class: None,
            hx_boost: false,
            badge_image: None,
            badge_alt: None,
        }
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn with_badge_image(mut self, src: impl Into<String>, alt: impl Into<String>) -> Self {
        self.badge_image = Some(src.into());
        self.badge_alt = Some(alt.into());
        self
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NavigationMenu {
    pub label: String,
    pub links: Vec<NavigationLink>,
}

impl NavigationMenu {
    pub fn new(label: impl Into<String>, links: Vec<NavigationLink>) -> Self {
        Self {
            label: label.into(),
            links,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NavigationEntry {
    Link(NavigationLink),
    Menu(NavigationMenu),
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NavigationModel {
    pub home: String,
    pub desktop_left: Vec<NavigationEntry>,
    pub desktop_right: Vec<NavigationLink>,
    pub mobile: Vec<NavigationLink>,
}

#[component]
pub fn NavItem(link: NavigationLink, current_section: Section) -> Element {
    let mut added_class = "";
    if link.section == current_section {
        added_class = "underline";
    }

    let mut class = link.class.unwrap_or_default();
    if !added_class.is_empty() {
        if !class.is_empty() {
            class.push(' ');
        }
        class.push_str(added_class);
    }

    rsx! {
        li {
            a {
                class: class,
                "hx-boost": if link.hx_boost { "true" } else { "false" },
                href: link.href,
                if let Some(src) = link.badge_image {
                    img {
                        src: src,
                        alt: link.badge_alt.unwrap_or_default()
                    }
                } else {
                    "{link.label}"
                }
            }
        }
    }
}

#[component]
fn DesktopEntry(entry: NavigationEntry, current_section: Section) -> Element {
    match entry {
        NavigationEntry::Link(link) => rsx!(
            NavItem {
                link,
                current_section,
            }
        ),
        NavigationEntry::Menu(menu) => rsx!(
            li {
                details {
                    summary {
                        "{menu.label}"
                    }
                    ul {
                        class: "p-2",
                        for link in menu.links {
                            NavItem {
                                link,
                                current_section: current_section.clone(),
                            }
                        }
                    }
                }
            }
        ),
    }
}

#[component]
pub fn Navigation(
    mobile_menu: Option<Element>,
    section: Section,
    model: NavigationModel,
    brand: Option<String>,
    site_header: Option<SiteHeader>,
) -> Element {
    let brand = brand.unwrap_or_else(|| "Bionic".to_string());

    rsx! {
        header {
            class: "sticky top-0 z-50",
            if let Some(site_header) = site_header {
                {site_header}
            }
            div {
                class: "backdrop-filter backdrop-blur-lg bg-base-100/80 border-b border-base-300",
                div {
                    class: "navbar justify-between",

                    div {
                        class: "flex items-center gap-4",
                        a {
                            href: model.home.clone(),
                            span {
                                class: "pl-3 flex flex-row gap-2",
                                strong { "{brand}" }
                            }
                        }

                        div { class: "hidden lg:flex",
                            ul { class: "menu menu-horizontal px-1 dropdown-content",
                                for entry in model.desktop_left {
                                    DesktopEntry {
                                        entry,
                                        current_section: section.clone(),
                                    }
                                }
                            }
                        }
                    }

                    div { class: "hidden lg:flex items-center",
                        ul { class: "menu menu-horizontal px-3",
                            for link in model.desktop_right {
                                NavItem {
                                    link,
                                    current_section: section.clone(),
                                }
                            }
                        }
                    }

                    div { class: "dropdown lg:hidden dropdown-end",
                        div {
                            tabindex: "0",
                            role: "button",
                            class: "btn btn-ghost",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                fill: "none",
                                class: "h-5 w-5",
                                path {
                                    d: "M4 6h16M4 12h8m-8 6h16",
                                    stroke_linejoin: "round",
                                    stroke_linecap: "round",
                                    stroke_width: "2"
                                }
                            }
                        }
                        ul {
                            class: "menu menu-sm dropdown-content mt-3 z-1 p-2 shadow-sm bg-base-100 rounded-box w-52",
                            for link in model.mobile {
                                NavItem {
                                    link,
                                    current_section: section.clone(),
                                }
                            }
                            {mobile_menu}
                        }
                    }
                }
            }
        }
    }
}
