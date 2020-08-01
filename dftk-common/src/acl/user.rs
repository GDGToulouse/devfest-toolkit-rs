use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use email::Mailbox;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::models::speaker::SpeakerKey;
use crate::models::sponsor::SponsorKey;
use core::fmt;
use serde::de::Visitor;

#[derive(Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Email(String);

impl Into<String> for Email {
    fn into(self) -> String {
        self.0
    }
}

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mailbox =
            Mailbox::from_str(s).map_err(|err| anyhow!("Invalid email '{}': {}", s, err))?;
        let email = Email(mailbox.to_string());

        Ok(email)
    }
}

struct EmailVisitor;

impl<'de> Visitor<'de> for EmailVisitor {
    type Value = Email;
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a valid language identifier string (see https://unicode.org/reports/tr35/#Unicode_language_identifier)")
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Email, E> {
        Email::from_str(value).map_err(|err| E::custom(format!("Error in deserializer {:?}", err)))
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(EmailVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

    fn email() -> Email {
        Email::from_str("plop@plop.org").unwrap()
    }

    fn speaker_key() -> SpeakerKey {
        SpeakerKey::new("plop")
    }

    fn sponsor_key() -> SponsorKey {
        SponsorKey::new("plop")
    }

    fn users() -> Vec<User> {
        vec![
            User::Guest,
            User::Admin { email: email() },
            User::Team { email: email() },
            User::Speaker {
                email: email(),
                key: speaker_key(),
            },
            User::Sponsor {
                email: email(),
                key: sponsor_key(),
            },
        ]
    }

    mod email {
        use super::*;

        #[test]
        fn should_create_from_valid_string() -> Result<()> {
            let str = "plop@plop.io";
            let result = Email::from_str(str)?;
            assert_eq!(result, Email(format!("<{}>", str)));
            Ok(())
        }

        #[test]
        fn should_reject_invalid_string() {
            let str = "plouf.com";
            let result = Email::from_str(str);
            assert!(result.is_err());
        }

        #[test]
        fn should_transform_to_string() {
            let str = "plop@plop.io";
            let email = Email(str.into());
            let result: String = email.into();
            assert_eq!(result, str);
        }
    }

    mod user {
        use async_graphql::serde_json;

        use super::*;

        #[test]
        fn should_be_serializable() {
            for user in users().iter() {
                let result = serde_json::to_string(user);
                assert!(result.is_ok())
            }
        }

        #[test]
        fn should_be_deserializable() {
            let json = r#"[
    { "type": "Guest" },
    { "type": "Admin", "email": "<plop@plop.org>" },
    { "type": "Team", "email": "<plop@plop.org>" },
    { "type": "Speaker", "email": "<plop@plop.org>", "key": "plop" },
    { "type": "Sponsor", "email": "<plop@plop.org>", "key": "plop" }
]"#;
            let result = serde_json::from_str::<Vec<User>>(json);
            assert!(result.is_ok());
        }
    }

    mod user_info {
        use super::*;

        fn user_info() -> Vec<UserInfo> {
            users()
                .iter()
                .map(|it| UserInfo {
                    user: it.clone(),
                    need_change_password: true,
                })
                .collect()
        }

        #[test]
        fn should_be_serializable() {
            for user_info in user_info().iter() {
                let result = serde_json::to_string(user_info);
                assert!(result.is_ok())
            }
        }

        #[test]
        fn should_be_deserializable() {
            let json = r#"[
    { "user": { "type": "Guest" }, "need_change_password": true },
    { "user": { "type": "Admin", "email": "<plop@plop.org>" }, "need_change_password": true },
    { "user": { "type": "Team", "email": "<plop@plop.org>" }, "need_change_password": true },
    { "user": { "type": "Speaker", "email": "<plop@plop.org>", "key": "plop" }, "need_change_password": true },
    { "user": { "type": "Sponsor", "email": "<plop@plop.org>", "key": "plop" }, "need_change_password": true }
]"#;
            let result = serde_json::from_str::<Vec<UserInfo>>(json);
            assert!(result.is_ok());
        }

        #[test]
        fn should_get_user() {
            let user_info = user_info().get(3).cloned().unwrap();
            let result = user_info.user();
            assert_eq!(result, users().get(3).unwrap().clone())
        }
        #[test]
        fn should_get_need_change_password() {
            let user_info = user_info().get(3).cloned().unwrap();
            let result = user_info.need_change_password();
            assert_eq!(result, true)
        }
    }
}
