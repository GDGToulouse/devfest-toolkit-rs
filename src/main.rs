use anyhow::Result;
use structopt::StructOpt;

use devfest_toolkit_rs::opts::CliOpt;
use devfest_toolkit_rs::run_command;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
        pretty_env_logger::init();
        log::warn!("No RUST_LOG environment variable found, set log to 'info'")
    } else {
        pretty_env_logger::init();
    }
    let opt = CliOpt::from_args();
    run_command(opt.command()).await?;

    Ok(())
}
