use anyhow::Result;
use structopt::StructOpt;

use devfest_toolkit_rs::opts::CliOpt;
use devfest_toolkit_rs::{configure_log, run_command};

#[tokio::main]
async fn main() -> Result<()> {
    let opt = CliOpt::from_args();
    configure_log(opt.debug())?;
    run_command(opt.command()).await?;

    Ok(())
}
