use anyhow::Result;
use log::info;

use dftk_common::models::language::Languages;
use dftk_common::models::site::{EventId, Site};

use crate::models::ChEvent;

pub mod models;

pub async fn read_event(config: &ConferenceHallConfig) -> Result<Site> {
    let ConferenceHallConfig {
        url,
        event_id,
        api_key,
    } = config;
    let client = reqwest::Client::new();
    info!("Find event {} info from conference_hall", event_id);
    let event_id: String = event_id.clone().into();
    let url = format!("{}/api/v1/event/{}", url, event_id);

    let response = client.get(&url).query(&[("key", api_key)]).send().await?;

    let event = response.json::<ChEvent>().await?;
    let result = event.to_site(EventId::new(event_id), Languages::default());

    Ok(result)
}

#[derive(Clone, Debug)]
pub struct ConferenceHallConfig {
    pub url: String,
    pub event_id: EventId,
    pub api_key: String,
}

impl ConferenceHallConfig {
    pub fn new(url: String, event_id: String, api_key: String) -> Self {
        let event_id = EventId::new(event_id);

        Self {
            url,
            event_id,
            api_key,
        }
    }
}
