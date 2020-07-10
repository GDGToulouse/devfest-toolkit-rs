use serde::{Deserialize, Serialize};
use slug::slugify;
use uuid::Uuid;

use crate::models::Markdown;
use crate::new_id;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct FormatKey(String);

impl FormatKey {
    pub fn new(name: &str) -> Self {
        Self(slugify(name))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionFormat {
    _id: Uuid,
    key: FormatKey,
    name: String,
    description: Option<Markdown>,
}

impl Into<String> for FormatKey {
    fn into(self) -> String {
        self.0
    }
}

impl Default for SessionFormat {
    fn default() -> Self {
        let id = new_id();
        let name = "<Unknown>";
        let key = FormatKey::new(name);
        let name = name.into();
        let description = None;

        SessionFormat {
            _id: id,
            key,
            name,
            description,
        }
    }
}

impl SessionFormat {
    pub fn new(id: Uuid, key: FormatKey, name: String, description: Option<Markdown>) -> Self {
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
    pub fn key(&self) -> FormatKey {
        self.key.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> Option<Markdown> {
        self.description.clone()
    }
}
