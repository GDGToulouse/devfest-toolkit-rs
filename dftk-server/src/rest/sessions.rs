use log::{debug, info};
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_common::models::session::{PartialSession, SessionId, SessionKey};
use dftk_database::sessions::SessionPatch;
use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::{with_repo, ServerContext, MAX_BODY_LENGTH};

///
/// Provide sessions routes
///
/// `GET    site/sessions`: list all sessions
///
/// `GET    site/sessions/{key}`: get a session
///
/// `POST   site/sessions`: create a session
///
/// `PUT    site/sessions/{id}` update a session
///
/// `DELETE site/sessions/{id}` delete a session
///

pub fn build_sessions_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_session);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_sessions);

    let get = warp::get() //
        .and(with_repo(context.repos())) //
        .and(warp::path::param::<SessionKey>())
        .and_then(get_session);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<SessionId>())
        .and_then(delete_session);

    let patch = warp::patch()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<SessionId>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(patch_session);

    warp::path("sessions")
        .and(create.or(list).or(get).or(delete).or(patch))
        .boxed()
}

async fn create_session(
    repos: Repositories,
    input: PartialSession,
) -> Result<impl Reply, Rejection> {
    info!("Creating a new session {:?}", input);
    let result = repos
        .session()
        .insert_session(input)
        .await
        .map_err(Oops::db)?;
    debug!("Created the session {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_sessions(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of sessions");
    let result = repos.session().find_all().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn get_session(repos: Repositories, key: SessionKey) -> Result<impl Reply, Rejection> {
    info!("Getting session {:?}", key);
    let result = repos.session().find_by_key(key).await.map_err(Oops::db)?;

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}

async fn patch_session(
    repos: Repositories,
    id: SessionId,
    input: SessionPatch,
) -> Result<impl Reply, Rejection> {
    info!("Update session {:?}", input);
    let result = repos
        .session()
        .update_session(id, input)
        .await
        .map_err(Oops::db)?;
    info!("Updated the sessions {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_session(repos: Repositories, id: SessionId) -> Result<impl Reply, Rejection> {
    info!("Deleting session {:?}", id);
    let result = repos.session().delete_session(id).await.map_err(Oops::db)?;
    info!("Deleted the session {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
