use crate::models::{PageInfo, PageParseResult};
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use log::{debug, info};
use regex::Regex;
use roxmltree::Document;
use std::{
    io::{BufRead, BufReader},
    process::Child,
};

const TAG_SIZE: usize = 7;

lazy_static! {
    /// Regex for capturing in-wiki links with optional disregarded rename.
    static ref LINK_REGEX: Regex =
        Regex::new(r#"\[\[([a-zA-Z0-9- ]+)[|]?[a-zA-Z0-9- ]*\]\]"#).unwrap();
}

fn try_get_page(content: String) -> Result<PageParseResult> {
    let index_start = match content.find("<page>") {
        Some(i) => i,
        None => return Ok(PageParseResult::new(None, content)),
    };
    let index_end = match content.find("</page>") {
        Some(i) => i,
        None => return Ok(PageParseResult::new(None, content)),
    };
    debug!("Page section index: {} -> {}", index_start, index_end);
    let page_portion: String = content
        .chars()
        .skip(index_start)
        .take(index_end - index_start + TAG_SIZE)
        .collect();
    let remainder: String = content.chars().skip(index_end + TAG_SIZE).collect();
    Ok(PageParseResult::new(Some(page_portion), remainder))
}

fn extract_links(content: &str) -> Vec<String> {
    let mut links: Vec<String> = vec![];
    for cap in LINK_REGEX.captures_iter(content) {
        links.push((&cap[1]).to_owned());
    }
    links
}

fn parse_page<'a>(doc: &'a Document) -> Result<(&'a str, &'a str)> {
    let title = doc
        .descendants()
        .find(|n| n.has_tag_name("title"))
        .ok_or_else(|| anyhow!("Could not find <title> tag"))?
        .text()
        .ok_or_else(|| anyhow!("Could not get text from title node"))?;
    debug!("Got title from page: {}", title);
    let content = doc
        .descendants()
        .find(|n| n.has_tag_name("text"))
        .ok_or_else(|| anyhow!("Could not find <text> node"))?
        .text()
        .ok_or_else(|| anyhow!("Could not get text from text node"))?;
    Ok((title, content))
}

pub(crate) fn monitor_command(cmd: &mut Child) -> Result<Vec<PageInfo>> {
    let mut pages = vec![];
    let mut buffer = String::new();

    let stdout = cmd
        .stdout
        .as_mut()
        .ok_or_else(|| anyhow!("Couldn't get stdout ref"))?;
    let sdtout_reader = BufReader::new(stdout);
    let stdout_lines = sdtout_reader.lines();

    for line in stdout_lines {
        let line = line?;
        buffer += &line;

        let result = try_get_page(buffer)?;
        buffer = result.remainder;

        match result.page {
            Some(p) => {
                let doc = Document::parse(&p)?;
                let (title, content) = parse_page(&doc)?;
                let links = extract_links(&content);
                pages.push(PageInfo::new(title, links));
                if pages.len() % 1000 == 0 {
                    info!("Parsed {} pages", pages.len());
                }
            }
            None => (),
        }
    }
    Ok(pages)
}

#[cfg(test)]
mod tests {
    use super::{extract_links, try_get_page};

    #[test]
    fn test_try_get_pages() {
        let s = "<page>Foobar</page>b".to_owned();
        let result = try_get_page(s).unwrap();

        assert_eq!("<page>Foobar</page>", result.page.unwrap());
        assert_eq!("b", result.remainder);
    }

    #[test]
    fn test_extract_links() {
        let s = "aaa [a] a| [ [[ [[aa]] a aa] [[aaa|aaaa]]";
        let links = extract_links(s);

        assert_eq!(links, vec!["aa", "aaa"]);
    }
}
