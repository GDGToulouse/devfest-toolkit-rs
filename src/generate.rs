use anyhow::Result;
use log::{debug, info};

use dftk_database::{MongodbConfig, Repositories};
use dftk_hugo_site::site_writer::GenerateResult;
use dftk_hugo_site::{generate, SiteConfig};

pub async fn run_generate(
    site_config: &SiteConfig,
    mongo_config: &MongodbConfig,
) -> Result<GenerateResult> {
    let database = mongo_config.database.clone();
    info!(
        "Generate site from {} to dir {:?}",
        database,
        site_config.site_dir.clone()
    );
    let repos = Repositories::build(mongo_config).await?;
    let site = repos.load_site().await?;

    debug!("Writing site to {:?}", site_config.site_dir);
    let result = generate(site_config, site).await?;

    Ok(result)
}
