use anyhow::{anyhow, Result};
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject};
use uuid::Uuid;

use dftk_common::models::language::Lang;
use dftk_common::models::session::category::{CategoryKey, SessionCategory};
use dftk_common::models::session::format::{FormatKey, SessionFormat};
use dftk_common::models::session::{PartialSession, Session, SessionId, SessionKey};
use dftk_common::models::speaker::SpeakerKey;
use dftk_database::sessions::{SessionDocument, SessionPatch};
use dftk_database::{Repositories, SynchronizeResult};
use dftk_hugo_site::site_writer::GenerateResult;

use crate::graphql::categories::CategoryOutputType;
use crate::graphql::formats::FormatOutputType;
use crate::graphql::speakers::SpeakerOutputType;

pub struct SessionOutputType {
    session: Session,
}

#[Object]
impl SessionOutputType {
    async fn id(&self) -> SessionId {
        self.session.id()
    }
    async fn key(&self) -> SessionKey {
        self.session.key()
    }
    async fn title(&self) -> String {
        self.session.title()
    }
    async fn level(&self) -> Option<String> {
        self.session.level().map(|it| it.into())
    }
    async fn language(&self) -> Lang {
        self.session.language()
    }
    async fn video_id(&self) -> Option<String> {
        self.session.video_id()
    }
    async fn presentation(&self) -> Option<String> {
        self.session.presentation()
    }
    async fn draft(&self) -> Option<bool> {
        self.session.draft()
    }
    async fn description(&self) -> String {
        self.session.description().into()
    }

    async fn format(&self, ctx: &Context<'_>) -> FieldResult<FormatOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let key: String = self.session.format().into();
        let format = repos.session_format().find_by_key(key.as_str()).await?;
        let format = format.unwrap_or_else(|| {
            anyhow!("No format found for key {:?}", key);
            SessionFormat::default()
        });
        let format = format.into();

        Ok(format)
    }
    async fn category(&self, ctx: &Context<'_>) -> FieldResult<CategoryOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let key: String = self.session.category().into();
        let category = repos.session_category().find_by_key(key.as_str()).await?;
        let category = category.unwrap_or_else(|| {
            anyhow!("No category found for key {:?}", key);
            SessionCategory::default()
        });
        let category = category.into();

        Ok(category)
    }

    async fn speakers(&self, ctx: &Context<'_>) -> FieldResult<Vec<SpeakerOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let keys = self
            .session
            .speakers()
            .iter()
            .map(|it| it.clone().into())
            .collect::<Vec<String>>();
        let speakers = repos.speaker().find_by_keys(keys.as_slice()).await?;
        let speakers = speakers.iter().map(|it| it.into()).collect();

        Ok(speakers)
    }
}

impl From<&SessionDocument> for SessionOutputType {
    fn from(doc: &SessionDocument) -> Self {
        let session = doc.clone().into();

        Self { session }
    }
}

impl From<&Session> for SessionOutputType {
    fn from(session: &Session) -> Self {
        let session = session.clone();

        Self { session }
    }
}

#[SimpleObject]
pub struct GenerateResultOutputType {
    nb_sessions: u32,
    nb_speakers: u32,
    nb_sponsors: u32,
    nb_team: u32,
}

impl From<GenerateResult> for GenerateResultOutputType {
    fn from(gr: GenerateResult) -> Self {
        Self {
            nb_sessions: gr.nb_sessions(),
            nb_speakers: gr.nb_speakers(),
            nb_sponsors: gr.nb_sponsors(),
            nb_team: gr.nb_team(),
        }
    }
}

#[SimpleObject]
pub struct SynchronizeResultOutputType {
    nb_categories: u32,
    nb_formats: u32,
    nb_sessions: u32,
    nb_speakers: u32,
}

impl From<SynchronizeResult> for SynchronizeResultOutputType {
    fn from(sr: SynchronizeResult) -> Self {
        Self {
            nb_categories: sr.nb_categories(),
            nb_formats: sr.nb_formats(),
            nb_sessions: sr.nb_sessions(),
            nb_speakers: sr.nb_speakers(),
        }
    }
}

#[SimpleObject]
pub struct SessionDocumentOutputType {
    id: SessionId,
    session: Option<SessionOutputType>,
    patch: SessionPatchOutputType,
}

impl From<SessionDocument> for SessionDocumentOutputType {
    fn from(doc: SessionDocument) -> Self {
        Self {
            id: doc.id(),
            session: doc.session().map(|session| SessionOutputType { session }),
            patch: doc.patch().into(),
        }
    }
}

#[SimpleObject]
pub struct SessionPatchOutputType {
    title: Option<String>,
    level: Option<String>,
    format: Option<String>,
    speakers: Option<Vec<String>>,
    category: Option<String>,
    language: Option<Lang>,
    video_id: Option<String>,
    presentation: Option<String>,
    draft: Option<bool>,
    description: Option<String>,
}

impl From<SessionPatch> for SessionPatchOutputType {
    fn from(s: SessionPatch) -> Self {
        Self {
            title: s.title(),
            level: s.level().map(|it| it.into()),
            format: s.format().map(|it| it.into()),
            speakers: s
                .speakers()
                .map(|it| it.iter().map(|it| it.clone().into()).collect()),
            category: s.category().map(|it| it.into()),
            language: s.language(),
            video_id: s.video_id(),
            presentation: s.presentation(),
            draft: s.draft(),
            description: s.description().map(|it| it.into()),
        }
    }
}

#[SimpleObject]
pub struct SessionCategoryOutputType {
    id: Uuid,
    key: CategoryKey,
    name: String,
    description: Option<String>,
}

impl From<SessionCategory> for SessionCategoryOutputType {
    fn from(sc: SessionCategory) -> Self {
        Self {
            id: sc.id(),
            key: sc.key(),
            name: sc.name(),
            description: sc.description().map(|it| it.into()),
        }
    }
}

#[SimpleObject]
pub struct SessionFormatOutputType {
    id: Uuid,
    key: FormatKey,
    name: String,
    description: Option<String>,
}

impl From<SessionFormat> for SessionFormatOutputType {
    fn from(sc: SessionFormat) -> Self {
        Self {
            id: sc.id(),
            key: sc.key(),
            name: sc.name(),
            description: sc.description().map(|it| it.into()),
        }
    }
}

#[InputObject]
pub struct SessionPatchInput {
    title: Option<String>,
    level: Option<String>,
    format: Option<String>,
    speakers: Option<Vec<String>>,
    category: Option<String>,
    language: Option<Lang>,
    video_id: Option<String>,
    presentation: Option<String>,
    draft: Option<bool>,
    description: Option<String>,
}

impl SessionPatchInput {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_session_patch(self) -> Result<SessionPatch> {
        let level = match self.level {
            None => None,
            Some(it) => Some(it.into()),
        };

        let result = SessionPatch::new(
            self.title.clone(),
            level,
            self.format.map(|it| FormatKey::new(it.as_str())),
            self.speakers
                .map(|it| it.iter().map(|it| SpeakerKey::new(it)).collect()),
            self.category.map(|it| CategoryKey::new(it.as_str())),
            self.language,
            self.video_id.clone(),
            self.presentation.clone(),
            self.draft,
            None,
            self.description.map(|it| it.into()),
        );

        Ok(result)
    }
}

#[InputObject]
pub struct SessionCreateInput {
    title: String,
    level: String,
    format: String,
    speakers: Vec<String>,
    category: String,
    language: Lang,
    draft: bool,
    description: String,
}

impl Into<PartialSession> for SessionCreateInput {
    fn into(self) -> PartialSession {
        let format = FormatKey::new(self.format.as_str());
        let speakers = self.speakers.iter().map(|it| SpeakerKey::new(it)).collect();
        let category = CategoryKey::new(self.category.as_str());
        let language = self.language.clone();
        let description = self.description.clone().into();

        PartialSession::new(
            self.title.clone(),
            Some(self.level.into()),
            format,
            speakers,
            category,
            language,
            None,
            None,
            Some(self.draft),
            None,
            description,
        )
    }
}
