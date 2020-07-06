use anyhow::Result;
use log::{debug, error, info};
use std::{
    env,
    fs::write,
    path::Path,
    process::{Command, Stdio},
};

mod models;
mod parsing;
use parsing::watch_command;

const OUTPUT_FILE_NAME: &str = "output.json";

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "wiki_grapher=debug");
    }
    pretty_env_logger::init();

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        error!("Must run with path to compressed Wikipedia database download");
        return Ok(());
    }

    info!("Starting");
    let mut cmd = Command::new("bzip2")
        .args(&["-d", "-c", &args[0]])
        .stdout(Stdio::piped())
        .spawn()?;

    let pages = watch_command(&mut cmd)?;
    let status = cmd.wait()?;
    debug!("Command exit status is {}", status.code().unwrap_or(0));

    debug!("Writing output file");
    write(Path::new(OUTPUT_FILE_NAME), serde_json::to_string(&pages)?)?;
    info!("Wrote data to '{}'", OUTPUT_FILE_NAME);

    Ok(())
}
