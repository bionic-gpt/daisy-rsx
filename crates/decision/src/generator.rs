use ssg_whiz::SitePage;

use crate::pages;

fn output_page(path: &str, html: String) -> SitePage {
    SitePage {
        path: path.to_string(),
        html,
    }
}

pub async fn generate_static_pages() -> Vec<SitePage> {
    vec![
        output_page("", pages::index::page()),
        output_page("contact", pages::contact::page()),
    ]
}
