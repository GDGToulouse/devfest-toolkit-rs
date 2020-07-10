use async_graphql::SimpleObject;

use dftk_common::models::session::format::{FormatKey, SessionFormat};

#[SimpleObject]
pub struct FormatOutputType {
    key: FormatKey,
    name: String,
    description: Option<String>,
}

impl From<SessionFormat> for FormatOutputType {
    fn from(value: SessionFormat) -> Self {
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
