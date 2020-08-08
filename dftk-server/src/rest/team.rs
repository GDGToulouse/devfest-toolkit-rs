use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

use dftk_common::models::team::PartialTeamMember;
use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::{with_repo, ServerContext, MAX_BODY_LENGTH};

///
/// Provide team members routes
///
/// `GET    site/team`: list all team members
///
/// `POST   site/team`: create a team member
///
/// `PUT    site/team/{id}` update a team member
///
/// `DELETE site/team/{id}` delete a team member

pub fn build_teams_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_team_member);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_teams);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and_then(delete_team_member);

    let update = warp::put()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Uuid>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(update_team_member);

    warp::path("team")
        .and(create.or(list).or(delete).or(update))
        .boxed()
}

async fn create_team_member(
    repos: Repositories,
    input: PartialTeamMember,
) -> Result<impl Reply, Rejection> {
    info!("Creating a new team member {:?}", input);
    let result = repos.team().create(input).await.map_err(Oops::db)?;
    debug!("Created the team member {:?} ", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn list_teams(repos: Repositories) -> Result<impl Reply, Rejection> {
    info!("Getting list of team members");
    let result = repos.team().find().await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn update_team_member(
    repos: Repositories,
    uuid: Uuid,
    input: PartialTeamMember,
) -> Result<impl Reply, Rejection> {
    info!("Update team member {:?}", input);
    let result = repos.team().update(uuid, input).await.map_err(Oops::db)?;
    info!("Updated the team members {:?}", result);
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn delete_team_member(repos: Repositories, uuid: Uuid) -> Result<impl Reply, Rejection> {
    info!("Deleting team member {:?}", uuid);
    let result = repos.team().delete(uuid).await.map_err(Oops::db)?;
    info!("Deleted the team member {:?}", result);

    result
        .map(|it| warp::reply::json(&it))
        .ok_or_else(warp::reject::not_found)
}
