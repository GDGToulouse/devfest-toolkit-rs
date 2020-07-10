use anyhow::Result;
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject};

use dftk_common::models::speaker::{PartialSpeaker, Speaker, SpeakerId, SpeakerKey};
use dftk_database::speakers::{SpeakerDocument, SpeakerPatch};
use dftk_database::Repositories;

use crate::graphql::sessions::SessionOutputType;
use crate::graphql::socials::{SocialInputType, SocialOutputType};

pub struct SpeakerOutputType {
    speaker: Speaker,
}

#[Object]
impl SpeakerOutputType {
    async fn id(&self) -> SpeakerId {
        self.speaker.id()
    }
    async fn key(&self) -> SpeakerKey {
        self.speaker.key()
    }
    async fn featured(&self) -> bool {
        self.speaker.featured()
    }
    async fn name(&self) -> String {
        self.speaker.name()
    }
    async fn company(&self) -> Option<String> {
        self.speaker.company()
    }
    async fn city(&self) -> Option<String> {
        self.speaker.city()
    }
    async fn photo_url(&self) -> Option<String> {
        self.speaker.photo_url()
    }
    async fn socials(&self) -> SocialOutputType {
        SocialOutputType::new(self.speaker.socials())
    }
    async fn draft(&self) -> Option<bool> {
        self.speaker.draft()
    }
    async fn description(&self) -> String {
        self.speaker.content().into()
    }

    async fn sessions(&self, ctx: &Context<'_>) -> FieldResult<Vec<SessionOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.session().find_by_speaker(&self.speaker.key()).await?;
        let result = result.iter().map(|it| it.into()).collect();

        Ok(result)
    }
}

impl From<&Speaker> for SpeakerOutputType {
    fn from(speaker: &Speaker) -> Self {
        let speaker = speaker.clone();

        Self { speaker }
    }
}

#[SimpleObject]
pub struct SpeakerDocumentOutputType {
    id: String,
    speaker: Option<SpeakerOutputType>,
    patch: SpeakerPatchOutputType,
}

impl From<SpeakerDocument> for SpeakerDocumentOutputType {
    fn from(doc: SpeakerDocument) -> Self {
        Self {
            id: doc.id().into(),
            speaker: doc.speaker().map(|speaker| SpeakerOutputType { speaker }),
            patch: doc.patch().into(),
        }
    }
}

#[SimpleObject]
pub struct SpeakerPatchOutputType {
    name: Option<String>,
    featured: Option<bool>,
    company: Option<String>,
    city: Option<String>,
    photo_url: Option<String>,
    socials: Option<SocialOutputType>,
    draft: Option<bool>,
    description: Option<String>,
}

impl From<SpeakerPatch> for SpeakerPatchOutputType {
    fn from(speaker: SpeakerPatch) -> Self {
        Self {
            name: speaker.name(),
            featured: speaker.featured(),
            company: speaker.company(),
            city: speaker.city(),
            photo_url: speaker.photo_url(),
            socials: speaker
                .socials()
                .map(|it| SocialOutputType::new(it.as_slice())),
            draft: speaker.draft(),
            description: speaker.description().map(|it| it.into()),
        }
    }
}

#[InputObject]
pub struct SpeakerCreateInput {
    featured: bool,
    name: String,
    company: Option<String>,
    city: Option<String>,
    photo_url: Option<String>,
    socials: SocialInputType,
    draft: bool,
    description: String,
}

impl Into<PartialSpeaker> for SpeakerCreateInput {
    fn into(self) -> PartialSpeaker {
        PartialSpeaker::new(
            self.name.clone(),
            self.featured,
            self.company.clone(),
            self.city.clone(),
            self.photo_url.clone(),
            self.socials.clone().into(),
            Some(self.draft),
            self.description.into(),
        )
    }
}

#[InputObject]
pub struct SpeakerPatchInput {
    name: Option<String>,
    featured: Option<bool>,
    company: Option<String>,
    city: Option<String>,
    photo_url: Option<String>,
    socials: Option<SocialInputType>,
    draft: Option<bool>,
    description: Option<String>,
}

impl SpeakerPatchInput {
    pub fn to_speaker_patch(&self) -> Result<SpeakerPatch> {
        let result = SpeakerPatch::new(
            self.name.clone(),
            self.featured,
            self.company.clone(),
            self.city.clone(),
            self.photo_url.clone(),
            self.socials.clone().map(|it| it.into()),
            self.draft,
            self.description.clone().map(|it| it.into()),
        );

        Ok(result)
    }
}
