use anyhow::Result;
// use crossbeam::channel::unbounded;
use log::{debug, info};
use std::{
    env,
    fs,
    path::Path,
    process::{Command, Stdio},
    // thread,
};

mod models;
mod parsing;
use parsing::monitor_command;

const PATH_TO: &str = "/media/sf_VirtualShareed/enwiki-20200401-pages-articles-multistream.xml.bz2";

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

    let pages = monitor_command(&mut cmd)?;
    let status = cmd.wait()?;
    info!("Exit status is {}", status.code().unwrap_or(0));
    debug!("Writing output file");
    fs::write(Path::new("output.json"), serde_json::to_string(&pages)?)?;
    info!("Wrote data to 'output.json'");

    Ok(())
}
