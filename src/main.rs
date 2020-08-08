use anyhow::Result;
use structopt::StructOpt;

use devfest_toolkit_rs::opts::CliOpt;
use devfest_toolkit_rs::run_command;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let opt = CliOpt::from_args();
    run_command(opt.command()).await?;

    Ok(())
}
