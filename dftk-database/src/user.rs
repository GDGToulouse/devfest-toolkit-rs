use anyhow::{anyhow, Result};
use argon2::Config;
use bson::Document;
use chbs::passphrase;
use log::{debug, info};
use mongodb::bson::doc;
use mongodb::options::{FindOneAndUpdateOptions, FindOptions};
use mongodb::{Collection, Database};
use rand::Rng;
use serde::Serialize;
use tokio::stream::StreamExt;

use dftk_common::acl::user::{Email, User, UserInfo};

use crate::from_document;

#[derive(Clone)]
struct Password(String);

impl Password {
    fn new(s: &[u8]) -> Self {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        let config = Config::default();
        let hash = argon2::hash_encoded(s, &salt, &config).unwrap();
        Password(hash)
    }

    fn verify(&self, password: &[u8]) -> bool {
        argon2::verify_encoded(self.0.as_str(), password).unwrap_or(false)
    }
}

#[derive(Serialize)]
struct UserDocument {
    user: User,
    need_change_password: bool,
    password: String,
}

#[derive(Clone)]
pub struct UserRepository {
    col: Collection,
}

impl UserRepository {
    pub async fn build(db: &Database) -> Result<Self> {
        let col = db.collection("users");
        let result = Self { col };

        Ok(result)
    }

    pub async fn find_all(&self) -> Result<Vec<User>> {
        let doc = FindOptions::builder()
            .projection(Some(user_projection()))
            .build();
        let mut cursor = self.col.find(doc! {}, Some(doc)).await?;
        let mut result = vec![];
        while let Some(doc) = cursor.next().await {
            let doc = doc.map_err(|err| anyhow!("Oops, {}", err))?;
            let user = get_user(&doc)?;
            result.push(user);
        }

        Ok(result)
    }

    pub async fn authenticate(&self, email: &Email, password: &[u8]) -> Result<UserInfo> {
        info!("Try authenticate {:?}", email);
        let query = query_by_email(email);
        debug!("authenticate query: {:#?}", query);
        let result = self.col.find_one(query, None).await?;

        if let Some(doc) = result {
            let db_password = get_password(&doc)?;
            if db_password.verify(password) {
                let user_info = get_user_info(&doc)?;
                Ok(user_info)
            } else {
                Err(anyhow!("No user found this e-mail & password"))
            }
        } else {
            Err(anyhow!("No user found this e-mail & password"))
        }
    }

    pub async fn new_user(&self, user: User) -> Result<String> {
        let generated = passphrase();
        let new_password = Password::new(generated.as_bytes());
        let need_change_password = true;
        let user_doc = UserDocument {
            user,
            need_change_password,
            password: new_password.clone().0,
        };
        let bson = bson::to_bson(&user_doc)?;
        let doc = bson.as_document().cloned().unwrap();
        self.col.insert_one(doc, None).await?;
        Ok(generated)
    }

    pub async fn delete_user(&self, email: &Email) -> Result<i64> {
        let query = query_by_email(email);
        let result = self.col.delete_one(query, None).await?;

        Ok(result.deleted_count)
    }

    pub async fn change_password(
        &self,
        email: &Email,
        old_password: &[u8],
        new_password: &[u8],
    ) -> Result<User> {
        // Check old password
        self.authenticate(email, old_password).await?;

        let filter = query_by_email(email);

        let new_password = Password::new(new_password);
        let update: Document = doc! {
            "$set": {
                "password" : new_password.0,
                "need_change_password" : false,
            }
        };
        let options = FindOneAndUpdateOptions::builder()
            .projection(Some(user_projection()))
            .build();
        let result = self
            .col
            .find_one_and_update(filter, update, Some(options))
            .await?;

        match result {
            None => Err(anyhow!("No user found this e-mail or old password")),
            Some(doc) => get_user(&doc),
        }
    }
}

fn query_by_email(email: &Email) -> Document {
    let email: String = email.clone().into();
    doc! { "user.email": email }
}

fn user_projection() -> Document {
    doc! {
        "_id": 0,
        "user": 1,
    }
}

fn get_user(doc: &Document) -> Result<User> {
    let doc = doc.get_document("user")?;
    let user = from_document::<User>(doc.clone())?;

    Ok(user)
}

fn get_password(doc: &Document) -> Result<Password> {
    let pwd = doc.get_str("password")?;
    let result = Password(pwd.into());

    Ok(result)
}

fn get_user_info(doc: &Document) -> Result<UserInfo> {
    let user = get_user(doc)?;
    let need_change_password = doc.get_bool("need_change_password")?;
    let user_info = UserInfo::new(user, need_change_password);

    Ok(user_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod password {
        use super::*;

        #[test]
        fn should_accept_valid_password() {
            let pwd = "pl0p".as_bytes();
            let password = Password::new(pwd);
            let result = password.verify(pwd);
            assert_eq!(result, true)
        }

        #[test]
        fn should_reject_valid_password() {
            let pwd = "pl0p".as_bytes();
            let password = Password::new(pwd);
            let result = password.verify("plaf".as_bytes());
            assert_eq!(result, false)
        }
    }
}
