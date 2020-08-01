use core::fmt;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use dftk_common::models::language::{Lang, Languages};
use dftk_common::models::session::category::{CategoryKey, SessionCategory};
use dftk_common::models::session::format::{FormatKey, SessionFormat};
use dftk_common::models::session::{Session, SessionId};
use dftk_common::models::site::{Address, DateRange, EventId, Geolocation, Name, Site, SiteInfo};
use dftk_common::models::socials::Social;
use dftk_common::models::speaker::{Speaker, SpeakerId, SpeakerKey};

#[derive(Deserialize, Debug, Clone)]
pub struct ChEvent {
    name: String,
    categories: Vec<ChDescription>,
    formats: Vec<ChDescription>,
    address: ChAddresses,
    #[serde(rename = "conferenceDates")]
    conference_dates: ChConferenceDates,
    talks: Vec<ChTalk>,
    speakers: Vec<ChSpeaker>,
}

impl ChEvent {
    pub fn to_site(&self, id: EventId, languages: Languages) -> Site {
        let info = self.site_info(id, languages);

        let mut formats = vec![];
        let mut format_map: HashMap<Uuid, FormatKey> = HashMap::new();
        let default_format = SessionFormat::default();
        for format in self.formats.iter() {
            let site_format: SessionFormat = format.clone().into();
            formats.push(site_format.clone());
            format_map.insert(format.id, site_format.key());
        }

        let mut categories = vec![];
        let mut category_map: HashMap<Uuid, CategoryKey> = HashMap::new();
        let default_category = SessionCategory::default();
        for category in self.categories.iter() {
            let site_category: SessionCategory = category.clone().into();
            categories.push(site_category.clone());
            category_map.insert(category.id, site_category.key());
        }

        let mut speakers = vec![];
        let mut speaker_map: HashMap<SpeakerId, SpeakerKey> = HashMap::new();
        for speaker in self.speakers.iter() {
            let site_speaker: Speaker = speaker.clone().into();
            speakers.push(site_speaker.clone());
            speaker_map.insert(site_speaker.id(), site_speaker.key());
        }

        // FIXME take only accepted
        let sessions: Vec<Session> = self
            .talks
            .iter()
            .map(|talk| {
                let category = talk
                    .categories
                    .and_then(|it| category_map.get(&it))
                    .cloned()
                    .unwrap_or_else(|| default_category.key());

                let format = talk
                    .formats
                    .and_then(|it| format_map.get(&it))
                    .cloned()
                    .unwrap_or_else(|| default_format.key());

                let speaker_keys = talk
                    .speakers
                    .iter()
                    .map(|it| speaker_map.get(it).unwrap())
                    .cloned()
                    .collect();

                talk.to_session(speaker_keys, format, category)
            })
            .collect();

        Site::new(
            info,
            sessions,
            speakers,
            categories,
            formats,
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        )
    }

    fn site_info(&self, id: EventId, languages: Languages) -> SiteInfo {
        let name = self.name.clone();
        let address = self.address.clone().into();
        let dates = self.conference_dates.clone().into();

        SiteInfo::new(id, name, address, languages, dates)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChDescription {
    id: Uuid,
    name: String,
    description: Option<String>,
}

impl ChDescription {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Into<SessionCategory> for ChDescription {
    fn into(self) -> SessionCategory {
        let ChDescription {
            id,
            name,
            description,
        } = self;
        let description = description.map(|s| s.into());
        let key = CategoryKey::new(name.as_str());

        SessionCategory::new(id, key, name, description)
    }
}

impl Into<SessionFormat> for ChDescription {
    fn into(self) -> SessionFormat {
        let ChDescription {
            id,
            name,
            description,
        } = self;
        let key = FormatKey::new(name.as_str());

        SessionFormat::new(id, key, name, description.map(|it| it.into()))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChName {
    short_name: String,
    long_name: String,
}

impl Into<Name> for ChName {
    fn into(self) -> Name {
        let ChName {
            long_name,
            short_name,
        } = self;

        Name::new(long_name, short_name)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChLatLng {
    lat: f64,
    lng: f64,
}

impl Into<Geolocation> for ChLatLng {
    fn into(self) -> Geolocation {
        let ChLatLng { lat, lng } = self;

        Geolocation::new(lat, lng)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChAddresses {
    locality: ChName,
    country: ChName,
    #[serde(rename = "latLng")]
    lat_lng: ChLatLng,
    #[serde(rename = "formattedAddress")]
    formatted_address: String,
}

impl Into<Address> for ChAddresses {
    fn into(self) -> Address {
        let locality = self.locality.into();
        let country = self.country.into();
        let lat_lng = self.lat_lng.into();

        Address::new(locality, country, lat_lng)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChConferenceDates {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl Into<DateRange> for ChConferenceDates {
    fn into(self) -> DateRange {
        let ChConferenceDates { start, end } = self;

        DateRange::new(start, end)
    }
}

#[derive(Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChTalkState {
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChTalk {
    id: SessionId,
    title: String,
    state: ChTalkState,
    level: Option<String>,
    #[serde(rename = "abstract")]
    _abstract: String,
    categories: Option<Uuid>,
    formats: Option<Uuid>,
    speakers: Vec<SpeakerId>,
    language: Option<String>,
}

impl Display for ChTalk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.id, self.title)
    }
}

impl ChTalk {
    pub fn id(&self) -> SessionId {
        self.id.clone()
    }
    pub fn state(&self) -> ChTalkState {
        self.state
    }

    pub fn to_session(
        &self,
        speakers: Vec<SpeakerKey>,
        format: FormatKey,
        category: CategoryKey,
    ) -> Session {
        let id = self.id.clone();
        let title = self.title.clone();
        let level = match self.level.clone() {
            Some(it) => Some(it.into()),
            None => None,
        };
        let language = self
            .language
            .as_ref()
            .map(|it| Lang::from_user_field(it.as_str()))
            .unwrap_or_else(Lang::default);
        let video_id = None;
        let presentation = None;
        let draft = match self.state {
            ChTalkState::Accepted => Some(false),
            _ => Some(true),
        };
        let office_hours = None;
        let description = self._abstract.clone().into();

        Session::new(
            id,
            title,
            level,
            format,
            speakers,
            category,
            language,
            video_id,
            presentation,
            draft,
            office_hours,
            description,
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChSpeaker {
    uid: SpeakerId,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    bio: Option<String>,
    company: Option<String>,
    #[serde(rename = "photoURL")]
    photo_url: String,
    twitter: Option<String>,
    github: Option<String>,
}

impl Display for ChSpeaker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}",
            self.uid,
            self.display_name.clone().unwrap_or_else(|| "".into())
        )
    }
}

impl ChSpeaker {
    pub fn id(&self) -> SpeakerId {
        self.uid.clone()
    }

    pub fn key(&self) -> SpeakerKey {
        let s = self
            .display_name
            .clone()
            .unwrap_or_else(|| self.uid.clone().into());

        SpeakerKey::new(s.as_str())
    }
}

impl Into<Speaker> for ChSpeaker {
    fn into(self) -> Speaker {
        let id = self.uid.clone();
        let name = self.display_name.clone().unwrap_or_else(|| "".into());
        let featured = false;
        let company = self.company.clone();
        let city = None;
        let photo_url = self.photo_url.clone().into();
        let mut socials = vec![];
        if let Some(twitter) = self.twitter.clone() {
            socials.push(Social::Twitter(twitter));
        }
        if let Some(github) = self.github.clone() {
            socials.push(Social::GitHub(github));
        }
        let draft = None;
        let description = self.bio.unwrap_or_else(|| "".into()).into();

        Speaker::new(
            id,
            name,
            featured,
            company,
            city,
            photo_url,
            socials,
            draft,
            description,
        )
    }
}
