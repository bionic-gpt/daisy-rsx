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
    pub logo_src: Option<String>,
    pub logo_alt: Option<String>,
    pub desktop_left: Vec<NavigationEntry>,
    pub desktop_right: Vec<NavigationLink>,
    pub mobile: Vec<NavigationLink>,
}

fn nav_entry_links(entries: Vec<NavigationEntry>) -> Vec<NavigationLink> {
    let mut links = Vec::new();
    for entry in entries {
        match entry {
            NavigationEntry::Link(link) => links.push(link),
            NavigationEntry::Menu(menu) => links.extend(menu.links),
        }
    }
    links
}

fn nav_link_class(link: &NavigationLink, current_section: &Section) -> Option<String> {
    let mut classes = Vec::new();

    if link.section == *current_section {
        classes.push("active".to_string());
    }

    if let Some(extra) = &link.class {
        if !extra.is_empty() {
            classes.push(extra.clone());
        }
    }

    if classes.is_empty() {
        None
    } else {
        Some(classes.join(" "))
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
    let desktop_left = nav_entry_links(model.desktop_left);

    rsx! {
        header {
            class: "sticky top-0 z-50 backdrop-filter backdrop-blur-lg bg-opacity-30",
            if let Some(site_header) = site_header {
                {site_header}
            }
            div { class: "navbar justify-between",
                div {
                    class: "flex items-center gap-4",
                    div { class: "dropdown lg:hidden",
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
                        ul { class: "menu menu-sm dropdown-content mt-3 z-10 w-52 rounded-box bg-base-100 p-2 shadow",
                            for link in &model.mobile {
                                li {
                                    a {
                                        class: nav_link_class(link, &section),
                                        "hx-boost": if link.hx_boost { "true" } else { "false" },
                                        href: link.href.clone(),
                                        if let Some(src) = &link.badge_image {
                                            img {
                                                src: src.clone(),
                                                alt: link.badge_alt.clone().unwrap_or_default()
                                            }
                                        } else {
                                            "{link.label}"
                                        }
                                    }
                                }
                            }
                            {mobile_menu}
                        }
                    }
                    ul { class: "flex flex-row items-center gap-4",
                        li {
                            a {
                                href: model.home.clone(),
                                span { class: "pl-3 flex flex-row gap-2 items-center",
                                    if let Some(logo_src) = model.logo_src {
                                        img {
                                            class: "h-8 w-auto",
                                            src: logo_src,
                                            alt: model.logo_alt.unwrap_or_else(|| format!("{brand} logo"))
                                        }
                                    }
                                    strong { "{brand}" }
                                }
                            }
                        }
                        for link in desktop_left {
                            li {
                                a {
                                    class: nav_link_class(&link, &section),
                                    "hx-boost": if link.hx_boost { "true" } else { "false" },
                                    href: link.href,
                                    if let Some(src) = &link.badge_image {
                                        img {
                                            src: src.clone(),
                                            alt: link.badge_alt.clone().unwrap_or_default()
                                        }
                                    } else {
                                        "{link.label}"
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "hidden lg:flex",
                    ul { class: "menu menu-horizontal px-1",
                        for link in model.desktop_right {
                            li {
                                a {
                                    class: nav_link_class(&link, &section),
                                    "hx-boost": if link.hx_boost { "true" } else { "false" },
                                    href: link.href,
                                    if let Some(src) = &link.badge_image {
                                        img {
                                            src: src.clone(),
                                            alt: link.badge_alt.clone().unwrap_or_default()
                                        }
                                    } else {
                                        "{link.label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
