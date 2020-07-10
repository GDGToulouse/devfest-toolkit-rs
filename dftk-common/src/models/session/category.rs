use serde::{Deserialize, Serialize};
use slug::slugify;
use uuid::Uuid;

use crate::models::Markdown;
use crate::new_id;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct CategoryKey(String);

impl CategoryKey {
    pub fn new(name: &str) -> Self {
        Self(slugify(name))
    }
}

impl Into<String> for CategoryKey {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionCategory {
    _id: Uuid,
    key: CategoryKey,
    name: String,
    description: Option<Markdown>,
}

impl SessionCategory {
    pub fn new(id: Uuid, key: CategoryKey, name: String, description: Option<Markdown>) -> Self {
        Self {
            _id: id,
            key,
            name,
            description,
        }
    }

    pub fn id(&self) -> Uuid {
        self._id
    }
    pub fn key(&self) -> CategoryKey {
        self.key.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<Markdown> {
        self.description.clone()
    }
}

impl Default for SessionCategory {
    fn default() -> Self {
        let id = new_id();
        let name = "<Unknown>";
        let key = CategoryKey::new(name);
        let name = name.into();
        let description = None;

        SessionCategory {
            _id: id,
            key,
            name,
            description,
        }
    }
}
