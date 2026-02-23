use daisy_rsx::marketing::footer::FooterLinks;
use daisy_rsx::marketing::navigation::{
    NavigationEntry, NavigationLink, NavigationModel, Section,
};
use ssg_whiz::SiteMeta;

pub fn navigation_links() -> NavigationModel {
    NavigationModel {
        home: "/".to_string(),
        logo_src: None,
        logo_alt: None,
        desktop_left: vec![
            NavigationEntry::Link(
                NavigationLink::external("Platform", "/#hero", Section::Home)
                    .with_class("nav-link"),
            ),
            NavigationEntry::Link(
                NavigationLink::external("Security", "/#manifesto", Section::Home)
                    .with_class("nav-link"),
            ),
            NavigationEntry::Link(
                NavigationLink::external("Integration", "/#artifacts", Section::Home)
                    .with_class("nav-link"),
            ),
            NavigationEntry::Link(
                NavigationLink::external("Contact", "/blog", Section::Blog)
                    .with_class("nav-link"),
            ),
        ],
        desktop_right: vec![
            NavigationLink::external("Schedule a Demo", "/blog", Section::Blog)
                .with_class("btn nav-cta nav-link"),
        ],
        mobile: vec![
            NavigationLink::external("Platform", "/#hero", Section::Home),
            NavigationLink::external("Security", "/#manifesto", Section::Home),
            NavigationLink::external("Integration", "/#artifacts", Section::Home),
            NavigationLink::external("Contact", "/blog", Section::Blog),
            NavigationLink::external("Schedule a Demo", "/blog", Section::Blog),
        ],
    }
}

pub fn footer_links() -> FooterLinks {
    FooterLinks {
        blog: "/blog".to_string(),
        pricing: "/pricing".to_string(),
        contact: "/contact".to_string(),
        terms: "/terms".to_string(),
        privacy: "/privacy".to_string(),
        about: None,
    }
}

pub fn site_meta() -> SiteMeta {
    SiteMeta {
        base_url: "https://decision.example.com".to_string(),
        site_name: "Decision Advantage".to_string(),
        brand_name: "Decision".to_string(),
        goatcounter: "".to_string(),
    }
}
