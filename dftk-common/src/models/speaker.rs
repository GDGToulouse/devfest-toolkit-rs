use core::fmt;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::models::socials::Social;
use crate::models::Markdown;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, Eq, PartialEq)]
pub struct SpeakerId(String);

impl SpeakerId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Display for SpeakerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<String> for SpeakerId {
    fn into(self) -> String {
        self.0
    }
}

impl FromStr for SpeakerId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            Ok(SpeakerId(s.into()))
        } else {
            Err(anyhow!("Could not be empty"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SpeakerKey(String);

impl SpeakerKey {
    pub fn new(name: &str) -> Self {
        Self(slugify(name))
    }
}

impl Into<String> for SpeakerKey {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Speaker {
    _id: SpeakerId,
    key: SpeakerKey,
    featured: bool,
    name: String,
    company: Option<String>,
    city: Option<String>,
    photo_url: Option<String>,
    socials: Vec<Social>,
    draft: Option<bool>,
    description: Markdown,
}

impl Speaker {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: SpeakerId,
        name: String,
        featured: bool,
        company: Option<String>,
        city: Option<String>,
        photo_url: Option<String>,
        socials: Vec<Social>,
        draft: Option<bool>,
        description: Markdown,
    ) -> Self {
        let key = SpeakerKey::new(name.as_str());

        Self {
            _id: id,
            key,
            featured,
            name,
            company,
            city,
            photo_url,
            socials,
            draft,
            description,
        }
    }

    pub fn id(&self) -> SpeakerId {
        self._id.clone()
    }
    pub fn key(&self) -> SpeakerKey {
        self.key.clone()
    }

    pub fn featured(&self) -> bool {
        self.featured
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn company(&self) -> Option<String> {
        self.company.clone()
    }
    pub fn city(&self) -> Option<String> {
        self.city.clone()
    }
    pub fn photo_url(&self) -> Option<String> {
        self.photo_url.clone()
    }
    pub fn socials(&self) -> &[Social] {
        self.socials.as_slice()
    }
    pub fn draft(&self) -> Option<bool> {
        self.draft
    }

    pub fn content(&self) -> Markdown {
        self.description.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialSpeaker {
    featured: bool,
    name: String,
    company: Option<String>,
    city: Option<String>,
    photo_url: Option<String>,
    socials: Vec<Social>,
    draft: Option<bool>,
    description: Markdown,
}

impl PartialSpeaker {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        featured: bool,
        company: Option<String>,
        city: Option<String>,
        photo_url: Option<String>,
        socials: Vec<Social>,
        draft: Option<bool>,
        description: Markdown,
    ) -> Self {
        Self {
            featured,
            name,
            company,
            city,
            photo_url,
            socials,
            draft,
            description,
        }
    }

    pub fn featured(&self) -> bool {
        self.featured
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn company(&self) -> Option<String> {
        self.company.clone()
    }
    pub fn city(&self) -> Option<String> {
        self.city.clone()
    }
    pub fn photo_url(&self) -> Option<String> {
        self.photo_url.clone()
    }
    pub fn socials(&self) -> &[Social] {
        self.socials.as_slice()
    }
    pub fn draft(&self) -> Option<bool> {
        self.draft
    }
    pub fn description(&self) -> Markdown {
        self.description.clone()
    }
}
