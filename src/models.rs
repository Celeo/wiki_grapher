#[derive(Debug)]
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

    pub(crate) fn to_pairs(&self) -> Vec<(&str, &str)> {
        let mut res: Vec<(&str, &str)> = vec![];
        for link in &self.links {
            res.push((&self.title, link));
        }
        res
    }
}
