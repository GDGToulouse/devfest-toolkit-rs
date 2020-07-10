use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

use dftk_common::models::language::Lang;
use dftk_common::models::sponsor::category::{SponsorCategory, SponsorCategoryKey};
use dftk_common::models::sponsor::{PartialSponsor, Sponsor, SponsorKey};

use crate::graphql::socials::{SocialInputType, SocialOutputType};

#[SimpleObject]
pub struct SponsorOutputType {
    key: SponsorKey,
    title: String,
    category: String,
    order: Option<i32>,
    logo: String,
    website: Option<String>,
    lang: Lang,
    why: Option<String>,
    socials: SocialOutputType,
    description: String,
}

impl From<Sponsor> for SponsorOutputType {
    fn from(sponsor: Sponsor) -> Self {
        Self {
            key: sponsor.key(),
            title: sponsor.title(),
            category: sponsor.category().into(),
            order: sponsor.order(),
            logo: sponsor.logo(),
            website: sponsor.website(),
            lang: sponsor.lang(),
            why: sponsor.why(),
            socials: SocialOutputType::new(sponsor.socials()),
            description: sponsor.description().into(),
        }
    }
}

#[SimpleObject]
pub struct SponsorCategoryOutputType {
    id: Uuid,
    key: SponsorCategoryKey,
    name: String,
}

impl From<SponsorCategory> for SponsorCategoryOutputType {
    fn from(sc: SponsorCategory) -> Self {
        Self {
            id: sc.id(),
            key: sc.key(),
            name: sc.name(),
        }
    }
}

#[InputObject]
pub struct SponsorInputType {
    title: String,
    category: SponsorCategoryKey,
    order: Option<i32>,
    logo: String,
    website: Option<String>,
    lang: Lang,
    why: Option<String>,
    socials: SocialInputType,
    description: String,
}

impl Into<PartialSponsor> for SponsorInputType {
    fn into(self) -> PartialSponsor {
        PartialSponsor::new(
            self.title.clone(),
            self.category.clone(),
            self.order,
            self.logo.clone(),
            self.website.clone(),
            self.lang.clone(),
            self.why.clone(),
            self.socials.into(),
            self.description.into(),
        )
    }
}
