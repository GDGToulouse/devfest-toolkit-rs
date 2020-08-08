#[macro_use]
extern crate log;

use uuid::Uuid;

pub mod acl;
pub mod models;

#[cfg(feature = "graphql")]
pub mod graphql;

pub fn new_id() -> Uuid {
    Uuid::new_v4()
}
