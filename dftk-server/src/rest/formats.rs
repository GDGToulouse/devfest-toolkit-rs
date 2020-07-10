use log::{debug, info};
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::rest::{with_repo, NameDescription, MAX_BODY_LENGTH};
use crate::ServerContext;

///
/// Provide session formats routes
///
/// `GET    site/formats`: list all session formats
///
/// `POST   site/formats`: create a session format
///
/// `PUT    site/formats/{id}` update a session format
///
/// `DELETE site/formats/{id}` delete a session format

pub fn build_session_formats_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_format);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_formats);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and_then(delete_format);

    let update = warp::put()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(update_format);

    warp::path("formats")
        .and(create.or(list).or(delete).or(update))
        .boxed()
}

async fn create_format(
    repos: Repositories,
    name_description: NameDescription,
) -> Result<impl Reply, Rejection> {
    let NameDescription { name, description } = name_description;
    info!("Creating a new format {:?}", name);
    let result = repos
        .session_format()
        .create(name, description)
        .await
        .map_err(Oops::db)?;
    debug!("Created the format {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_formats(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of session formats");
    let result = repos.session_format().find().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn update_format(
    repos: Repositories,
    uuid: Uuid,
    name_description: NameDescription,
) -> Result<impl Reply, Rejection> {
    let NameDescription { name, description } = name_description;
    info!("Update format {}", name);
    let result = repos
        .session_format()
        .update(uuid, name, description)
        .await
        .map_err(Oops::db)?;
    info!("Updated the format {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_format(repos: Repositories, uuid: Uuid) -> Result<impl Reply, Rejection> {
    info!("Deleting format {:?}", uuid);
    let result = repos
        .session_format()
        .delete(uuid)
        .await
        .map_err(Oops::db)?;
    info!("Deleted the format {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
