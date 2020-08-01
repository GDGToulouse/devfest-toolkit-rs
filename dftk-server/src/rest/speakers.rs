use log::{debug, info};
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_common::models::speaker::{PartialSpeaker, SpeakerId, SpeakerKey};
use dftk_database::speakers::SpeakerPatch;
use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::{with_repo, ServerContext, MAX_BODY_LENGTH};

///
/// Provide speakers routes
///
/// `GET    site/speakers`: list all speakers
///
/// `GET    site/speakers/{key}`: get a speaker
///
/// `POST   site/speakers`: create a speaker
///
/// `PUT    site/speakers/{id}` update a speaker
///
/// `DELETE site/speakers/{id}` delete a speaker
///

pub fn build_speakers_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_speaker);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_speakers);

    let get = warp::get() //
        .and(with_repo(context.repos())) //
        .and(warp::path::param::<SpeakerKey>())
        .and_then(get_speaker);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<SpeakerId>())
        .and_then(delete_speaker);

    let patch = warp::patch()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<SpeakerId>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(patch_speaker);

    warp::path("speakers")
        .and(create.or(list).or(get).or(delete).or(patch))
        .boxed()
}

async fn create_speaker(
    repos: Repositories,
    input: PartialSpeaker,
) -> Result<impl Reply, Rejection> {
    info!("Creating a new speaker {:?}", input);
    let result = repos
        .speaker()
        .insert_speaker(input)
        .await
        .map_err(Oops::db)?;
    debug!("Created the speaker {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_speakers(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of speakers");
    let result = repos.speaker().find_all().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn get_speaker(repos: Repositories, key: SpeakerKey) -> Result<impl Reply, Rejection> {
    info!("Getting speaker {:?}", key);
    let result = repos.speaker().find_by_key(key).await.map_err(Oops::db)?;

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}

async fn patch_speaker(
    repos: Repositories,
    id: SpeakerId,
    input: SpeakerPatch,
) -> Result<impl Reply, Rejection> {
    info!("Update speaker {:?}", input);
    let result = repos
        .speaker()
        .update_speaker(id, input)
        .await
        .map_err(Oops::db)?;
    info!("Updated the speakers {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_speaker(repos: Repositories, id: SpeakerId) -> Result<impl Reply, Rejection> {
    info!("Deleting speaker {:?}", id);
    let result = repos.speaker().delete_speaker(id).await.map_err(Oops::db)?;
    info!("Deleted the speaker {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
