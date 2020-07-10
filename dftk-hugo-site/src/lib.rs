use std::fs::remove_file;
use std::path::PathBuf;

use anyhow::Result;
use log::{debug, info, trace, warn};

use dftk_common::models::site::Site;

use crate::markdown_writer::FrontMatterMarkdownWriter;
use crate::site_writer::{GenerateResult, SiteWriter};

pub mod data_writer;
pub mod markdown_writer;
pub mod models;
pub mod site_writer;

#[derive(Clone, Debug)]
pub struct SiteConfig {
    pub site_dir: PathBuf,
}

impl SiteConfig {
    pub fn new(site_dir: PathBuf) -> Self {
        Self { site_dir }
    }
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self::new(PathBuf::from("./_output/site"))
    }
}

fn new_speaker_writer(config: &SiteConfig) -> FrontMatterMarkdownWriter {
    let mut parent_path = config.site_dir.clone();
    parent_path.push("content");
    parent_path.push("speakers");

    FrontMatterMarkdownWriter::new("speakers", parent_path)
}

fn new_sponsor_writer(config: &SiteConfig) -> FrontMatterMarkdownWriter {
    let mut parent_path = config.site_dir.clone();
    parent_path.push("content");
    parent_path.push("partners");

    FrontMatterMarkdownWriter::new("partners", parent_path)
}

fn new_session_writer(config: &SiteConfig) -> FrontMatterMarkdownWriter {
    let mut parent_path = config.site_dir.clone();
    parent_path.push("content");
    parent_path.push("sessions");

    FrontMatterMarkdownWriter::new("sessions", parent_path)
}

fn new_team_writer(config: &SiteConfig) -> FrontMatterMarkdownWriter {
    let mut parent_path = config.site_dir.clone();
    parent_path.push("content");
    parent_path.push("team");

    FrontMatterMarkdownWriter::new("team", parent_path)
}

pub async fn generate(site_config: &SiteConfig, site: Site) -> Result<GenerateResult> {
    info!("Generate site to dir {:?}", site_config.site_dir.clone());

    debug!("Writing site to {:?}", site_config.site_dir);
    let writer = SiteWriter::new(&site_config);
    let result = writer.write_site(&site).await?;

    Ok(result)
}

pub async fn clean_site_dir(site_dir: PathBuf) -> Result<()> {
    debug!("Cleaning site dir {:?}", site_dir);
    let globs = vec![
        "content/partners/*.md",
        "content/sessions/*.md",
        "content/speakers/*.md",
        "content/team/*.md",
        "data/categories.yml",
        "data/formats.yml",
        "data/rooms.yml",
        "data/schedule.yml",
        "data/slots.yml",
        "static/site.json",
    ];

    for g in globs {
        let some_path = format!("{}/{}", site_dir.to_str().unwrap(), g);
        debug!("  cleaning {}", some_path);
        let files = glob::glob(some_path.as_str())?;
        for file in files {
            match file {
                Ok(path) => {
                    trace!("Removing {:?}", path);
                    remove_file(path).unwrap();
                }
                Err(err) => warn!("Oops, issue conference_hall file {:?}", err),
            }
        }
    }

    Ok(())
}
