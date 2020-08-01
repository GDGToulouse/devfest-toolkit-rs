use log::{debug, info};
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::{with_repo, ServerContext, MAX_BODY_LENGTH};

///
/// Provide team member types routes
///
/// `GET    site/member-types`: list all team member types
///
/// `POST   site/member-types`: create a member type
///
/// `PUT    site/member-types/{id}` update a member type
///
/// `DELETE site/member-types/{id}` delete a member type

pub fn build_team_member_types_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_member_type);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_member_types);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and_then(delete_member_type);

    let update = warp::put()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(update_member_type);

    warp::path("member-types")
        .and(create.or(list).or(delete).or(update))
        .boxed()
}

async fn create_member_type(repos: Repositories, name: String) -> Result<impl Reply, Rejection> {
    info!("Creating a new member type {:?}", name);
    let result = repos.member_type().create(name).await.map_err(Oops::db)?;
    debug!("Created the member type {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_member_types(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of team member types");
    let result = repos.member_type().find().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn update_member_type(
    repos: Repositories,
    uuid: Uuid,
    name: String,
) -> Result<impl Reply, Rejection> {
    info!("Update member type {}", name);
    let result = repos
        .member_type()
        .update(uuid, name)
        .await
        .map_err(Oops::db)?;
    info!("Updated the member type {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_member_type(repos: Repositories, uuid: Uuid) -> Result<impl Reply, Rejection> {
    info!("Deleting member type {:?}", uuid);
    let result = repos.member_type().delete(uuid).await.map_err(Oops::db)?;
    info!("Deleted the member type {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
