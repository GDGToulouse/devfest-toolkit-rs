use anyhow::{bail, ensure, Result};
use log::{debug, info};
use mongodb::bson::doc;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use dftk_common::models::language::Lang;
use dftk_common::models::session::category::CategoryKey;
use dftk_common::models::session::format::FormatKey;
use dftk_common::models::session::{PartialSession, Session, SessionId, SessionKey, SessionLevel};
use dftk_common::models::speaker::SpeakerKey;
use dftk_common::models::Markdown;
use dftk_common::new_id;

use crate::cursor_to_vec;
use crate::repository::MongodbRepository;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SessionDocument {
    #[serde(rename = "_id")]
    id: SessionId,
    key: SessionKey,
    session: Option<Session>,
    patch: SessionPatch,
}

impl SessionDocument {
    fn merge(&self, new_session: &Session) -> Self {
        let id = self.id();
        let key = self.key.clone();
        let session = Some(new_session.clone());
        let patch = self.patch.clone();
        Self {
            id,
            key,
            session,
            patch,
        }
    }

    fn validate(&self) -> Result<()> {
        if self.session.is_none() {
            ensure!(self.patch.title.is_some(), "only some is allowed");
            ensure!(self.patch.format.is_some(), "only some is allowed");
            ensure!(self.patch.speakers.is_some(), "only some is allowed");
            ensure!(self.patch.category.is_some(), "only some is allowed");
            ensure!(self.patch.language.is_some(), "only some is allowed");
            ensure!(self.patch.description.is_some(), "only some is allowed");
        }

        Ok(())
    }

    pub fn id(&self) -> SessionId {
        self.id.clone()
    }
    pub fn session(&self) -> Option<Session> {
        self.session.clone()
    }
    pub fn patch(&self) -> SessionPatch {
        self.patch.clone()
    }
}

impl From<Session> for SessionDocument {
    fn from(session: Session) -> Self {
        let id = session.id();
        let key = session.key();
        let session = Some(session);
        let patch = SessionPatch::default();

        Self {
            id,
            key,
            session,
            patch,
        }
    }
}

impl Into<Session> for SessionDocument {
    fn into(self) -> Session {
        let id = self.id();
        let title = match self.patch.title {
            Some(t) => t,
            None => self.session.clone().unwrap().title(),
        };
        let level = match self.patch.level {
            Some(t) => Some(t),
            None => self.session.clone().unwrap().level(),
        };
        let format = match self.patch.format {
            Some(t) => t,
            None => self.session.clone().unwrap().format(),
        };
        let speakers = match self.patch.speakers {
            Some(t) => t,
            None => self.session.clone().unwrap().speakers(),
        };
        let category = match self.patch.category {
            Some(t) => t,
            None => self.session.clone().unwrap().category(),
        };
        let language = match self.patch.language {
            Some(t) => t,
            None => self.session.clone().unwrap().language(),
        };
        let video_id = match self.patch.video_id {
            Some(t) => Some(t),
            None => self.session.clone().unwrap().video_id(),
        };
        let presentation = match self.patch.presentation {
            Some(t) => Some(t),
            None => self.session.clone().unwrap().presentation(),
        };
        let draft = match self.patch.draft {
            Some(t) => Some(t),
            None => self.session.clone().unwrap().draft(),
        };
        let office_hours = match self.patch.office_hours {
            Some(t) => Some(t),
            None => self.session.clone().unwrap().office_hours(),
        };
        let description = match self.patch.description {
            Some(t) => t,
            None => self.session.clone().unwrap().description(),
        };

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

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SessionPatch {
    title: Option<String>,
    level: Option<SessionLevel>,
    format: Option<FormatKey>,
    speakers: Option<Vec<SpeakerKey>>,
    category: Option<CategoryKey>,
    language: Option<Lang>,
    video_id: Option<String>,
    presentation: Option<String>,
    draft: Option<bool>,
    office_hours: Option<Vec<SessionKey>>,
    description: Option<Markdown>,
}

impl SessionPatch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: Option<String>,
        level: Option<SessionLevel>,
        format: Option<FormatKey>,
        speakers: Option<Vec<SpeakerKey>>,
        category: Option<CategoryKey>,
        language: Option<Lang>,
        video_id: Option<String>,
        presentation: Option<String>,
        draft: Option<bool>,
        office_hours: Option<Vec<SessionKey>>,
        description: Option<Markdown>,
    ) -> Self {
        SessionPatch {
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

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn level(&self) -> Option<SessionLevel> {
        self.level
    }

    pub fn format(&self) -> Option<FormatKey> {
        self.format.clone()
    }

    pub fn speakers(&self) -> Option<Vec<SpeakerKey>> {
        self.speakers.clone()
    }

    pub fn category(&self) -> Option<CategoryKey> {
        self.category.clone()
    }

    pub fn language(&self) -> Option<Lang> {
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
    pub fn description(&self) -> Option<Markdown> {
        self.description.clone()
    }
}

impl From<PartialSession> for SessionPatch {
    fn from(ps: PartialSession) -> Self {
        Self::new(
            Some(ps.title()),
            ps.level(),
            Some(ps.format()),
            Some(ps.speakers()),
            Some(ps.category()),
            Some(ps.language()),
            ps.video_id(),
            ps.presentation(),
            ps.draft(),
            ps.office_hours(),
            Some(ps.description()),
        )
    }
}

#[derive(Clone)]
pub struct SessionRepository {
    col: Collection,
    repo: MongodbRepository<SessionDocument>,
}

impl SessionRepository {
    pub fn new(db: &Database) -> Self {
        let col_name = "sessions";
        let col = db.collection(col_name);
        let repo = MongodbRepository::new(db, col_name);

        Self { col, repo }
    }

    pub async fn find_all(&self) -> Result<Vec<Session>> {
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

    pub async fn find_by_id(&self, id: SessionId) -> Result<Option<SessionDocument>> {
        let sid: String = id.into();
        self.repo.find_by_id(sid.as_str()).await
    }

    // From Conference Hall
    pub async fn synchronize_sessions(&self, sessions: &[Session]) -> Result<Vec<Session>> {
        info!("Synchronize site sessions");
        let mut result = vec![];
        for ses in sessions {
            debug!("Synchronize {:?}", ses);

            let id: String = ses.id().into();
            let maybe_session = self.repo.find_by_id(id.as_str()).await?;
            let session = match maybe_session {
                Some(doc) => {
                    let to_update = doc.merge(ses);
                    self.repo.update(id.as_str(), &to_update).await?;
                    to_update
                }
                None => {
                    let to_create = ses.clone().into();
                    self.repo.insert(&to_create).await?;
                    to_create
                }
            };
            result.push(session.into());
        }

        Ok(result)
    }

    // From IHM

    pub async fn find_by_speaker(&self, key: &SpeakerKey) -> Result<Vec<Session>> {
        let s: String = key.clone().into();
        let query = doc! {
            "$or": [
               { "patch.speakers": &s },
               { "session.speakers": &s }
            ]
        };
        let mut cursor = self.col.find(query, None).await?;
        let result = cursor_to_vec::<SessionDocument>(&mut cursor).await?;
        let result = result
            .iter()
            .cloned()
            .map(|it| it.into())
            // Need to double check when patched
            .filter(|it: &Session| it.speakers().contains(key))
            .collect();

        Ok(result)
    }

    pub async fn find_speakers(&self, key: &SessionKey) -> Result<Vec<SpeakerKey>> {
        let key: String = key.clone().into();
        let session = self.repo.find_by_key(key.as_str()).await?;
        let result = match session {
            Some(s) => {
                let session: Session = s.into();
                session.speakers()
            }
            None => vec![],
        };

        Ok(result)
    }

    pub async fn insert_session(&self, input: PartialSession) -> Result<SessionDocument> {
        let id = SessionId::new(new_id().to_string());
        let patch: SessionPatch = input.into();
        let key = SessionKey::new(patch.clone().title.unwrap().as_str());
        let session = SessionDocument {
            id,
            key,
            session: None,
            patch,
        };
        session.validate()?;
        self.repo.insert(&session).await?;

        Ok(session)
    }

    pub async fn update_session(
        &self,
        id: SessionId,
        patch: SessionPatch,
    ) -> Result<SessionDocument> {
        let sid: String = id.clone().into();
        let option = self.repo.find_by_id(sid.as_str()).await?;
        let session = match option {
            Some(sd) => sd,
            None => bail!("No session found with id {}", id),
        };
        let key = session.key.clone();
        let updated = SessionDocument {
            id: session.id.clone(),
            key,
            session: session.session,
            patch,
        };
        updated.validate()?;
        self.repo.save_or_update(sid.as_str(), &updated).await?;

        Ok(updated)
    }

    pub async fn delete_session(&self, id: SessionId) -> Result<Option<SessionDocument>> {
        let sid: String = id.into();
        let result = self.repo.remove_by_id(sid.as_str()).await?;

        Ok(result)
    }
}
