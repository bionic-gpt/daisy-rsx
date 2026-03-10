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

#[component]
pub fn NavItem(link: NavigationLink, current_section: Section) -> Element {
    let mut class = String::from("site-nav__link");
    if link.section == current_section {
        class.push_str(" site-nav__link--active");
    }
    if let Some(extra) = link.class {
        if !extra.is_empty() {
            class.push(' ');
            class.push_str(&extra);
        }
    }

    rsx! {
        li { class: "site-nav__item",
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
        NavigationEntry::Link(link) => rsx!(NavItem {
            link,
            current_section,
        }),
        NavigationEntry::Menu(menu) => rsx!(
            li { class: "site-nav__item site-nav__item--menu",
                details { class: "site-nav__menu",
                    summary { class: "site-nav__menu-summary",
                        "{menu.label}"
                    }
                    ul { class: "site-nav__submenu",
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
            class: "site-nav",
            if let Some(site_header) = site_header {
                {site_header}
            }
            div { class: "site-nav__backdrop",
                div { class: "site-nav__inner",
                    div { class: "site-nav__brand-group",
                        a {
                            class: "site-nav__brand-link",
                            href: model.home.clone(),
                            span { class: "site-nav__brand",
                                if let Some(logo_src) = model.logo_src {
                                    img {
                                        class: "site-nav__logo",
                                        src: logo_src,
                                        alt: model.logo_alt.unwrap_or_else(|| format!("{brand} logo"))
                                    }
                                }
                                strong { class: "site-nav__brand-text",
                                    "{brand}"
                                }
                            }
                        }

                        nav { class: "site-nav__desktop site-nav__desktop--left",
                            ul { class: "site-nav__list",
                                for entry in model.desktop_left {
                                    DesktopEntry {
                                        entry,
                                        current_section: section.clone(),
                                    }
                                }
                            }
                        }
                    }

                    nav { class: "site-nav__desktop site-nav__desktop--right",
                        ul { class: "site-nav__list site-nav__list--right",
                            for link in model.desktop_right {
                                NavItem {
                                    link,
                                    current_section: section.clone(),
                                }
                            }
                        }
                    }

                    details { class: "site-nav__mobile",
                        summary {
                            class: "site-nav__mobile-toggle",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                fill: "none",
                                class: "site-nav__mobile-icon",
                                path {
                                    d: "M4 6h16M4 12h8m-8 6h16",
                                    stroke_linejoin: "round",
                                    stroke_linecap: "round",
                                    stroke_width: "2"
                                }
                            }
                        }
                        ul { class: "site-nav__mobile-menu",
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
