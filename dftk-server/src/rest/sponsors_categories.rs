use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::{with_repo, ServerContext, MAX_BODY_LENGTH};

///
/// Provide team member types routes
///
/// `GET    site/sponsor-categories`: list all sponsor categories
///
/// `POST   site/sponsor-categories`: create a sponsor category
///
/// `PUT    site/sponsor-categories/{id}` update a sponsor category
///
/// `DELETE site/sponsor-categories/{id}` delete a sponsor category

pub fn build_sponsor_categoryies_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_sponsor_category);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_sponsor_categories);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and_then(delete_sponsor_category);

    let update = warp::put()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(update_sponsor_category);

    warp::path("sponsor-categories")
        .and(create.or(list).or(delete).or(update))
        .boxed()
}

async fn create_sponsor_category(
    repos: Repositories,
    name: String,
) -> Result<impl Reply, Rejection> {
    info!("Creating a new sponsor category {:?}", name);
    let result = repos
        .sponsor_category()
        .create(name)
        .await
        .map_err(Oops::db)?;
    debug!("Created the sponsor category {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_sponsor_categories(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of sponsor categories");
    let result = repos.sponsor_category().find().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn update_sponsor_category(
    repos: Repositories,
    uuid: Uuid,
    name: String,
) -> Result<impl Reply, Rejection> {
    info!("Update sponsor category {}", name);
    let result = repos
        .sponsor_category()
        .update(uuid, name)
        .await
        .map_err(Oops::db)?;
    info!("Updated the sponsor category {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_sponsor_category(repos: Repositories, uuid: Uuid) -> Result<impl Reply, Rejection> {
    info!("Deleting sponsor category {:?}", uuid);
    let result = repos
        .sponsor_category()
        .delete(uuid)
        .await
        .map_err(Oops::db)?;
    info!("Deleted the sponsor category {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
