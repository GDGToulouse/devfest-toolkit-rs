use serde::{Deserialize, Serialize};
use slug::slugify;
use uuid::Uuid;

use crate::new_id;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MemberTypeKey(String);

impl MemberTypeKey {
    pub fn new(name: &str) -> Self {
        Self(slugify(name))
    }
}

impl Into<String> for MemberTypeKey {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberType {
    id: Uuid,
    key: MemberTypeKey,
    name: String,
}

impl MemberType {
    pub fn new(id: Uuid, key: MemberTypeKey, name: String) -> Self {
        Self { id, key, name }
    }
    pub fn create(key: MemberTypeKey, name: String) -> Self {
        let id = new_id();

        Self { id, key, name }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn key(&self) -> MemberTypeKey {
        self.key.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Into<String> for MemberType {
    fn into(self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
