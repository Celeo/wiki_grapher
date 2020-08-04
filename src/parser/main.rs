use anyhow::Result;
use log::{debug, error, info};
use std::{
    env,
    fs::{self, File},
    path::Path,
    process::{Command, Stdio},
};

mod models;
mod parsing;
use parsing::watch_command;

const CSV_FILE_NAME: &str = "data.csv";
const CSV_BAK_FILE_NAME: &str = "data.csv.bak";

fn setup_csv() -> Result<File> {
    debug!("Setting up the CSV file");
    let path = Path::new(CSV_FILE_NAME);
    if path.exists() {
        debug!("CSV file already exists");
        let backup_path = Path::new(CSV_BAK_FILE_NAME);
        if backup_path.exists() {
            debug!("Removing previous backup file");
            fs::remove_file(backup_path)?;
        }
        debug!("Renaming last CSV file to the backup");
        fs::rename(path, backup_path)?;
    }
    debug!("DB created");
    let f = File::create(CSV_FILE_NAME)?;
    Ok(f)
}

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
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

    let file = setup_csv()?;
    watch_command(&mut cmd, file)?;

    let status = cmd.wait()?;
    debug!("Command exit status is {}", status.code().unwrap_or(0));

    Ok(())
}
