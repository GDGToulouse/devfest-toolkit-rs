use async_graphql::{Context, FieldResult, Object};

use dftk_common::models::session::SessionId;
use dftk_common::models::speaker::SpeakerId;
use dftk_database::Repositories;

use crate::graphql::categories::CategoryOutputType;
use crate::graphql::formats::FormatOutputType;
use crate::graphql::info::SiteInfoOutputType;
use crate::graphql::schedule::ScheduleOutputType;
use crate::graphql::sessions::{SessionDocumentOutputType, SessionOutputType};
use crate::graphql::speakers::{SpeakerDocumentOutputType, SpeakerOutputType};
use crate::graphql::sponsors::{SponsorCategoryOutputType, SponsorOutputType};
use crate::graphql::teams::{MemberTypeOutputType, TeamMemberOutputType};

pub struct QuerySite;

#[Object]
impl QuerySite {
    /// Getting site general info
    async fn info(&self, ctx: &Context<'_>) -> FieldResult<SiteInfoOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let info = repos.info().find_first().await?;
        let info = info.into();

        Ok(info)
    }

    /// Getting session categories
    async fn categories(&self, ctx: &Context<'_>) -> FieldResult<Vec<CategoryOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let categories = repos.session_category().find().await?;
        let categories = categories.iter().map(|it| it.clone().into()).collect();

        Ok(categories)
    }

    /// Getting session formats
    async fn formats(&self, ctx: &Context<'_>) -> FieldResult<Vec<FormatOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let formats = repos.session_format().find().await?;
        let formats = formats.iter().map(|it| it.clone().into()).collect();

        Ok(formats)
    }

    /// Getting a detail session information
    async fn session_by_id(
        &self,
        ctx: &Context<'_>,
        id: SessionId,
    ) -> FieldResult<Option<SessionDocumentOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let session = repos.session().find_by_id(id).await?;
        let session: Option<SessionDocumentOutputType> = session.map(|it| it.into());

        Ok(session)
    }

    /// Getting all sessions
    async fn sessions(&self, ctx: &Context<'_>) -> FieldResult<Vec<SessionOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let sessions = repos.session().find_all().await?;
        let sessions = sessions
            .iter()
            .map(move |it| it.into())
            .collect::<Vec<SessionOutputType>>();

        Ok(sessions)
    }

    /// Getting a speaker detail
    async fn speaker_by_id(
        &self,
        ctx: &Context<'_>,
        id: SpeakerId,
    ) -> FieldResult<Option<SpeakerDocumentOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let speaker = repos.speaker().find_by_id(id).await?;
        let speaker: Option<SpeakerDocumentOutputType> = speaker.map(|it| it.into());

        Ok(speaker)
    }

    /// Getting all speakers
    async fn speakers(&self, ctx: &Context<'_>) -> FieldResult<Vec<SpeakerOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let speakers = repos.speaker().find_all().await?;
        let speakers = speakers.iter().map(|it| it.into()).collect();

        Ok(speakers)
    }

    /// Getting schedule
    async fn schedule(&self) -> FieldResult<ScheduleOutputType> {
        Ok(ScheduleOutputType)
    }

    /// Getting all team speaker
    async fn team(&self, ctx: &Context<'_>) -> FieldResult<Vec<TeamMemberOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let team = repos.team().find().await?;
        let team = team.iter().map(move |it| it.clone().into()).collect();

        Ok(team)
    }

    /// Getting team member types
    async fn member_types(&self, ctx: &Context<'_>) -> FieldResult<Vec<MemberTypeOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let member_types = repos.member_type().find().await?;
        let member_types = member_types.iter().map(|it| it.clone().into()).collect();

        Ok(member_types)
    }

    /// Getting all sponsors
    async fn sponsors(&self, ctx: &Context<'_>) -> FieldResult<Vec<SponsorOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let sponsors = repos.sponsor().find().await?;
        let sponsors = sponsors.iter().map(move |it| it.clone().into()).collect();

        Ok(sponsors)
    }

    /// Getting sponsor categories
    async fn sponsor_categories(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<SponsorCategoryOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let sponsor_categories = repos.sponsor_category().find().await?;
        let sponsor_categories = sponsor_categories
            .iter()
            .map(|it| it.clone().into())
            .collect();

        Ok(sponsor_categories)
    }
}
