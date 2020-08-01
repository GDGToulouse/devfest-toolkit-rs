use anyhow::{bail, Result};
use mongodb::Database;
use uuid::Uuid;

use dftk_common::models::sponsor::{PartialSponsor, Sponsor, SponsorKey};

use crate::repository::MongodbRepository;

#[derive(Clone)]
pub struct SponsorRepository {
    repo: MongodbRepository<Sponsor>,
}

impl SponsorRepository {
    pub fn new(db: &Database) -> Self {
        let repo = MongodbRepository::new(&db, "sponsors");
        Self { repo }
    }

    pub async fn create(&self, element: PartialSponsor) -> Result<Sponsor> {
        let element: Sponsor = element.into();
        // check key not exists
        let key: String = element.key().into();
        let option = self.repo.find_by_key(key.as_str()).await?;
        if option.is_none() {
            self.repo.insert(&element).await?;
            Ok(element)
        } else {
            bail!("A sponsor already exists with key '{}'", key)
        }
    }

    pub async fn find(&self) -> Result<Vec<Sponsor>> {
        self.repo.find_all().await
    }

    pub async fn find_by_key(&self, key: SponsorKey) -> Result<Option<Sponsor>> {
        let k: String = key.into();
        self.repo.find_by_key(k.as_str()).await
    }

    pub async fn update(&self, id: Uuid, element: PartialSponsor) -> Result<Sponsor> {
        let sid = id.to_string();
        let option = self.repo.find_by_id(sid.as_str()).await?;
        if let Some(mt) = option {
            let result = Sponsor::new(
                id,
                mt.key(),
                element.title(),
                element.category(),
                element.order(),
                element.logo(),
                element.website(),
                element.lang(),
                element.why(),
                element.socials().to_vec(),
                element.description(),
            );
            self.repo
                .save_or_update(id.to_string().as_str(), &result)
                .await?;
            Ok(result)
        } else {
            bail!("No team member has id '{}'", id)
        }
    }

    pub async fn delete(&self, id: Uuid) -> Result<Option<Sponsor>> {
        let id = id.to_string();
        let result = self.repo.remove_by_id(id.as_str()).await?;

        Ok(result)
    }
}
