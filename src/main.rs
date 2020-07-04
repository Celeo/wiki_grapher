use anyhow::{anyhow, Result};
use log::{debug, info};
use roxmltree::Document;
use std::{
    env,
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

const PATH_TO: &str = "/media/sf_VirtualShareed/enwiki-20200401-pages-articles-multistream.xml.bz2";

#[derive(Debug)]
struct PageParseResult {
    page: Option<String>,
    remainder: String,
}

impl PageParseResult {
    fn new(page: Option<String>, remainder: &str) -> Self {
        Self {
            page,
            remainder: remainder.to_owned(),
        }
    }
}

fn try_get_page(content: &str) -> Result<PageParseResult> {
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
        .take(index_end - index_start + 7)
        .collect();
    let remainder: String = content.chars().skip(index_end + 7).collect();
    Ok(PageParseResult::new(Some(page_portion), &remainder))
}

fn parse_page(page: &str) -> Result<String> {
    let doc = Document::parse(page)?;
    let title_node = doc
        .descendants()
        .find(|n| n.has_tag_name("title"))
        .ok_or_else(|| anyhow!("Could not find <title> tag"))?;
    let title = title_node
        .text()
        .ok_or_else(|| anyhow!("Could not get text from node"))?;
    debug!("Got title from page: {}", title);
    Ok(title.to_owned())
}

fn monitor_command(cmd: &mut Child) -> Result<Vec<String>> {
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

        let result = try_get_page(&buffer)?;
        buffer = result.remainder;

        match result.page {
            Some(p) => {
                pages.push(parse_page(&p)?);
                if pages.len() % 100 == 0 {
                    info!("Parsed {} pages", pages.len());
                }
            }
            None => (),
        }
    }
    Ok(pages)
}

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "wiki_grapher=debug");
    }
    pretty_env_logger::init();
    info!("Starting");

    let mut cmd = Command::new("bzip2")
        .args(&["-d", "-c", PATH_TO])
        .stdout(Stdio::piped())
        .spawn()?;

    let titles = monitor_command(&mut cmd)?;
    let status = cmd.wait()?;
    println!("Exit status is {}", status.code().unwrap_or(0));
    println!("{}", titles.join(", "));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::try_get_page;

    #[test]
    fn test_try_get_pages() {
        let s = "<page>Foobar</page>b";
        let result = try_get_page(s).unwrap();

        assert_eq!("<page>Foobar</page>", result.page.unwrap());
        assert_eq!("b", result.remainder);
    }
}
