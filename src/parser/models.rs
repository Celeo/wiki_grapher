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

    // pub(crate) fn to_pairs(&self) -> Vec<(&str, &str)> {
    //     let mut res: Vec<(&str, &str)> = vec![];
    //     for link in &self.links {
    //         res.push((&self.title, link));
    //     }
    //     res
    // }

    pub(crate) fn to_csv_line(&self) -> String {
        let rest = self
            .links
            .iter()
            .map(|title| {
                if title.contains(',') {
                    format!("\"{}\"", title)
                } else {
                    title.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("{},{}", self.title, rest)
    }
}
