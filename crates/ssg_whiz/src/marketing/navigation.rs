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

    rsx! {
        header {
            class: "sticky top-0 z-50 backdrop-filter backdrop-blur-lg bg-opacity-30",
            if let Some(site_header) = site_header {
                {site_header}
            }
            div { class: "navbar justify-between pl-4 pr-4",
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
                                span { class: "flex flex-row gap-2 items-center",
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
                        for entry in model.desktop_left {
                            match entry {
                                NavigationEntry::Link(link) => rsx! {
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
                                },
                                NavigationEntry::Menu(menu) => rsx! {
                                    li { class: "dropdown dropdown-hover",
                                        div {
                                            tabindex: "0",
                                            role: "button",
                                            "aria-haspopup": "true",
                                            class: "flex cursor-pointer items-center gap-1",
                                            "{menu.label}"
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 20 20",
                                                fill: "currentColor",
                                                class: "h-3 w-3",
                                                path { d: "M5.23 7.21a.75.75 0 011.06.02L10 11.06l3.71-3.83a.75.75 0 111.08 1.04l-4.25 4.39a.75.75 0 01-1.08 0L5.21 8.27a.75.75 0 01.02-1.06z" }
                                            }
                                        }
                                        ul { class: "menu dropdown-content z-10 w-52 rounded-box bg-base-100 p-2 shadow",
                                            for link in menu.links {
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
                }
                div { class: "hidden lg:flex items-center gap-3",
                    for link in model.desktop_right {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_desktop_menu_without_flattening_links() {
        let model = NavigationModel {
            home: "/".to_string(),
            logo_src: None,
            logo_alt: None,
            desktop_left: vec![
                NavigationEntry::Link(
                    NavigationLink::new("Top", "/top", Section::Home).with_class("custom-link"),
                ),
                NavigationEntry::Menu(NavigationMenu::new(
                    "Product",
                    vec![
                        NavigationLink::new("Pricing", "/pricing", Section::Pricing),
                        NavigationLink::external("Docs", "/docs", Section::Docs),
                        NavigationLink::new("Badge", "/badge", Section::Blog)
                            .with_badge_image("/badge.svg", "Badge"),
                    ],
                )),
            ],
            desktop_right: vec![],
            mobile: vec![NavigationLink::new("Mobile Only", "/mobile", Section::Home)],
        };

        let html = dioxus_ssr::render_element(rsx! {
            Navigation {
                mobile_menu: None,
                section: Section::Pricing,
                model,
                brand: Some("Test".to_string()),
                site_header: None
            }
        });

        assert!(html.contains(r#"<li class="dropdown dropdown-hover">"#));
        assert!(html.contains(r#"role="button" aria-haspopup="true" class="flex cursor-pointer items-center gap-1">Product"#));
        assert!(html.contains(r#"class="h-3 w-3""#));
        assert!(!html.contains(r#"role="button" class="btn btn-ghost">Product"#));
        assert!(html.contains(
            r#"<ul class="menu dropdown-content z-10 w-52 rounded-box bg-base-100 p-2 shadow">"#
        ));
        assert!(html.contains(r#"class="custom-link" hx-boost="true" href="/top">Top"#));
        assert!(html.contains(r#"class="active" hx-boost="true" href="/pricing">Pricing"#));
        assert!(html.contains(r#"hx-boost="false" href="/docs">Docs"#));
        assert!(html.contains(r#"src="/badge.svg" alt="Badge""#));
    }
}
