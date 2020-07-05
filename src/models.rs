use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct PageInfo {
    pub(crate) title: String,
    pub(crate) links: Vec<String>,
}
