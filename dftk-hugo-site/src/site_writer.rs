use std::fs::{create_dir_all, File};

use anyhow::Result;
use log::info;
use serde::Serialize;

use dftk_common::models::site::Site;

use crate::data_writer::DataWriter;
use crate::markdown_writer::FrontMatterMarkdownWriter;
use crate::{
    new_session_writer, new_speaker_writer, new_sponsor_writer, new_team_writer, SiteConfig,
};

#[derive(Serialize, Debug, Copy, Clone)]
pub struct GenerateResult {
    nb_sessions: u32,
    nb_speakers: u32,
    nb_sponsors: u32,
    nb_team: u32,
}

impl GenerateResult {
    pub fn nb_sessions(&self) -> u32 {
        self.nb_sessions
    }
    pub fn nb_speakers(&self) -> u32 {
        self.nb_speakers
    }
    pub fn nb_sponsors(&self) -> u32 {
        self.nb_sponsors
    }
    pub fn nb_team(&self) -> u32 {
        self.nb_team
    }
}

pub(crate) struct SiteWriter {
    config: SiteConfig,

    speaker_writer: FrontMatterMarkdownWriter,
    team_writer: FrontMatterMarkdownWriter,
    session_writer: FrontMatterMarkdownWriter,
    sponsor_writer: FrontMatterMarkdownWriter,

    category_data_writer: DataWriter,
    format_data_writer: DataWriter,
    room_data_writer: DataWriter,
    schedule_data_writer: DataWriter,
    slot_data_writer: DataWriter,
}

impl SiteWriter {
    pub fn new(config: &SiteConfig) -> Self {
        let config = config.clone();
        let speaker_writer = new_speaker_writer(&config);
        let team_writer = new_team_writer(&config);
        let session_writer = new_session_writer(&config);
        let sponsor_writer = new_sponsor_writer(&config);

        let category_data_writer = DataWriter::new(&config, "categories");
        let format_data_writer = DataWriter::new(&config, "formats");
        let room_data_writer = DataWriter::new(&config, "rooms");
        let schedule_data_writer = DataWriter::new(&config, "schedule");
        let slot_data_writer = DataWriter::new(&config, "slots");

        Self {
            config,
            speaker_writer,
            team_writer,
            session_writer,
            sponsor_writer,
            category_data_writer,
            format_data_writer,
            room_data_writer,
            schedule_data_writer,
            slot_data_writer,
        }
    }

    async fn write_site_json(&self, site: &Site) -> Result<()> {
        let mut site_path = self.config.site_dir.clone();
        site_path.push("static");
        create_dir_all(site_path.clone())?;
        let mut file_path = site_path;
        file_path.push("site.json");
        info!("Write the site to {:?}", file_path);

        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, site)?;

        Ok(())
    }

    pub async fn write_site(&self, site: &Site) -> Result<GenerateResult> {
        info!("Write site");

        // Write markdown
        let nb_speakers = self.speaker_writer.write_all(site.speakers())? as u32;
        let nb_sessions = self.session_writer.write_all(site.sessions())? as u32;
        let nb_sponsors = self.sponsor_writer.write_all(site.sponsors())? as u32;
        let nb_team = self.team_writer.write_all(site.team())? as u32;

        // Write session data
        self.category_data_writer
            .write_all(site.categories())
            .await?;
        self.format_data_writer.write_all(site.formats()).await?;
        // Write schedule data/
        self.schedule_data_writer.write_all(site.schedule()).await?;
        self.room_data_writer.write_all(site.rooms()).await?;
        self.slot_data_writer.write_all(site.slots()).await?;

        // Write json
        self.write_site_json(site).await?;

        // FIXME write iCal

        let result = GenerateResult {
            nb_speakers,
            nb_sessions,
            nb_sponsors,
            nb_team,
        };

        Ok(result)
    }
}
