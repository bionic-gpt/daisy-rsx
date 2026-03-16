use ssg_whiz::{FooterLinks, NavigationEntry, NavigationLink, NavigationModel, Section, SiteMeta};

pub fn navigation_links() -> NavigationModel {
    NavigationModel {
        home: "/".to_string(),
        logo_src: Some("/logo.svg".to_string()),
        logo_alt: Some("Example Site logo".to_string()),
        desktop_left: vec![
            NavigationEntry::Link(NavigationLink::external("Home", "/", Section::Home)),
            NavigationEntry::Link(NavigationLink::external(
                "Docs",
                "/getting-started",
                Section::Docs,
            )),
            NavigationEntry::Link(NavigationLink::external("Blog", "/blog", Section::Blog)),
        ],
        desktop_right: vec![NavigationLink::new("Get Started", "/#hero", Section::Home)
            .with_class("btn btn-primary")],
        mobile: vec![
            NavigationLink::external("Home", "/", Section::Home),
            NavigationLink::external("Docs", "/getting-started", Section::Docs),
            NavigationLink::external("Blog", "/blog", Section::Blog),
            NavigationLink::new("Get Started", "/#hero", Section::Home),
        ],
    }
}

pub fn footer_links() -> FooterLinks {
    FooterLinks {
        blog: "/blog".to_string(),
        pricing: "/".to_string(),
        contact: "/".to_string(),
        terms: "/".to_string(),
        privacy: "/".to_string(),
        about: None,
        variant: None,
    }
}

pub fn site_meta() -> SiteMeta {
    SiteMeta {
        base_url: "https://example.com".to_string(),
        site_name: "Example Site".to_string(),
        brand_name: "Example Site".to_string(),
        goatcounter: "".to_string(),
    }
}
