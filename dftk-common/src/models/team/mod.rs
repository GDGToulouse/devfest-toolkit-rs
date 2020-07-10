use serde::{Deserialize, Serialize};
use slug::slugify;
use uuid::Uuid;

use crate::models::socials::Social;
use crate::models::team::member_type::MemberTypeKey;
use crate::models::Markdown;
use crate::new_id;

pub mod member_type;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMemberKey(String);

impl TeamMemberKey {
    pub fn new(title: &str) -> Self {
        Self(slugify(title))
    }
}

impl Into<String> for TeamMemberKey {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for TeamMemberKey {
    fn from(s: String) -> Self {
        TeamMemberKey(s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMember {
    id: Uuid,
    key: TeamMemberKey,
    member_type: MemberTypeKey,
    title: String,
    subtitle: Option<String>,
    photo: String,
    socials: Vec<Social>,
    description: Markdown,
}

impl TeamMember {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        key: TeamMemberKey,
        member_type: MemberTypeKey,
        title: String,
        subtitle: Option<String>,
        photo: String,
        socials: Vec<Social>,
        description: Markdown,
    ) -> Self {
        Self {
            id,
            key,
            member_type,
            title,
            subtitle,
            photo,
            socials,
            description,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn key(&self) -> TeamMemberKey {
        self.key.clone()
    }
    pub fn member_type(&self) -> MemberTypeKey {
        self.member_type.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn subtitle(&self) -> Option<String> {
        self.subtitle.clone()
    }
    pub fn photo(&self) -> String {
        self.photo.clone()
    }
    pub fn socials(&self) -> &[Social] {
        self.socials.as_slice()
    }
    pub fn description(&self) -> Markdown {
        self.description.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialTeamMember {
    member_type: MemberTypeKey,
    title: String,
    subtitle: Option<String>,
    photo: String,
    socials: Vec<Social>,
    description: Markdown,
}

impl PartialTeamMember {
    pub fn new(
        member_type: MemberTypeKey,
        title: String,
        subtitle: Option<String>,
        photo: String,
        socials: Vec<Social>,
        description: Markdown,
    ) -> Self {
        Self {
            member_type,
            title,
            subtitle,
            photo,
            socials,
            description,
        }
    }

    pub fn member_type(&self) -> MemberTypeKey {
        self.member_type.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn subtitle(&self) -> Option<String> {
        self.subtitle.clone()
    }
    pub fn photo(&self) -> String {
        self.photo.clone()
    }
    pub fn socials(&self) -> &[Social] {
        self.socials.as_slice()
    }
    pub fn description(&self) -> Markdown {
        self.description.clone()
    }
}

impl Into<TeamMember> for PartialTeamMember {
    fn into(self) -> TeamMember {
        let id = new_id();
        let key = TeamMemberKey::new(self.title.as_str());

        TeamMember::new(
            id,
            key,
            self.member_type(),
            self.title(),
            self.subtitle(),
            self.photo(),
            self.socials().to_vec(),
            self.description(),
        )
    }
}
