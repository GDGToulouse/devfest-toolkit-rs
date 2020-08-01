use anyhow::{bail, ensure, Result};
use log::{debug, info};
use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use dftk_common::models::socials::Social;
use dftk_common::models::speaker::{PartialSpeaker, Speaker, SpeakerId, SpeakerKey};
use dftk_common::models::Markdown;
use dftk_common::new_id;

use crate::repository::MongodbRepository;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpeakerDocument {
    #[serde(rename = "_id")]
    id: SpeakerId,
    key: SpeakerKey,
    speaker: Option<Speaker>,
    patch: SpeakerPatch,
}

impl SpeakerDocument {
    fn merge(&self, new_speaker: &Speaker) -> Self {
        let id = self.id();
        let key = self.key.clone();
        let speaker = Some(new_speaker.clone());
        let patch = self.patch.clone();

        Self {
            id,
            key,
            speaker,
            patch,
        }
    }

    pub fn id(&self) -> SpeakerId {
        self.id.clone()
    }
    pub fn speaker(&self) -> Option<Speaker> {
        self.speaker.clone()
    }
    pub fn patch(&self) -> SpeakerPatch {
        self.patch.clone()
    }

    fn validate(&self) -> Result<()> {
        if self.speaker.is_none() {
            ensure!(self.patch.name.is_some(), "only some is allowed");
            ensure!(self.patch.description.is_some(), "only some is allowed");
        }

        Ok(())
    }
}

impl From<Speaker> for SpeakerDocument {
    fn from(speaker: Speaker) -> Self {
        let id = speaker.id();
        let key = speaker.key();
        let speaker = Some(speaker);
        let patch = SpeakerPatch::default();

        Self {
            id,
            key,
            speaker,
            patch,
        }
    }
}

impl Into<Speaker> for SpeakerDocument {
    fn into(self) -> Speaker {
        let id = self.id.clone();
        let name = match self.patch.name {
            Some(t) => t,
            None => self.speaker.clone().unwrap().name(),
        };
        let featured = match self.patch.featured {
            Some(t) => t,
            None => self.speaker.clone().unwrap().featured(),
        };
        let company = match self.patch.company {
            Some(t) => Some(t),
            None => self.speaker.clone().unwrap().company(),
        };
        let city = match self.patch.city {
            Some(t) => Some(t),
            None => self.speaker.clone().unwrap().city(),
        };
        let photo_url = match self.patch.photo_url {
            Some(t) => Some(t),
            None => self.speaker.clone().unwrap().photo_url(),
        };
        let socials = match self.patch.socials {
            Some(t) => t,
            None => self.speaker.clone().unwrap().socials().to_vec(),
        };
        let draft = match self.patch.draft {
            Some(t) => Some(t),
            None => self.speaker.clone().unwrap().draft(),
        };
        let description = match self.patch.description {
            Some(t) => t,
            None => self.speaker.clone().unwrap().content(),
        };

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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SpeakerPatch {
    name: Option<String>,
    featured: Option<bool>,
    company: Option<String>,
    city: Option<String>,
    photo_url: Option<String>,
    socials: Option<Vec<Social>>,
    draft: Option<bool>,
    description: Option<Markdown>,
}

impl SpeakerPatch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Option<String>,
        featured: Option<bool>,
        company: Option<String>,
        city: Option<String>,
        photo_url: Option<String>,
        socials: Option<Vec<Social>>,
        draft: Option<bool>,
        description: Option<Markdown>,
    ) -> Self {
        SpeakerPatch {
            name,
            featured,
            company,
            city,
            photo_url,
            socials,
            draft,
            description,
        }
    }
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn featured(&self) -> Option<bool> {
        self.featured
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
    pub fn socials(&self) -> Option<Vec<Social>> {
        self.socials.clone()
    }
    pub fn draft(&self) -> Option<bool> {
        self.draft
    }
    pub fn description(&self) -> Option<Markdown> {
        self.description.clone()
    }
}

impl From<PartialSpeaker> for SpeakerPatch {
    fn from(ps: PartialSpeaker) -> Self {
        Self::new(
            Some(ps.name()),
            Some(ps.featured()),
            ps.company(),
            ps.city(),
            ps.photo_url(),
            Some(ps.socials().to_vec()),
            ps.draft(),
            Some(ps.description()),
        )
    }
}

#[derive(Clone)]
pub struct SpeakerRepository {
    repo: MongodbRepository<SpeakerDocument>,
}

impl SpeakerRepository {
    pub fn new(db: &Database) -> Self {
        let col_name = "speakers";
        let repo = MongodbRepository::new(db, col_name);

        Self { repo }
    }

    pub async fn find_all(&self) -> Result<Vec<Speaker>> {
        let result = self
            .repo
            .find_all()
            .await?
            .iter()
            .cloned()
            .map(|it| it.into())
            .collect();

        Ok(result)
    }
    pub async fn find_by_id(&self, id: SpeakerId) -> Result<Option<SpeakerDocument>> {
        let sid: String = id.into();
        self.repo.find_by_id(sid.as_str()).await
    }

    pub async fn find_by_key(&self, key: SpeakerKey) -> Result<Option<SpeakerDocument>> {
        let k: String = key.into();
        self.repo.find_by_key(k.as_str()).await
    }

    pub async fn find_by_keys(&self, keys: &[String]) -> Result<Vec<Speaker>> {
        let result = self
            .repo
            .find_by_keys(keys)
            .await?
            .iter()
            .cloned()
            .map(|it| it.into())
            .collect();

        Ok(result)
    }

    // From Conference Hall
    pub async fn synchronize_speakers(&self, speakers: &[Speaker]) -> Result<Vec<Speaker>> {
        info!("Synchronize site speaker");
        let mut result = vec![];
        for speaker in speakers {
            debug!("Synchronize {:?}", speaker);

            let id: String = speaker.id().into();
            let maybe_speaker = self.repo.find_by_id(id.as_str()).await?;
            let speaker = match maybe_speaker {
                Some(s) => {
                    let to_update = s.merge(speaker);
                    self.repo.update(id.as_str(), &to_update).await?;
                    to_update
                }
                None => {
                    let to_create: SpeakerDocument = speaker.clone().into();
                    self.repo.insert(&to_create).await?;
                    to_create
                }
            };
            result.push(speaker.into());
        }

        Ok(result)
    }

    // From UI
    pub async fn insert_speaker(&self, input: PartialSpeaker) -> Result<SpeakerDocument> {
        let id = SpeakerId::new(new_id().to_string());
        let patch: SpeakerPatch = input.into();
        let key = SpeakerKey::new(patch.clone().name.unwrap().as_str());
        let speaker = SpeakerDocument {
            id,
            key,
            speaker: None,
            patch,
        };
        speaker.validate()?;
        self.repo.insert(&speaker).await?;

        Ok(speaker)
    }

    pub async fn update_speaker(
        &self,
        id: SpeakerId,
        patch: SpeakerPatch,
    ) -> Result<SpeakerDocument> {
        let sid: String = id.clone().into();
        let option = self.repo.find_by_id(sid.as_str()).await?;
        let speaker = match option {
            Some(sd) => sd,
            None => bail!("No speaker found with id {}", id),
        };

        let updated = SpeakerDocument {
            id: speaker.id.clone(),
            key: speaker.key.clone(),
            speaker: speaker.speaker.clone(),
            patch,
        };
        updated.validate()?;
        let sid: String = speaker.id().into();
        self.repo.save_or_update(sid.as_str(), &updated).await?;

        Ok(updated)
    }

    pub async fn delete_speaker(&self, id: SpeakerId) -> Result<Option<SpeakerDocument>> {
        let sid = id.to_string();
        let result = self.repo.remove_by_id(sid.as_str()).await?;

        Ok(result)
    }
}
