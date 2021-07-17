mod attestation;
mod commands;
mod datastore;
use log::debug;
use pretty_env_logger;

use crate::commands::{Command, UtuOptions, WatchOpts};
use anyhow::Result;
use gumdrop::Options;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize the logger
    pretty_env_logger::init();
    // Parse options from the environment.
    // If there's an error or the user requests help,
    // the process will exit after giving the appropriate response.
    let opts = UtuOptions::parse_args_default_or_exit();

    match opts.command {
        Some(c) => match c {
            Command::Watch(options) => {
                let c = datastore::new().await.unwrap();
                let dbs = c.list_database_names(None, None).await?;
                println!("{:#?}", dbs);
                println!("WatchOpts {:?}", options);
                debug!("This is awarning");
            }
            Command::Sign(options) => println!("SignOpts {:?}", options),
        },
        None => debug!("no valid command given here"),
    }
    Ok(())
}
