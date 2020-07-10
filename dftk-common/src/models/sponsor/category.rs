use serde::{Deserialize, Serialize};
use slug::slugify;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SponsorCategoryKey(String);

impl SponsorCategoryKey {
    pub fn new(name: &str) -> Self {
        Self(slugify(name))
    }
}

impl Into<String> for SponsorCategoryKey {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SponsorCategory {
    _id: Uuid,
    key: SponsorCategoryKey,
    name: String,
}

impl SponsorCategory {
    pub fn new(id: Uuid, key: SponsorCategoryKey, name: String) -> Self {
        Self { _id: id, key, name }
    }

    pub fn id(&self) -> Uuid {
        self._id
    }

    pub fn key(&self) -> SponsorCategoryKey {
        self.key.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Into<String> for SponsorCategory {
    fn into(self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
