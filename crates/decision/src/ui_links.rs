use daisy_rsx::marketing::footer::FooterLinks;
use daisy_rsx::marketing::navigation::NavigationLinks;
use ssg_whiz::SiteMeta;

pub fn navigation_links() -> NavigationLinks {
    NavigationLinks {
        home: "/".to_string(),
        pricing: "/pricing".to_string(),
        blog: "/blog".to_string(),
        docs: "/docs".to_string(),
        architect_course: "/architect-course".to_string(),
        partners: "/partners".to_string(),
        contact: "/contact".to_string(),
        product_chat: "/product/chat".to_string(),
        product_assistants: "/product/assistants".to_string(),
        product_integrations: "/product/integrations".to_string(),
        product_automations: "/product/automations".to_string(),
        product_developers: "/product/developers".to_string(),
        sign_in_up: "#".to_string(),
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
        site_name: "Decision".to_string(),
        brand_name: "Decision".to_string(),
        goatcounter: "".to_string(),
    }
}
