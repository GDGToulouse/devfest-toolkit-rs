use serde::{Deserialize, Serialize};

use crate::markdown_writer::FrontMatterMarkdown;
use dftk_common::models::socials::Social;
use dftk_common::models::team::member_type::MemberTypeKey;
use dftk_common::models::team::{TeamMember, TeamMemberKey};
use dftk_common::models::Markdown;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamMemberFrontMatter {
    key: TeamMemberKey,
    member_type: MemberTypeKey,
    title: String,
    subtitle: Option<String>,
    photo: String,
    socials: Vec<Social>,
}

impl FrontMatterMarkdown<TeamMemberFrontMatter> for TeamMember {
    fn unique_key(&self) -> String {
        self.key().into()
    }

    fn front_matter(&self) -> TeamMemberFrontMatter {
        TeamMemberFrontMatter {
            key: self.key(),
            member_type: self.member_type(),
            title: self.title(),
            subtitle: self.subtitle(),
            photo: self.photo(),
            socials: self.socials().to_vec(),
        }
    }

    fn content(&self) -> Markdown {
        self.description()
    }
}
