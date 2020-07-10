use std::marker::PhantomData;

use anyhow::{anyhow, Result};
use log::{debug, info};
use mongodb::bson::doc;
use mongodb::options::UpdateOptions;
use mongodb::{Collection, Database};
use serde::de::DeserializeOwned;
use serde::export::fmt::Debug;
use serde::Serialize;

use crate::{cursor_to_vec, from_document, to_document};

#[derive(Clone)]
pub struct MongodbRepository<T> {
    db_name: String,
    col_name: String,
    col: Collection,
    resource_type: PhantomData<T>,
}

impl<T> MongodbRepository<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    pub fn new(db: &Database, col_name: &str) -> Self {
        let db_name = db.name().into();
        let col = db.collection(col_name);
        let col_name = String::from(col_name);

        Self {
            db_name,
            col_name,
            col,
            resource_type: PhantomData,
        }
    }

    pub async fn insert(&self, element: &T) -> Result<bool> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        info!("Insert a {} from {}", col_name, db_name);
        let doc = to_document(element)?;
        let result = col.insert_one(doc, None).await?;
        debug!("...inserted {:?}", result);

        Ok(true)
    }

    pub async fn save_or_update(&self, id: &str, element: &T) -> Result<bool> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        debug!("Save or update a {} [{}] from {}", col_name, id, db_name);
        let doc = to_document(element)?;
        let option = UpdateOptions::builder().upsert(true).build();
        let result = col.update_one(doc! {"_id": id}, doc, option).await?;
        debug!("...saved {:?}", result);
        let result = if let Some(new_id) = result.upserted_id {
            debug!("...saved a {} with id [{}]", col_name, new_id);
            true
        } else {
            debug!("...updated a {} with id [{}]", col_name, id);
            false
        };

        Ok(result)
    }

    pub async fn update(&self, id: &str, element: &T) -> Result<bool> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        debug!("Update a {} [{}] from {}", col_name, id, db_name);
        let doc = to_document(element)?;
        let result = col.update_one(doc! {"_id": id}, doc, None).await?;
        debug!("...updated {:?}", result);

        Ok(result.modified_count == 1)
    }

    pub async fn find_first(&self) -> Result<T> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        info!("Find first {} from {}", col_name, db_name);
        let result = col.find_one(doc! {}, None).await?;
        debug!("...found first {:?}", result);
        let doc = result.ok_or_else(|| anyhow!("Missing {} in database", col_name))?;
        let result = from_document(doc)?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<T>> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        debug!("Find {} by id [{}] from {}", col_name, id, db_name);
        let result = col.find_one(doc! {"_id": &id}, None).await?;
        debug!("...found {}: {:?}", id, result);
        let option = match result {
            Some(doc) => Some(from_document(doc)?),
            None => None,
        };

        Ok(option)
    }

    pub async fn find_by_key(&self, key: &str) -> Result<Option<T>> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        debug!("Find {} by key '{}' from {}", col_name, key, db_name);
        let result = col.find_one(doc! {"key": &key}, None).await?;
        debug!("...found {}: {:?}", key, result);
        let option = match result {
            Some(doc) => Some(from_document::<T>(doc)?),
            None => None,
        };

        Ok(option)
    }

    pub async fn find_by_keys(&self, keys: &[String]) -> Result<Vec<T>> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        debug!("Find {} by keys {:?} from {}", col_name, keys, db_name);
        let mut cursor = col.find(doc! {"key": {"$in": keys}}, None).await?;
        let result = cursor_to_vec(&mut cursor).await?;
        debug!("...found {} {}", result.len(), col_name);

        Ok(result)
    }

    pub async fn find_all(&self) -> Result<Vec<T>> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        info!("Find all {} from {}", col_name, db_name);
        let mut cursor = col.find(doc! {}, None).await?;
        let result = cursor_to_vec(&mut cursor).await?;
        debug!("...found {} {}", result.len(), col_name);

        Ok(result)
    }

    pub async fn update_all(&self, elements: &[T]) -> Result<usize> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        self.remove_all().await?;
        info!("Update all {} from {}", col_name, db_name);
        let documents = elements.iter().filter_map(|elt| to_document::<T>(elt).ok());
        let result = col.insert_many(documents, None).await?;
        debug!("...updated {:?}", result);

        Ok(elements.len())
    }

    pub async fn remove_all(&self) -> Result<usize> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        info!("Remove all {} from {}", col_name, db_name);
        let result = col.delete_many(doc! {}, None).await?;
        debug!("...deleted {:?}", result);

        Ok(result.deleted_count as usize)
    }

    pub async fn remove_by_id(&self, id: &str) -> Result<Option<T>> {
        let MongodbRepository {
            col_name,
            db_name,
            col,
            ..
        } = self;
        debug!("Remove {} with id [{}] from {}", col_name, id, db_name);
        let result = col.find_one_and_delete(doc! {"_id": &id}, None).await?;
        debug!("...deleted {}: {:?}", id, result);
        let result = match result {
            Some(t) => Some(from_document::<T>(t)?),
            None => None,
        };

        Ok(result)
    }
}
