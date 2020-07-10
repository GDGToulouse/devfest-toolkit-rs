use async_graphql::SimpleObject;

use dftk_common::models::language::{Lang, Languages};

#[SimpleObject]
pub struct LanguagesOutputType {
    main: Lang,
    others: Vec<Lang>,
}

impl From<Languages> for LanguagesOutputType {
    fn from(languages: Languages) -> Self {
        let main = languages.main();
        let others = languages.others().to_vec();

        Self { main, others }
    }
}
