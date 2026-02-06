use daisy_rsx::marketing::{footer::FooterLinks, navigation::NavigationLinks};

pub fn navigation_links() -> NavigationLinks {
    NavigationLinks {
        home: crate::routes::marketing::Index {}.to_string(),
        pricing: crate::routes::marketing::Pricing {}.to_string(),
        blog: crate::routes::blog::Index {}.to_string(),
        docs: crate::routes::docs::Index {}.to_string(),
        architect_course: crate::routes::architect_course::Index {}.to_string(),
        partners: crate::routes::marketing::PartnersPage {}.to_string(),
        contact: crate::routes::marketing::Contact {}.to_string(),
        product_chat: crate::routes::product::Chat {}.to_string(),
        product_assistants: crate::routes::product::Assistants {}.to_string(),
        product_integrations: crate::routes::product::Integrations {}.to_string(),
        product_automations: crate::routes::product::Automations {}.to_string(),
        product_developers: crate::routes::product::Developers {}.to_string(),
        sign_in_up: crate::routes::SIGN_IN_UP.to_string(),
    }
}

pub fn footer_links() -> FooterLinks {
    FooterLinks {
        blog: crate::routes::blog::Index {}.to_string(),
        pricing: crate::routes::marketing::Pricing {}.to_string(),
        contact: crate::routes::marketing::Contact {}.to_string(),
        terms: crate::routes::marketing::Terms {}.to_string(),
        privacy: crate::routes::marketing::Privacy {}.to_string(),
        about: None,
    }
}
