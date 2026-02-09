use daisy_rsx::marketing::footer::FooterLinks;
use daisy_rsx::marketing::navigation::{NavigationLink, NavigationModel, Section};
use ssg_whiz::SiteMeta;

pub fn navigation_links() -> NavigationModel {
    NavigationModel {
        home: "/".to_string(),
        logo_src: Some("/logo.svg".to_string()),
        logo_alt: Some("Airbus Decision logo".to_string()),
        desktop_left: vec![],
        desktop_right: vec![NavigationLink::new("Blog", "/blog", Section::Blog)],
        mobile: vec![
            NavigationLink::new("Home", "/", Section::Home),
            NavigationLink::new("Blog", "/blog", Section::Blog),
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
        brand_name: "Decision Advantage".to_string(),
        goatcounter: "".to_string(),
    }
}
