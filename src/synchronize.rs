use anyhow::Result;
use log::{debug, info};

use dftk_conference_hall::{read_event, ConferenceHallConfig};
use dftk_database::{MongodbConfig, Repositories, SynchronizeResult};

pub async fn run_synchronize(
    ch_config: &ConferenceHallConfig,
    mongo_config: &MongodbConfig,
) -> Result<SynchronizeResult> {
    info!(
        "Synchronize conference_hall data to DB {}",
        mongo_config.database
    );
    let repos = Repositories::build(mongo_config).await?;
    debug!("Loading Event from conference_hall...");
    let site = read_event(&ch_config).await?;
    let result = repos.synchronize(site).await?;

    Ok(result)
}
