use std::str::FromStr;

use anyhow::{anyhow, Result};
use async_graphql::{Enum, InputObject, SimpleObject};

use dftk_common::acl::user::{Email, User};
use dftk_common::models::speaker::SpeakerKey;
use dftk_common::models::sponsor::SponsorKey;

#[Enum]
pub enum UserKind {
    Admin,
    Team,
    Speaker,
    Sponsor,
}

#[SimpleObject]
pub struct UserCreateOutput {
    new_password: String,
}

impl UserCreateOutput {
    pub fn new(new_password: String) -> Self {
        Self { new_password }
    }
}

#[InputObject]
pub struct UserCreateInput {
    kind: UserKind,
    email: String,
    key: Option<String>,
}

pub fn to_user(user: &UserCreateInput) -> Result<User> {
    let UserCreateInput { kind, email, key } = user;
    let email = Email::from_str(email.as_str())?;
    let user = match kind {
        UserKind::Admin => User::Admin { email },
        UserKind::Team => User::Team { email },
        UserKind::Speaker => {
            let key = key
                .clone()
                .ok_or_else(|| anyhow!("Expected a the speaker key"))?;
            let key = SpeakerKey::new(key.as_str());
            User::Speaker { email, key }
        }
        UserKind::Sponsor => {
            let key = key
                .clone()
                .ok_or_else(|| anyhow!("Expected a the sponsor key"))?;
            let key = SponsorKey::new(key.as_str());
            User::Sponsor { email, key }
        }
    };

    Ok(user)
}
