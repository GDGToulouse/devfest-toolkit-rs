use serde::{Deserialize, Serialize};

use dftk_common::models::socials::Social;
use dftk_common::models::speaker::{Speaker, SpeakerId, SpeakerKey};
use dftk_common::models::Markdown;

use crate::markdown_writer::FrontMatterMarkdown;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpeakerFrontMatter {
    id: SpeakerId,
    key: SpeakerKey,
    featured: bool,
    name: String,
    company: Option<String>,
    city: Option<String>,
    photo_url: Option<String>,
    socials: Vec<Social>,
    draft: Option<bool>,
}

impl FrontMatterMarkdown<SpeakerFrontMatter> for Speaker {
    fn unique_key(&self) -> String {
        self.key().into()
    }

    fn front_matter(&self) -> SpeakerFrontMatter {
        SpeakerFrontMatter {
            key: self.key(),
            id: self.id(),
            featured: self.featured(),
            name: self.name(),
            company: self.company(),
            city: self.city(),
            photo_url: self.photo_url(),
            socials: self.socials().to_vec(),
            draft: self.draft(),
        }
    }

    fn content(&self) -> Markdown {
        self.content()
    }
}
