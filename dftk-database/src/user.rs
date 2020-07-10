use anyhow::{anyhow, Result};
use bson::Document;
use chbs::passphrase;
use mongodb::bson::doc;
use mongodb::options::{FindOneAndUpdateOptions, FindOneOptions, FindOptions};
use mongodb::{Collection, Database};
use serde::Serialize;
use tokio::stream::StreamExt;

use crate::from_document;
use dftk_common::acl::user::{Email, User, UserInfo};

#[derive(Serialize, Clone)]
struct Password(String);

impl From<&str> for Password {
    fn from(s: &str) -> Self {
        Password::new(s)
    }
}

impl Password {
    fn new(s: &str) -> Self {
        // FIXME should slat a salted
        // https://rust-lang-nursery.github.io/rust-cookbook/cryptography/encryption.html
        Password(s.into())
    }
}

#[derive(Serialize)]
struct UserDocument {
    user: User,
    need_change_password: bool,
    password: Password,
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

    pub async fn authenticate(&self, email: &Email, password: &str) -> Result<UserInfo> {
        let password = Password::new(password);
        let query = query_by_email_password(email, &password);
        let doc = FindOneOptions::builder()
            .projection(Some(user_info_projection()))
            .build();
        let result = self.col.find_one(query, Some(doc)).await?;
        let doc = result.ok_or_else(|| anyhow!("No user found this e-mail or old password"))?;
        let user_info = get_user_info(&doc)?;

        Ok(user_info)
    }

    pub async fn new_user(&self, user: User) -> Result<String> {
        let new_password = passphrase();
        let need_change_password = true;
        let user_doc = UserDocument {
            user,
            need_change_password,
            password: Password::new(new_password.as_str()),
        };
        let bson = bson::to_bson(&user_doc)?;
        let doc = bson.as_document().cloned().unwrap();
        self.col.insert_one(doc, None).await?;
        Ok(new_password)
    }

    pub async fn delete_user(&self, email: &Email) -> Result<i64> {
        let query = query_by_email(email);
        let result = self.col.delete_one(query, None).await?;

        Ok(result.deleted_count)
    }

    pub async fn change_password(
        &self,
        email: &Email,
        old_password: &str,
        new_password: &str,
    ) -> Result<User> {
        let old_password = Password::new(old_password);
        let filter = query_by_email_password(email, &old_password);

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

fn query_by_email_password(email: &Email, password: &Password) -> Document {
    let email: String = email.clone().into();
    let password: String = password.0.clone();
    doc! {
        "user.email": email,
        "password": password,
    }
}

fn user_projection() -> Document {
    doc! {
        "_id": 0,
        "user": 1,
    }
}

fn user_info_projection() -> Document {
    doc! {
        "_id": 0,
        "user": 1,
        "need_change_password": 1,
    }
}

fn get_user(doc: &Document) -> Result<User> {
    let doc = doc.get_document("user")?;
    let user = from_document::<User>(doc.clone())?;

    Ok(user)
}

fn get_user_info(doc: &Document) -> Result<UserInfo> {
    let user = get_user(doc)?;
    let need_change_password = doc.get_bool("need_change_password")?;
    let user_info = UserInfo::new(user, need_change_password);

    Ok(user_info)
}
