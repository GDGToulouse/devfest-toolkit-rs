use core::fmt;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use log::warn;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use slug::slugify;

use crate::models::language::Lang;
use crate::models::session::category::CategoryKey;
use crate::models::session::format::FormatKey;
use crate::models::speaker::SpeakerKey;
use crate::models::Markdown;

pub mod category;
pub mod format;

/// Represent a session identifier
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: String) -> Self {
        SessionId::from_str(id.as_str()).unwrap()
    }
}

impl Into<String> for SessionId {
    fn into(self) -> String {
        self.0
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for SessionId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            Ok(SessionId(s.into()))
        } else {
            Err(anyhow!("Could not be empty"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SessionKey(String);

impl SessionKey {
    pub fn new(title: &str) -> Self {
        Self(slugify(title))
    }
}

impl Into<String> for SessionKey {
    fn into(self) -> String {
        self.0
    }
}

impl FromStr for SessionKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = SessionKey(s.into());

        Ok(key)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SessionLevel {
    All,
    Advanced,
    Expert,
}

impl From<String> for SessionLevel {
    fn from(str: String) -> Self {
        if str.to_lowercase() == "all" || str.to_lowercase() == "beginner" {
            SessionLevel::All
        } else if str.to_lowercase() == "advanced" || str.to_lowercase() == "intermediate" {
            SessionLevel::Advanced
        } else if str.to_lowercase() == "expert" {
            SessionLevel::Expert
        } else {
            warn!("No SessionLevel found for '{}'", str);
            SessionLevel::default()
        }
    }
}

impl Into<String> for SessionLevel {
    fn into(self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl Default for SessionLevel {
    fn default() -> Self {
        SessionLevel::All
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    id: SessionId,
    key: SessionKey,
    title: String,
    level: Option<SessionLevel>,
    format: FormatKey,
    speakers: Vec<SpeakerKey>,
    category: CategoryKey,
    language: Lang,
    video_id: Option<String>,
    presentation: Option<String>,
    draft: Option<bool>,
    office_hours: Option<Vec<SessionKey>>,
    description: Markdown,
}

impl Session {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: SessionId,
        title: String,
        level: Option<SessionLevel>,
        format: FormatKey,
        speakers: Vec<SpeakerKey>,
        category: CategoryKey,
        language: Lang,
        video_id: Option<String>,
        presentation: Option<String>,
        draft: Option<bool>,
        office_hours: Option<Vec<SessionKey>>,
        description: Markdown,
    ) -> Self {
        let key = SessionKey::new(title.as_str());
        Self {
            id,
            key,
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
        }
    }

    pub fn id(&self) -> SessionId {
        self.id.clone()
    }
    pub fn key(&self) -> SessionKey {
        self.key.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn level(&self) -> Option<SessionLevel> {
        self.level
    }
    pub fn format(&self) -> FormatKey {
        self.format.clone()
    }
    pub fn speakers(&self) -> Vec<SpeakerKey> {
        self.speakers.clone()
    }
    pub fn category(&self) -> CategoryKey {
        self.category.clone()
    }
    pub fn language(&self) -> Lang {
        self.language.clone()
    }
    pub fn video_id(&self) -> Option<String> {
        self.video_id.clone()
    }
    pub fn presentation(&self) -> Option<String> {
        self.presentation.clone()
    }
    pub fn draft(&self) -> Option<bool> {
        self.draft
    }
    pub fn office_hours(&self) -> Option<Vec<SessionKey>> {
        self.office_hours.clone()
    }
    pub fn description(&self) -> Markdown {
        self.description.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialSession {
    title: String,
    level: Option<SessionLevel>,
    format: FormatKey,
    speakers: Vec<SpeakerKey>,
    category: CategoryKey,
    language: Lang,
    video_id: Option<String>,
    presentation: Option<String>,
    draft: Option<bool>,
    office_hours: Option<Vec<SessionKey>>,
    description: Markdown,
}

impl PartialSession {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: String,
        level: Option<SessionLevel>,
        format: FormatKey,
        speakers: Vec<SpeakerKey>,
        category: CategoryKey,
        language: Lang,
        video_id: Option<String>,
        presentation: Option<String>,
        draft: Option<bool>,
        office_hours: Option<Vec<SessionKey>>,
        description: Markdown,
    ) -> Self {
        Self {
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
        }
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn level(&self) -> Option<SessionLevel> {
        self.level
    }
    pub fn format(&self) -> FormatKey {
        self.format.clone()
    }
    pub fn speakers(&self) -> Vec<SpeakerKey> {
        self.speakers.clone()
    }
    pub fn category(&self) -> CategoryKey {
        self.category.clone()
    }
    pub fn language(&self) -> Lang {
        self.language.clone()
    }
    pub fn video_id(&self) -> Option<String> {
        self.video_id.clone()
    }
    pub fn presentation(&self) -> Option<String> {
        self.presentation.clone()
    }
    pub fn draft(&self) -> Option<bool> {
        self.draft
    }
    pub fn office_hours(&self) -> Option<Vec<SessionKey>> {
        self.office_hours.clone()
    }
    pub fn description(&self) -> Markdown {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod session_id {
        use super::*;

        #[test]
        fn could_be_created_from_str() {
            let given = "plop";
            let result = SessionId::from_str(given).unwrap();
            assert_eq!(result.0, String::from(given));
        }

        #[test]
        #[should_panic]
        fn could_not_be_created_from_empty_str() {
            let given = "";
            SessionId::from_str(given).unwrap();
        }

        #[test]
        fn could_be_transform_to_string() {
            let string = String::from("plop");
            let given = SessionId(string.clone());
            let result: String = given.into();
            assert_eq!(result, string);
        }

        #[test]
        fn could_be_display() {
            let given = SessionId(String::from("plop"));
            println!("{}", given);
        }
    }
}
