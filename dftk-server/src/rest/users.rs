use log::{debug, info};
use serde::Deserialize;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use dftk_common::acl::user::{Email, User};
use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::rest::{with_repo, MAX_BODY_LENGTH};
use crate::ServerContext;

/// Provide user routes
///
/// `GET     users`: list users
///
/// `POST    users`: create a new user
///
/// `PUT     users/{email}`: update user password
///
/// `DELETE  users/{email}`: delete user

pub fn build_users_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let create = warp::post()
        .and(with_repo(context.repos()))
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(create_user);

    let list = warp::get() //
        .and(with_repo(context.repos())) //
        .and_then(list_users);

    let delete = warp::delete()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Email>())
        .and_then(delete_user);

    let update = warp::put()
        .and(with_repo(context.repos()))
        .and(warp::path::param::<Email>())
        .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
        .and(warp::body::json())
        .and_then(update_user);

    create.or(list).or(delete).or(update).boxed()
}

async fn create_user(repos: Repositories, user: User) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Creating a new user {:?}", user);
    let pwd = repos
        .user()
        .new_user(user.clone())
        .await
        .map_err(Oops::db)?;
    // FIXME remote the password from log, send an email
    debug!("Created user {:?} with the new password: '{}'", user, pwd);
    let result = warp::reply::reply();
    Ok(result)
}

async fn list_users(repos: Repositories) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Getting list of users");
    let users = repos.user().find_all().await.map_err(Oops::db)?;
    let result = warp::reply::json(&users);

    Ok(result)
}

#[derive(Deserialize, Debug, Clone)]
struct ChangePassword {
    old: String,
    new: String,
}

async fn update_user(
    repos: Repositories,
    email: Email,
    change_password: ChangePassword,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Update user {:?} password", email);
    let ChangePassword { old, new } = change_password;
    let info = repos
        .user()
        .change_password(&email, old.as_str(), new.as_str())
        .await
        .map_err(Oops::db)?;
    info!("Updated user {:?}", info);
    let result = warp::reply::reply();
    Ok(result)
}

async fn delete_user(
    repos: Repositories,
    email: Email,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Deleting user {:?}", email);
    repos.user().delete_user(&email).await.map_err(Oops::db)?;
    info!("Deleted user {:?}", email);
    let result = warp::reply::reply();
    Ok(result)
}
