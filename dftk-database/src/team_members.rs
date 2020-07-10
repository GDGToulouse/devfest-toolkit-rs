use anyhow::{bail, Result};
use mongodb::Database;
use uuid::Uuid;

use dftk_common::models::team::{PartialTeamMember, TeamMember};

use crate::repository::MongodbRepository;

#[derive(Clone)]
pub struct TeamMemberRepository {
    repo: MongodbRepository<TeamMember>,
}

impl TeamMemberRepository {
    pub fn new(db: &Database) -> Self {
        let repo = MongodbRepository::new(&db, "team_member_types");
        Self { repo }
    }

    pub async fn create(&self, element: PartialTeamMember) -> Result<TeamMember> {
        let element: TeamMember = element.into();
        // check key not exists
        let key: String = element.key().into();
        let option = self.repo.find_by_key(key.as_str()).await?;
        if option.is_none() {
            self.repo.insert(&element).await?;
            Ok(element)
        } else {
            bail!("A team member already exists with key '{}'", key)
        }
    }

    pub async fn find(&self) -> Result<Vec<TeamMember>> {
        self.repo.find_all().await
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<TeamMember>> {
        self.repo.find_by_key(key).await
    }

    pub async fn update(&self, id: Uuid, element: PartialTeamMember) -> Result<TeamMember> {
        let sid = id.to_string();
        let option = self.repo.find_by_id(sid.as_str()).await?;
        if let Some(mt) = option {
            let result = TeamMember::new(
                id,
                mt.key(),
                element.member_type(),
                element.title(),
                element.subtitle(),
                element.photo(),
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

    pub async fn delete(&self, id: Uuid) -> Result<Option<TeamMember>> {
        let id = id.to_string();
        let result = self.repo.remove_by_id(id.as_str()).await?;

        Ok(result)
    }
}
