use anyhow::Result;
use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;

use crate::clean::run_clean;
use crate::generate::run_generate;
use crate::opts::Command;
use crate::synchronize::run_synchronize;
use dftk_server::{run_server, ServerContext};

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

pub fn configure_log(debug: bool) -> Result<()> {
    let colors = ColoredLevelConfig::new()
        // use builder methods
        .debug(Color::Green)
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);

    let level = if debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            // let target = record.target().split("::").last().unwrap_or_default();
            let target = record.target();

            out.finish(format_args!(
                "{} [{:<5}] {} - {}",
                chrono::Local::now().format("%H:%M:%S.%3f"),
                colors.color(record.level()),
                target.cyan(),
                message
            ))
        })
        .level(level)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
