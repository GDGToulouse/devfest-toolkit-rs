use std::str::FromStr;

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::models::speaker::SpeakerKey;
use crate::models::sponsor::SponsorKey;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Email(String);

impl Into<String> for Email {
    fn into(self) -> String {
        self.0
    }
}

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // FIXME check
        let email = Email(s.into());

        Ok(email)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum User {
    Guest,
    Admin { email: Email },
    Team { email: Email },
    Speaker { email: Email, key: SpeakerKey },
    Sponsor { email: Email, key: SponsorKey },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    user: User,
    need_change_password: bool,
}

impl UserInfo {
    pub fn new(user: User, need_change_password: bool) -> Self {
        UserInfo {
            user,
            need_change_password,
        }
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn need_change_password(&self) -> bool {
        self.need_change_password
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn should_create_email_from_valid_string() -> Result<()> {
        let str = "plop@plop.io";

        let result = Email::from_str(str)?;

        assert_eq!(result, Email(str.into()));

        Ok(())
    }
}
