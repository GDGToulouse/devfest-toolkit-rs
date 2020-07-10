#[cfg(feature = "graphql")]
use async_graphql::{Context, FieldResult, Object};
use uuid::Uuid;

use dftk_common::models::session::SessionId;
use dftk_common::models::speaker::SpeakerId;
use dftk_conference_hall::{read_event, ConferenceHallConfig};
use dftk_database::Repositories;
use dftk_hugo_site::{generate, SiteConfig};

use crate::graphql::sessions::{
    GenerateResultOutputType, SessionCategoryOutputType, SessionCreateInput,
    SessionDocumentOutputType, SessionFormatOutputType, SessionPatchInput,
    SynchronizeResultOutputType,
};
use crate::graphql::speakers::{SpeakerCreateInput, SpeakerDocumentOutputType, SpeakerPatchInput};
use crate::graphql::sponsors::{SponsorCategoryOutputType, SponsorInputType, SponsorOutputType};
use crate::graphql::teams::{MemberTypeOutputType, TeamMemberInputType, TeamMemberOutputType};
use crate::graphql::user::{to_user, UserCreateInput, UserCreateOutput};

pub struct MutationSite;

#[Object]
impl MutationSite {
    /// Insert a new user
    async fn new_user(
        &self,
        ctx: &Context<'_>,
        user: UserCreateInput,
    ) -> FieldResult<UserCreateOutput> {
        let repos = ctx.data_unchecked::<Repositories>();
        let user = to_user(&user)?;
        // FIXME check speaker / sponsor Key
        let password = repos.user().new_user(user).await?;
        let result = UserCreateOutput::new(password);

        Ok(result)
    }

    /// Fetch site info, talks and speakers from Conference Hall and update the database
    async fn synchronize(&self, ctx: &Context<'_>) -> FieldResult<SynchronizeResultOutputType> {
        let ch_config = ctx.data_unchecked::<ConferenceHallConfig>();
        let repos = ctx.data_unchecked::<Repositories>();
        let site = read_event(ch_config).await?;
        let result = repos.synchronize(site).await?;

        Ok(result.into())
    }

    /// Generate Hugo Site files (speaker, sessions, team, sponsors, schedule)
    async fn generate(&self, ctx: &Context<'_>) -> FieldResult<GenerateResultOutputType> {
        let site_config = ctx.data_unchecked::<SiteConfig>();
        let repos = ctx.data_unchecked::<Repositories>();
        let site = repos.load_site().await?;
        let result = generate(&site_config, site).await?;

        Ok(result.into())
    }

    /// Patching a session
    async fn patch_session(
        &self,
        ctx: &Context<'_>,
        id: SessionId,
        patch: SessionPatchInput,
    ) -> FieldResult<SessionDocumentOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let patch = patch.to_session_patch()?;
        // FIXME check speaker key / category key / level key
        let result = repos.session().update_session(id, patch).await?;

        Ok(result.into())
    }

    /// Create a new session
    async fn create_session(
        &self,
        ctx: &Context<'_>,
        patch: SessionCreateInput,
    ) -> FieldResult<SessionDocumentOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        // FIXME check speaker key / category key / level key
        let result = repos.session().insert_session(patch.into()).await?;

        Ok(result.into())
    }

    /// Delete a session
    async fn delete_session(
        &self,
        ctx: &Context<'_>,
        id: SessionId,
    ) -> FieldResult<Option<SessionDocumentOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.session().delete_session(id).await?;
        let result: Option<SessionDocumentOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    /// Create a session category
    async fn create_session_category(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
    ) -> FieldResult<SessionCategoryOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.session_category().create(name, description).await?;

        Ok(result.into())
    }

    /// Update a session category
    async fn update_session_category(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: String,
        description: Option<String>,
    ) -> FieldResult<SessionCategoryOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos
            .session_category()
            .update(id, name, description)
            .await?;

        Ok(result.into())
    }

    /// Delete a session category
    async fn delete_session_category(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Option<SessionCategoryOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.session_category().delete(id).await?;
        let result: Option<SessionCategoryOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    /// Create a session format
    async fn create_session_format(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: Option<String>,
    ) -> FieldResult<SessionFormatOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.session_format().create(name, description).await?;

        Ok(result.into())
    }

    /// Update a session format
    async fn update_session_format(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: String,
        description: Option<String>,
    ) -> FieldResult<SessionFormatOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.session_format().update(id, name, description).await?;

        Ok(result.into())
    }

    /// Delete a session format
    async fn delete_session_format(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Option<SessionFormatOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.session_format().delete(id).await?;
        let result: Option<SessionFormatOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    /// Patching a speaker
    async fn patch_speaker(
        &self,
        ctx: &Context<'_>,
        id: SpeakerId,
        patch: SpeakerPatchInput,
    ) -> FieldResult<SpeakerDocumentOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let patch = patch.to_speaker_patch()?;
        let result = repos.speaker().update_speaker(id, patch).await?;

        Ok(result.into())
    }

    /// Create a new speaker
    async fn create_speaker(
        &self,
        ctx: &Context<'_>,
        input: SpeakerCreateInput,
    ) -> FieldResult<SpeakerDocumentOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.speaker().insert_speaker(input.into()).await?;

        Ok(result.into())
    }

    /// Delete a speaker
    async fn delete_speaker(
        &self,
        ctx: &Context<'_>,
        id: SpeakerId,
    ) -> FieldResult<Option<SpeakerDocumentOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.speaker().delete_speaker(id).await?;
        let result: Option<SpeakerDocumentOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    /// Create a team member type
    async fn create_member_type(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> FieldResult<MemberTypeOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.member_type().create(name).await?;

        Ok(result.into())
    }

    /// Update a member type
    async fn update_member_type(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: String,
    ) -> FieldResult<MemberTypeOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.member_type().update(id, name).await?;

        Ok(result.into())
    }

    /// Delete a member type
    async fn delete_member_type(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Option<MemberTypeOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.member_type().delete(id).await?;
        let result: Option<MemberTypeOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    /// Create a team member
    async fn create_team_member(
        &self,
        ctx: &Context<'_>,
        input: TeamMemberInputType,
    ) -> FieldResult<TeamMemberOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.team().create(input.into()).await?;

        Ok(result.into())
    }

    /// Update a team member
    async fn update_team_member(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: TeamMemberInputType,
    ) -> FieldResult<TeamMemberOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.team().update(id, input.into()).await?;

        Ok(result.into())
    }

    /// Delete a team member
    async fn delete_team_member(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Option<TeamMemberOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.team().delete(id).await?;
        let result: Option<TeamMemberOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    /// Create a sponsor category
    async fn create_sponsor_category(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> FieldResult<SponsorCategoryOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.sponsor_category().create(name).await?;

        Ok(result.into())
    }

    /// Update a sponsor category
    async fn update_sponsor_category(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: String,
    ) -> FieldResult<SponsorCategoryOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.sponsor_category().update(id, name).await?;

        Ok(result.into())
    }

    /// Delete a sponsor category
    async fn delete_sponsor_category(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Option<SponsorCategoryOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.sponsor_category().delete(id).await?;
        let result: Option<SponsorCategoryOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    /// Create a sponsor
    async fn create_sponsor(
        &self,
        ctx: &Context<'_>,
        input: SponsorInputType,
    ) -> FieldResult<SponsorOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.sponsor().create(input.into()).await?;

        Ok(result.into())
    }

    /// Update a sponsor
    async fn update_sponsor(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: SponsorInputType,
    ) -> FieldResult<SponsorOutputType> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.sponsor().update(id, input.into()).await?;

        Ok(result.into())
    }

    /// Delete a sponsor
    async fn delete_sponsor(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> FieldResult<Option<SponsorOutputType>> {
        let repos = ctx.data_unchecked::<Repositories>();
        let result = repos.sponsor().delete(id).await?;
        let result: Option<SponsorOutputType> = result.map(|it| it.into());

        Ok(result)
    }

    // FIXME schedule
}
