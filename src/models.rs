use serde::Serialize;

#[derive(Debug)]
pub(crate) struct PageParseResult {
    pub(crate) page: Option<String>,
    pub(crate) remainder: String,
}

impl PageParseResult {
    pub(crate) fn new(page: Option<String>, remainder: String) -> Self {
        Self { page, remainder }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct PageInfo {
    pub(crate) title: String,
    pub(crate) links: Vec<String>,
}

impl PageInfo {
    pub(crate) fn new(title: &str, links: Vec<String>) -> Self {
        Self {
            title: title.to_owned(),
            links,
        }
    }
}
