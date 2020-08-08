use std::path::PathBuf;

use anyhow::Result;

use dftk_hugo_site::clean_site_dir;

// FIXME should provide feedback result
pub async fn run_clean(site_dir: Option<PathBuf>) -> Result<()> {
    info!("Run the clean command");

    if let Some(site_dir) = site_dir {
        clean_site_dir(site_dir).await?
    } else {
        debug!("No site dir to clean")
    }

    Ok(())
}
