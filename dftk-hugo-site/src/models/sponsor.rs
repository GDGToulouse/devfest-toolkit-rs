use serde::{Deserialize, Serialize};

use dftk_common::models::language::Lang;
use dftk_common::models::socials::Social;
use dftk_common::models::sponsor::category::SponsorCategoryKey;
use dftk_common::models::sponsor::{Sponsor, SponsorKey};
use dftk_common::models::Markdown;

use crate::markdown_writer::FrontMatterMarkdown;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SponsorFrontMatter {
    key: SponsorKey,
    title: String,
    category: SponsorCategoryKey,
    order: Option<i32>,
    logo: String,
    website: Option<String>,
    lang: Lang,
    why: Option<String>,
    socials: Vec<Social>,
}

impl FrontMatterMarkdown<SponsorFrontMatter> for Sponsor {
    fn unique_key(&self) -> String {
        self.key().into()
    }

    fn front_matter(&self) -> SponsorFrontMatter {
        SponsorFrontMatter {
            key: self.key(),
            title: self.title(),
            category: self.category(),
            order: self.order(),
            logo: self.logo(),
            website: self.website(),
            lang: self.lang(),
            why: self.why(),
            socials: self.socials().to_vec(),
        }
    }

    fn content(&self) -> Markdown {
        self.description()
    }
}
