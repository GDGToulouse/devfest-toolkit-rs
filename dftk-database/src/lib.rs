use std::fmt::Debug;

use anyhow::{anyhow, Context, Result};
use bson::Document;
use log::{debug, info};
use mongodb::{Client, Cursor};
use serde::{Deserialize, Serialize};
use tokio::stream::StreamExt;

use dftk_common::acl::operation::Operation;
use dftk_common::acl::user::User;
use dftk_common::models::schedule::{Room, ScheduleDay, Slot};
use dftk_common::models::site::{Site, SiteInfo};
use dftk_common::models::speaker::SpeakerKey;

use crate::repository::MongodbRepository;
use crate::session_categories::SessionCategoryRepository;
use crate::session_formats::SessionFormatRepository;
use crate::sessions::SessionRepository;
use crate::speakers::SpeakerRepository;
use crate::sponsor_type::SponsorCategoryRepository;
use crate::sponsors::SponsorRepository;
use crate::team_member_types::MemberTypeRepository;
use crate::team_members::TeamMemberRepository;
use crate::user::UserRepository;

pub mod repository;
pub mod session_categories;
pub mod session_formats;
pub mod sessions;
pub mod speakers;
pub mod sponsor_type;
pub mod sponsors;
pub mod team_member_types;
pub mod team_members;
pub mod user;

#[derive(Clone, Debug)]
pub struct MongodbConfig {
    pub url: String,
    pub database: String,
}

impl MongodbConfig {
    pub fn new(url: String, database: String) -> Self {
        Self { url, database }
    }
}

impl Default for MongodbConfig {
    fn default() -> Self {
        let url = "mongodb://localhost:27017/".into();
        let database = "devfest".into();

        Self { url, database }
    }
}

#[derive(Serialize, Debug, Copy, Clone)]
pub struct SynchronizeResult {
    nb_categories: u32,
    nb_formats: u32,
    nb_sessions: u32,
    nb_speakers: u32,
}

impl SynchronizeResult {
    pub fn nb_categories(&self) -> u32 {
        self.nb_categories
    }
    pub fn nb_formats(&self) -> u32 {
        self.nb_formats
    }
    pub fn nb_sessions(&self) -> u32 {
        self.nb_sessions
    }
    pub fn nb_speakers(&self) -> u32 {
        self.nb_speakers
    }
}

#[derive(Clone)]
pub struct Repositories {
    user: UserRepository,

    info: MongodbRepository<SiteInfo>,

    session: SessionRepository,
    session_category: SessionCategoryRepository,
    session_format: SessionFormatRepository,

    speaker: SpeakerRepository,

    team: TeamMemberRepository,
    member_type: MemberTypeRepository,

    sponsor: SponsorRepository,
    sponsor_category: SponsorCategoryRepository,

    room: MongodbRepository<Room>,
    slot: MongodbRepository<Slot>,
    schedule: MongodbRepository<ScheduleDay>,
}

impl Repositories {
    pub async fn build(config: &MongodbConfig) -> Result<Repositories> {
        info!(
            "Connection to mongodb {} using database {}",
            config.url, config.database
        );

        let client = Client::with_uri_str(config.url.as_str()).await?;
        let db = client.database(config.database.as_str());
        let user = UserRepository::build(&db).await?;

        let info = MongodbRepository::new(&db, "info");

        let session_category = SessionCategoryRepository::new(&db);
        let session_format = SessionFormatRepository::new(&db);
        let session = SessionRepository::new(&db);

        let speaker = SpeakerRepository::new(&db);

        let team = TeamMemberRepository::new(&db);
        let member_type = MemberTypeRepository::new(&db);

        let sponsor = SponsorRepository::new(&db);
        let sponsor_category = SponsorCategoryRepository::new(&db);

        let room = MongodbRepository::new(&db, "rooms");
        let slot = MongodbRepository::new(&db, "slots");
        let schedule = MongodbRepository::new(&db, "schedule");

        // FIXME indexes

        let repositories = Repositories {
            user,
            info,
            session_category,
            session_format,
            session,
            speaker,
            team,
            member_type,
            sponsor,
            sponsor_category,
            room,
            slot,
            schedule,
        };

        Ok(repositories)
    }
    pub fn user(&self) -> UserRepository {
        self.user.clone()
    }
    pub fn info(&self) -> MongodbRepository<SiteInfo> {
        self.info.clone()
    }
    pub fn session(&self) -> SessionRepository {
        self.session.clone()
    }
    pub fn session_category(&self) -> SessionCategoryRepository {
        self.session_category.clone()
    }
    pub fn session_format(&self) -> SessionFormatRepository {
        self.session_format.clone()
    }
    pub fn speaker(&self) -> SpeakerRepository {
        self.speaker.clone()
    }
    pub fn team(&self) -> TeamMemberRepository {
        self.team.clone()
    }
    pub fn member_type(&self) -> MemberTypeRepository {
        self.member_type.clone()
    }
    pub fn sponsor(&self) -> SponsorRepository {
        self.sponsor.clone()
    }
    pub fn sponsor_category(&self) -> SponsorCategoryRepository {
        self.sponsor_category.clone()
    }
    pub fn room(&self) -> MongodbRepository<Room> {
        self.room.clone()
    }
    pub fn slot(&self) -> MongodbRepository<Slot> {
        self.slot.clone()
    }
    pub fn schedule(&self) -> MongodbRepository<ScheduleDay> {
        self.schedule.clone()
    }

    pub async fn is_allowed(&self, user: &User, operation: &Operation) -> Result<bool> {
        let allowed = match user {
            User::Guest => false,
            User::Admin { .. } => true,
            User::Team { .. } => operation.is_view(),
            User::Speaker { key, .. } => self.operation_speakers(operation).await?.contains(key),
            User::Sponsor { key, .. } => operation
                .sponsor() //
                .map(|it| &it == key) //
                .unwrap_or(false),
        };

        Ok(allowed)
    }

    async fn operation_speakers(&self, operation: &Operation) -> Result<Vec<SpeakerKey>> {
        let result = match operation {
            Operation::ViewSpeaker(k) => vec![k.clone()],
            Operation::EditSpeaker(k) => vec![k.clone()],
            Operation::ViewSession(k) => self.session.find_speakers(k).await?,
            Operation::EditSession(k) => self.session.find_speakers(k).await?,
            _ => vec![],
        };

        Ok(result)
    }

    pub async fn load_site(&self) -> Result<Site> {
        debug!("Load site info");
        let site_info = self.info.find_first().await?;

        debug!("Load site sessions");
        let sessions = self.session.find_all().await?;
        let categories = self.session_category.find().await?;
        let formats = self.session_format.find().await?;

        debug!("Load site speakers");
        let speakers = self.speaker.find_all().await?;

        debug!("Load site rooms");
        let rooms = self.room.find_all().await?;

        debug!("Load site slots");
        let slots = self.slot.find_all().await?;

        debug!("Load site schedule");
        let schedule = self.schedule.find_all().await?;

        debug!("Load site team");
        let team = self.team.find().await?;
        let member_types = self.member_type.find().await?;

        debug!("Load site sponsors");
        let sponsors = self.sponsor.find().await?;
        let sponsor_categories = self.sponsor_category.find().await?;

        // Building site
        let site = Site::new(
            site_info,
            sessions,
            speakers,
            categories,
            formats,
            rooms,
            slots,
            schedule,
            team,
            member_types,
            sponsors,
            sponsor_categories,
        );

        Ok(site)
    }

    pub async fn synchronize(&self, site: Site) -> Result<SynchronizeResult> {
        debug!("Synchronise site info");
        self.info.remove_all().await?;
        self.info.insert(&site.info()).await?;

        debug!("Synchronise site categories");
        let nb_categories = self.session_category.update_all(site.categories()).await? as u32;

        debug!("Synchronise site formats");
        let nb_formats = self.session_format.update_all(site.formats()).await? as u32;

        debug!("Synchronise site sessions");
        let nb_sessions = self
            .session
            .synchronize_sessions(site.sessions())
            .await?
            .len() as u32;

        debug!("Synchronise site speakers");
        let nb_speakers = self
            .speaker
            .synchronize_speakers(site.speakers())
            .await?
            .len() as u32;

        Ok(SynchronizeResult {
            nb_categories,
            nb_formats,
            nb_sessions,
            nb_speakers,
        })
    }
}

fn to_document<T>(element: &T) -> Result<Document>
where
    T: Serialize + Debug,
{
    let bson = bson::to_bson::<T>(element)
        .with_context(|| format!("Could not serialize the element: {:?}", element))?;
    let doc = bson.as_document().cloned().unwrap();

    Ok(doc)
}

fn from_document<'a, T>(document: Document) -> Result<T>
where
    T: Deserialize<'a>,
{
    let bson = document.clone().into();
    let element = bson::from_bson::<T>(bson)
        .with_context(|| format!("Could not deserialize document: {:?}", document))?;

    Ok(element)
}

async fn cursor_to_vec<'a, T>(cursor: &mut Cursor) -> Result<Vec<T>>
where
    T: Deserialize<'a>,
{
    let mut result = vec![];
    while let Some(doc) = cursor.next().await {
        let doc = doc.map_err(|err| anyhow!("Oops, {}", err))?;
        let element = from_document::<T>(doc)?;
        result.push(element);
    }

    Ok(result)
}
