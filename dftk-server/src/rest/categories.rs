use log::{debug, info};
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::rest::{with_repo, NameDescription, MAX_BODY_LENGTH};
use crate::ServerContext;

///
/// Provide session categories routes
///
/// `GET    site/categories`: list all session categories
///
/// `POST   site/categories`: create a session category
///
/// `PUT    site/categories/{id}` update a session category
///
/// `DELETE site/categories/{id}` delete a session category

pub fn build_session_categories_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_category);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_categories);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and_then(delete_category);

    let update = warp::put()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(update_category);

    warp::path("categories")
        .and(create.or(list).or(delete).or(update))
        .boxed()
}

async fn create_category(
    repos: Repositories,
    name_description: NameDescription,
) -> Result<impl Reply, Rejection> {
    let NameDescription { name, description } = name_description;
    info!("Creating a new category {:?}", name);
    let result = repos
        .session_category()
        .create(name, description)
        .await
        .map_err(Oops::db)?;
    debug!("Created the category {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_categories(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of session categories");
    let result = repos.session_category().find().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn update_category(
    repos: Repositories,
    uuid: Uuid,
    name_description: NameDescription,
) -> Result<impl Reply, Rejection> {
    let NameDescription { name, description } = name_description;
    info!("Update category {}", name);
    let result = repos
        .session_category()
        .update(uuid, name, description)
        .await
        .map_err(Oops::db)?;
    info!("Updated the category {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_category(repos: Repositories, uuid: Uuid) -> Result<impl Reply, Rejection> {
    info!("Deleting category {:?}", uuid);
    let result = repos
        .session_category()
        .delete(uuid)
        .await
        .map_err(Oops::db)?;
    info!("Deleted the category {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
