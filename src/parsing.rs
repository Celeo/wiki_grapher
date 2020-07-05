use crate::models::PageInfo;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use log::{debug, error, info};
use regex::Regex;
use std::{
    io::{BufRead, BufReader},
    process::Child,
};

lazy_static! {
    /// Regex for capturing in-wiki links with optional disregarded rename.
    static ref TITLE_REGEX: Regex = Regex::new(r#"<title>([a-zA-Z0-9-/>> ]+)</title>"#).unwrap();
    static ref CONTENT_REGEX: Regex = Regex::new(r#""#).unwrap();
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

fn parse_page(content: &str) -> Result<(String, String)> {
    let title = &TITLE_REGEX
        .captures(content)
        .ok_or_else(|| anyhow!("Could not find <title> tag"))?[1];

    let text_start = content
        .find("<text")
        .ok_or_else(|| anyhow!("Could not find opening text tag"))?;
    let text_end = content
        .find("</text>")
        .ok_or_else(|| anyhow!("Could not find closing text tag"))?;
    let text: String = content
        .chars()
        .skip(text_start)
        .skip_while(|&c| c != '>')
        .skip(2)
        .take(text_end - text_start)
        .collect();

    Ok((title.to_owned(), text))
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
            let (title, content) = match parse_page(&buffer) {
                Ok((t, c)) => (t, c),
                Err(e) => {
                    error!("Could not parse page: {}", e);
                    buffer.clear();
                    continue;
                }
            };
            let links = extract_links(&content);
            debug!("Parsed {}", title);
            pages.push(PageInfo { title, links });
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

    #[test]
    fn test_extract_links() {
        let s = "aaa [a] a| [ [[ [[aa]] a aa] [[aaa|aaaa]]";
        let links = extract_links(s);

        assert_eq!(links, vec!["aa", "aaa"]);
    }

    #[test]
    fn test_parse_page2() {
        let s = r#"
        <page>
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
        </page>
        "#;
        let (title, text) = parse_page(s).unwrap();

        assert_eq!(title, "AccessibleComputing");
        assert_eq!(text.len(), 156);
    }
}
