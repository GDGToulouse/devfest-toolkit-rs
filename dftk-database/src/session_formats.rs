use anyhow::{bail, Result};
use mongodb::Database;
use uuid::Uuid;

use dftk_common::models::session::format::{FormatKey, SessionFormat};
use dftk_common::new_id;

use crate::repository::MongodbRepository;

#[derive(Clone)]
pub struct SessionFormatRepository {
    repo: MongodbRepository<SessionFormat>,
}

impl SessionFormatRepository {
    pub fn new(db: &Database) -> Self {
        let repo = MongodbRepository::new(&db, "session_formats");
        Self { repo }
    }

    pub async fn create(&self, name: String, description: Option<String>) -> Result<SessionFormat> {
        // check key not exists
        let key = FormatKey::new(name.as_str());
        let sk: String = key.clone().into();
        let option = self.repo.find_by_key(sk.as_str()).await?;
        if option.is_none() {
            let description = description.map(|it| it.into());
            let element = SessionFormat::new(new_id(), key, name, description);
            self.repo.insert(&element).await?;
            Ok(element)
        } else {
            bail!("A session format already exists with key '{}'", sk)
        }
    }

    pub async fn find(&self) -> Result<Vec<SessionFormat>> {
        self.repo.find_all().await
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<SessionFormat>> {
        self.repo.find_by_key(key).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Result<SessionFormat> {
        let sid = id.to_string();
        let option = self.repo.find_by_id(sid.as_str()).await?;
        if let Some(sc) = option {
            let description = description.map(|it| it.into());
            let result = SessionFormat::new(id, sc.key(), name, description);
            self.repo
                .save_or_update(id.to_string().as_str(), &result)
                .await?;
            Ok(result)
        } else {
            bail!("No session format has with id '{}'", id)
        }
    }

    pub async fn update_all(&self, elements: &[SessionFormat]) -> Result<usize> {
        self.repo.update_all(elements).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<Option<SessionFormat>> {
        let id = id.to_string();
        let result = self.repo.remove_by_id(id.as_str()).await?;

        Ok(result)
    }
}
