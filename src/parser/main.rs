use anyhow::Result;
use log::{debug, error, info};
use rusqlite::Connection;
use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

mod models;
mod parsing;
use parsing::watch_command;

const DB_FILE_NAME: &str = "data.db";
const DB_BAK_FILE_NAME: &str = "data.db.bak";

fn setup_db() -> Result<Connection> {
    debug!("Setting up the DB");
    let path = Path::new(DB_FILE_NAME);
    if path.exists() {
        debug!("DB file already exists");
        let backup_path = Path::new(DB_BAK_FILE_NAME);
        if backup_path.exists() {
            debug!("Removing previous backup file");
            fs::remove_file(backup_path)?;
        }
        debug!("Renaming last db file to the backup");
        fs::rename(path, backup_path)?;
    }
    debug!("Opening connection to create tables");
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "
        CREATE TABLE pages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        );

        CREATE TABLE links (
            page_from INTEGER NOT NULL,
            page_to TEXT NOT NULL,

            FOREIGN KEY(page_from) REFERENCES pages(id)
        );
    ",
    )?;
    debug!("DB created");
    Ok(conn)
}

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "parser=info");
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

    let conn = setup_db()?;
    watch_command(&mut cmd, conn)?;

    let status = cmd.wait()?;
    debug!("Command exit status is {}", status.code().unwrap_or(0));

    Ok(())
}
