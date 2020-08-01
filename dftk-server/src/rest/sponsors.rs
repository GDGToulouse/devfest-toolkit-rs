use log::{debug, info};
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_common::models::sponsor::{PartialSponsor, SponsorKey};
use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::{with_repo, ServerContext, MAX_BODY_LENGTH};

///
/// Provide sponsors routes
///
/// `GET    site/sponsors`: list all sponsors
///
/// `GET    site/sponsors/{key}`: get a sponsor
///
/// `POST   site/sponsors`: create a sponsor
///
/// `PUT    site/sponsors/{id}` update a sponsor
///
/// `DELETE site/sponsors/{id}` delete a sponsor

pub fn build_sponsors_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_sponsor);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_sponsors);

    let get = warp::get() //
        .and(with_repo(context.repos())) //
        .and(warp::path::param::<SponsorKey>())
        .and_then(get_sponsor);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and_then(delete_sponsor);

    let update = warp::put()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(update_sponsor);

    warp::path("sponsors")
        .and(create.or(list).or(get).or(delete).or(update))
        .boxed()
}

async fn create_sponsor(
    repos: Repositories,
    input: PartialSponsor,
) -> Result<impl Reply, Rejection> {
    info!("Creating a new sponsor {:?}", input);
    let result = repos.sponsor().create(input).await.map_err(Oops::db)?;
    debug!("Created the sponsor {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_sponsors(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of sponsors");
    let result = repos.sponsor().find().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn get_sponsor(repos: Repositories, key: SponsorKey) -> Result<impl Reply, Rejection> {
    info!("Getting sponsor {:?}", key);
    let result = repos.sponsor().find_by_key(key).await.map_err(Oops::db)?;

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}

async fn update_sponsor(
    repos: Repositories,
    uuid: Uuid,
    input: PartialSponsor,
) -> Result<impl Reply, Rejection> {
    info!("Update sponsor {:?}", input);
    let result = repos
        .sponsor()
        .update(uuid, input)
        .await
        .map_err(Oops::db)?;
    info!("Updated the sponsors {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_sponsor(repos: Repositories, uuid: Uuid) -> Result<impl Reply, Rejection> {
    info!("Deleting sponsor {:?}", uuid);
    let result = repos.sponsor().delete(uuid).await.map_err(Oops::db)?;
    info!("Deleted the sponsor {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
