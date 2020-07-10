use anyhow::{bail, Result};
use mongodb::Database;
use uuid::Uuid;

use dftk_common::models::session::category::{CategoryKey, SessionCategory};
use dftk_common::new_id;

use crate::repository::MongodbRepository;

#[derive(Clone)]
pub struct SessionCategoryRepository {
    repo: MongodbRepository<SessionCategory>,
}

impl SessionCategoryRepository {
    pub fn new(db: &Database) -> Self {
        let repo = MongodbRepository::new(&db, "session_categories");
        Self { repo }
    }

    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<SessionCategory> {
        // check key not exists
        let key = CategoryKey::new(name.as_str());
        let sk: String = key.clone().into();
        let option = self.repo.find_by_key(sk.as_str()).await?;
        if option.is_none() {
            let description = description.map(|it| it.into());
            let element = SessionCategory::new(new_id(), key, name, description);
            self.repo.insert(&element).await?;
            Ok(element)
        } else {
            bail!("A session category already exists with key '{}'", sk)
        }
    }

    pub async fn find(&self) -> Result<Vec<SessionCategory>> {
        self.repo.find_all().await
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<SessionCategory>> {
        self.repo.find_by_key(key).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Result<SessionCategory> {
        let sid = id.to_string();
        let option = self.repo.find_by_id(sid.as_str()).await?;
        if let Some(sc) = option {
            let description = description.map(|it| it.into());
            let result = SessionCategory::new(id, sc.key(), name, description);
            self.repo
                .save_or_update(id.to_string().as_str(), &result)
                .await?;
            Ok(result)
        } else {
            bail!("No session category type has with id '{}'", id)
        }
    }

    pub async fn update_all(&self, elements: &[SessionCategory]) -> Result<usize> {
        self.repo.update_all(elements).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<Option<SessionCategory>> {
        let id = id.to_string();
        let result = self.repo.remove_by_id(id.as_str()).await?;

        Ok(result)
    }
}
