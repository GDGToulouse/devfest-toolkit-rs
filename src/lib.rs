#[macro_use]
extern crate log;

use anyhow::Result;

use dftk_server::{run_server, ServerContext};

use crate::clean::run_clean;
use crate::generate::run_generate;
use crate::opts::Command;
use crate::synchronize::run_synchronize;

pub mod clean;
pub mod generate;
pub mod opts;
pub mod synchronize;

pub async fn run_command(command: Command) -> Result<()> {
    match command {
        Command::Serve {
            site_dir,
            conference_hall,
            mongodb,
            server,
        } => {
            let context = ServerContext::build(
                site_dir.into(),
                conference_hall.into(),
                mongodb.into(),
                server.into(),
            )
            .await?;
            run_server(context).await?
        }

        Command::Synchronize {
            conference_hall,
            mongodb,
        } => {
            // synchronize data
            let result = run_synchronize(&conference_hall.into(), &mongodb.into()).await?;
            info!("Synchronization result: {:?}", result);
        }

        Command::Generate { site_dir, mongodb } => {
            // generate site
            let result = run_generate(&site_dir.into(), &mongodb.into()).await?;
            info!("Generate result: {:?}", result);
        }

        Command::Clean { site_dir } => {
            // just clean
            run_clean(site_dir).await?
        }
    };

    Ok(())
}
