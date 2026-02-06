use std::fs;
use std::path::Path;

use dioxus::prelude::*;

use daisy_rsx::marketing::navigation::Section;
use ssg_whiz::Page as OutputPage;
use crate::layouts::blog::{BlogList, BlogPost};
use crate::layouts::docs::Document;
use crate::layouts::pages::MarkdownPage;
use crate::pages;

#[derive(PartialEq, Eq, Clone)]
pub struct Summary {
    pub source_folder: &'static str,
    pub categories: Vec<Category>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Category {
    pub name: String,
    pub pages: Vec<Page>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Page {
    pub date: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub folder: &'static str,
    pub markdown: &'static str,
    pub image: Option<&'static str>,
    pub author: Option<&'static str>,
    pub author_image: Option<&'static str>,
}

impl Page {
    pub fn permalink(&self) -> String {
        format!("https://bionic-gpt.com/{}", self.folder)
    }
}

fn output_page(path: &str, html: String) -> OutputPage {
    OutputPage {
        path: path.to_string(),
        html,
    }
}

pub async fn generate_product() -> Vec<OutputPage> {
    vec![
        output_page("product/assistants", pages::product::assistants::page()),
        output_page("product/automations", pages::product::automations::page()),
        output_page("product/chat", pages::product::chat::page()),
        output_page("product/developers", pages::product::developers::page()),
        output_page("product/integrations", pages::product::integrations::page()),
    ]
}

pub async fn generate_solutions() -> Vec<OutputPage> {
    vec![
        output_page("solutions/education", pages::solutions::education::page()),
        output_page("solutions/support", pages::solutions::support::page()),
    ]
}

pub async fn generate_marketing() -> Vec<OutputPage> {
    vec![
        output_page("pricing", pages::pricing::pricing()),
        output_page("partners", pages::partners::partners_page()),
        output_page("contact", pages::contact::contact_page()),
        output_page("", pages::home::home_page()),
    ]
}

pub fn generate(summary: Summary) -> Vec<OutputPage> {
    let mut pages_out = Vec::new();
    for category in summary.categories {
        for page in category.pages {
            let page_ele = rsx! {
                BlogPost {
                    post: page
                }
            };

            let html = crate::render(page_ele);
            pages_out.push(output_page(page.folder, html));
        }
    }

    pages_out
}

pub fn generate_docs(summary: Summary, section: Section) -> Vec<OutputPage> {
    let mut pages_out = Vec::new();
    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                Document {
                    summary: summary.clone(),
                    category: category.clone(),
                    doc: *page,
                    current_section: section.clone(),
                }
            };

            let html = crate::render(page_ele);
            pages_out.push(output_page(page.folder, html));
        }
    }

    pages_out
}

pub async fn generate_pages(summary: Summary) -> Vec<OutputPage> {
    let mut pages_out = Vec::new();
    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                MarkdownPage {
                    post: *page
                }
            };
            let html = crate::render(page_ele);
            pages_out.push(output_page(page.folder, html));
        }
    }

    pages_out
}

pub async fn generate_blog_list(summary: Summary) -> Vec<OutputPage> {
    let page_ele = rsx! {
        BlogList {
            summary
        }
    };
    let html = crate::render(page_ele);
    vec![output_page("blog", html)]
}

pub fn copy_folder(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_folder(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
