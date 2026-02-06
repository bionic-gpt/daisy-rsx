use daisy_rsx::marketing::navigation::Section;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Summary {
    pub source_folder: &'static str,
    pub categories: Vec<Category>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Category {
    pub name: String,
    pub pages: Vec<PageSummary>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PageSummary {
    pub date: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub folder: &'static str,
    pub markdown: &'static str,
    pub image: Option<&'static str>,
    pub author: Option<&'static str>,
    pub author_image: Option<&'static str>,
}

impl PageSummary {
    pub fn permalink(&self) -> String {
        format!("https://bionic-gpt.com/{}", self.folder)
    }
}

pub type BlogSummary = Summary;
pub type PagesSummary = Summary;
pub type DocumentSummary = Summary;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DocumentSite {
    pub summary: DocumentSummary,
    pub section: Section,
}
