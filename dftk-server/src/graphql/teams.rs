use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

use dftk_common::models::team::member_type::{MemberType, MemberTypeKey};
use dftk_common::models::team::{PartialTeamMember, TeamMember, TeamMemberKey};

use crate::graphql::socials::{SocialInputType, SocialOutputType};

#[SimpleObject]
pub struct MemberTypeOutputType {
    id: Uuid,
    key: MemberTypeKey,
    name: String,
}

impl From<MemberType> for MemberTypeOutputType {
    fn from(mt: MemberType) -> Self {
        Self {
            id: mt.id(),
            key: mt.key(),
            name: mt.name(),
        }
    }
}

#[SimpleObject]
pub struct TeamMemberOutputType {
    key: TeamMemberKey,
    member_type: String,
    title: String,
    subtitle: Option<String>,
    photo: String,
    socials: SocialOutputType,
    description: String,
}

impl From<TeamMember> for TeamMemberOutputType {
    fn from(team: TeamMember) -> Self {
        Self {
            key: team.key(),
            member_type: team.member_type().into(),
            title: team.title(),
            subtitle: team.subtitle(),
            photo: team.photo(),
            socials: SocialOutputType::new(team.socials()),
            description: team.description().into(),
        }
    }
}

#[InputObject]
pub struct TeamMemberInputType {
    member_type: MemberTypeKey,
    title: String,
    subtitle: Option<String>,
    photo: String,
    socials: SocialInputType,
    description: String,
}

impl Into<PartialTeamMember> for TeamMemberInputType {
    fn into(self) -> PartialTeamMember {
        PartialTeamMember::new(
            self.member_type.clone(),
            self.title.clone(),
            self.subtitle.clone(),
            self.photo.clone(),
            self.socials.into(),
            self.description.into(),
        )
    }
}

#[InputObject]
pub struct MemberTypeCreateInputType {
    key: MemberTypeKey,
    name: String,
}

impl Into<MemberType> for MemberTypeCreateInputType {
    fn into(self) -> MemberType {
        MemberType::create(self.key, self.name)
    }
}
