use std::str::FromStr;

use serde::{Deserialize, Serialize};
use slug::slugify;
use uuid::Uuid;

use crate::models::language::Lang;
use crate::models::socials::Social;
use crate::models::sponsor::category::SponsorCategoryKey;
use crate::models::Markdown;
use crate::new_id;

pub mod category;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SponsorKey(String);

impl SponsorKey {
    pub fn new(title: &str) -> Self {
        Self(slugify(title))
    }
}

impl Into<String> for SponsorKey {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for SponsorKey {
    fn from(s: String) -> Self {
        SponsorKey(s)
    }
}

impl FromStr for SponsorKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = SponsorKey(s.into());

        Ok(key)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sponsor {
    _id: Uuid,
    key: SponsorKey,
    title: String,
    category: SponsorCategoryKey,
    order: Option<i32>,
    logo: String,
    website: Option<String>,
    lang: Lang,
    why: Option<String>,
    socials: Vec<Social>,
    description: Markdown,
}

impl Sponsor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        key: SponsorKey,
        title: String,
        category: SponsorCategoryKey,
        order: Option<i32>,
        logo: String,
        website: Option<String>,
        lang: Lang,
        why: Option<String>,
        socials: Vec<Social>,
        description: Markdown,
    ) -> Self {
        Self {
            _id: id,
            key,
            title,
            category,
            order,
            logo,
            website,
            lang,
            why,
            socials,
            description,
        }
    }

    pub fn id(&self) -> Uuid {
        self._id
    }
    pub fn key(&self) -> SponsorKey {
        self.key.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn category(&self) -> SponsorCategoryKey {
        self.category.clone()
    }
    pub fn order(&self) -> Option<i32> {
        self.order
    }
    pub fn logo(&self) -> String {
        self.logo.clone()
    }
    pub fn website(&self) -> Option<String> {
        self.website.clone()
    }
    pub fn lang(&self) -> Lang {
        self.lang.clone()
    }
    pub fn why(&self) -> Option<String> {
        self.why.clone()
    }
    pub fn socials(&self) -> &[Social] {
        self.socials.as_slice()
    }

    pub fn description(&self) -> Markdown {
        self.description.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialSponsor {
    title: String,
    category: SponsorCategoryKey,
    order: Option<i32>,
    logo: String,
    website: Option<String>,
    lang: Lang,
    why: Option<String>,
    socials: Vec<Social>,
    description: Markdown,
}

impl PartialSponsor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: String,
        category: SponsorCategoryKey,
        order: Option<i32>,
        logo: String,
        website: Option<String>,
        lang: Lang,
        why: Option<String>,
        socials: Vec<Social>,
        description: Markdown,
    ) -> Self {
        Self {
            title,
            category,
            order,
            logo,
            website,
            lang,
            why,
            socials,
            description,
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn category(&self) -> SponsorCategoryKey {
        self.category.clone()
    }
    pub fn order(&self) -> Option<i32> {
        self.order
    }
    pub fn logo(&self) -> String {
        self.logo.clone()
    }
    pub fn website(&self) -> Option<String> {
        self.website.clone()
    }
    pub fn lang(&self) -> Lang {
        self.lang.clone()
    }
    pub fn why(&self) -> Option<String> {
        self.why.clone()
    }
    pub fn socials(&self) -> &[Social] {
        self.socials.as_slice()
    }

    pub fn description(&self) -> Markdown {
        self.description.clone()
    }
}

impl Into<Sponsor> for PartialSponsor {
    fn into(self) -> Sponsor {
        let id = new_id();
        let key = SponsorKey::new(self.title.as_str());

        Sponsor::new(
            id,
            key,
            self.title(),
            self.category(),
            self.order,
            self.logo(),
            self.website(),
            self.lang(),
            self.why(),
            self.socials().to_vec(),
            self.description(),
        )
    }
}
