use anyhow::{bail, Result};
use mongodb::Database;
use uuid::Uuid;

use dftk_common::models::sponsor::category::{SponsorCategory, SponsorCategoryKey};
use dftk_common::new_id;

use crate::repository::MongodbRepository;

#[derive(Clone)]
pub struct SponsorCategoryRepository {
    repo: MongodbRepository<SponsorCategory>,
}

impl SponsorCategoryRepository {
    pub fn new(db: &Database) -> Self {
        let repo = MongodbRepository::new(&db, "sponsor_categories");
        Self { repo }
    }

    pub async fn create(&self, name: String) -> Result<SponsorCategory> {
        // check key not exists
        let key = SponsorCategoryKey::new(name.as_str());
        let sk: String = key.clone().into();
        let option = self.repo.find_by_key(sk.as_str()).await?;
        if option.is_none() {
            let element = SponsorCategory::new(new_id(), key, name);
            self.repo.insert(&element).await?;
            Ok(element)
        } else {
            bail!("A sponsor category already exists with key '{}'", sk)
        }
    }

    pub async fn find(&self) -> Result<Vec<SponsorCategory>> {
        self.repo.find_all().await
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<SponsorCategory>> {
        self.repo.find_by_key(key).await
    }

    pub async fn update(&self, id: Uuid, name: String) -> Result<SponsorCategory> {
        let sid = id.to_string();
        let option = self.repo.find_by_id(sid.as_str()).await?;
        if let Some(mt) = option {
            let result = SponsorCategory::new(id, mt.key(), name);
            self.repo
                .save_or_update(id.to_string().as_str(), &result)
                .await?;
            Ok(result)
        } else {
            bail!("No member type has with id '{}'", id)
        }
    }

    pub async fn delete(&self, id: Uuid) -> Result<Option<SponsorCategory>> {
        let id = id.to_string();
        let result = self.repo.remove_by_id(id.as_str()).await?;

        Ok(result)
    }
}
