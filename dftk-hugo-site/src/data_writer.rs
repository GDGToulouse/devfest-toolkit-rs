use std::fs::{create_dir_all, File};
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;

use crate::SiteConfig;

pub struct DataWriter {
    data_path: PathBuf,
    label: String,
}

impl DataWriter {
    pub fn new(config: &SiteConfig, label: &str) -> Self {
        let mut data_path = config.site_dir.clone();
        data_path.push("data");
        let label = String::from(label);

        Self { data_path, label }
    }

    pub async fn write_all<S>(&self, elements: &[S]) -> Result<()>
    where
        S: Serialize,
    {
        create_dir_all(self.data_path.clone())?;
        let mut file_path = self.data_path.clone();
        file_path.push(format!("{}.yml", self.label.clone().to_lowercase()));
        info!("Write all {} to {:?}", self.label.clone(), file_path);

        let file = File::create(file_path)?;
        serde_yaml::to_writer(file, elements)?;

        Ok(())
    }
}
