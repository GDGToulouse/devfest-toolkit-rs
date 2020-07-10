use async_graphql::SimpleObject;

use dftk_common::models::session::category::{CategoryKey, SessionCategory};

#[SimpleObject]
pub struct CategoryOutputType {
    key: CategoryKey,
    name: String,
    description: Option<String>,
}

impl From<SessionCategory> for CategoryOutputType {
    fn from(value: SessionCategory) -> Self {
        let key = value.key();
        let name = value.name();
        let description = value.description().map(|it| it.into());

        Self {
            key,
            name,
            description,
        }
    }
}
