use crate::models::PageInfo;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use log::{debug, info};
use regex::Regex;
use roxmltree::Document;
use std::{
    io::{BufRead, BufReader},
    process::Child,
};

lazy_static! {
    /// Regex for capturing in-wiki links with optional disregarded rename.
    static ref LINK_REGEX: Regex =
        Regex::new(r#"\[\[([a-zA-Z0-9- ]+)[|]?[a-zA-Z0-9- ]*\]\]"#).unwrap();
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
    let content = doc
        .descendants()
        .find(|n| n.has_tag_name("text"))
        .ok_or_else(|| anyhow!("Could not find <text> node"))?
        .text()
        .ok_or_else(|| anyhow!("Could not get text from text node"))?;
    Ok((title, content))
}

pub(crate) fn watch_command(cmd: &mut Child) -> Result<Vec<PageInfo>> {
    let stdout = cmd
        .stdout
        .as_mut()
        .ok_or_else(|| anyhow!("Couldn't get stdout ref"))?;
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    let mut pages = vec![];
    let mut buffer = String::new();
    let mut skipped_header = false;

    for line in stdout_lines {
        let line = line?;
        buffer += &line.trim();
        if !skipped_header {
            if line.ends_with("</siteinfo>") {
                debug!("Gotten past the header");
                skipped_header = true;
            }
            continue;
        }
        if line.ends_with("</page>") {
            let doc = Document::parse(&buffer)?;
            let (title, content) = parse_page(&doc)?;
            let links = extract_links(content);
            pages.push(PageInfo::new(title, links));
            debug!("Parsed {}", title);
            if pages.len() % 1000 == 0 {
                info!("Parsed {} pages", pages.len());
            }
            buffer.clear();
        }
    }

    Ok(pages)
}

#[cfg(test)]
mod tests {
    use super::{extract_links, parse_page};
    use roxmltree::Document;

    #[test]
    fn test_extract_links() {
        let s = "aaa [a] a| [ [[ [[aa]] a aa] [[aaa|aaaa]]";
        let links = extract_links(s);

        assert_eq!(links, vec!["aa", "aaa"]);
    }

    #[test]
    fn test_parse_page() {
        let doc = Document::parse(
            r#"<page>
            <title>AccessibleComputing</title>
            <ns>0</ns>
            <id>10</id>
            <redirect title="Computer accessibility" />
            <revision>
            <id>854851586</id>
            <parentid>834079434</parentid>
            <timestamp>2018-08-14T06:47:24Z</timestamp>
            <contributor>
                <username>Godsy</username>
                <id>23257138</id>
            </contributor>
            <comment>remove from category for seeking instructions on rcats</comment>
            <model>wikitext</model>
            <format>text/x-wiki</format>
            <text bytes="94" xml:space="preserve">#REDIRECT [[Computer accessibility]]

        {{R from move}}
        {{R from CamelCase}}
        {{R unprintworthy}}</text>
            <sha1>42l0cvblwtb4nnupxm6wo000d27t6kf</sha1>
            </revision>
        </page>"#,
        )
        .unwrap();
        let (title, text) = parse_page(&doc).unwrap();

        assert_eq!(title, "AccessibleComputing");
        assert_eq!(text.len(), 118);
    }
}
