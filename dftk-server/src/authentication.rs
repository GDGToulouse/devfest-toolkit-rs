use std::collections::HashMap;
use std::str::FromStr;

use cookie::{Cookie, SameSite};
use log::info;
use time::Duration;
use warp::filters::BoxedFilter;
use warp::http::Response;
use warp::{Filter, Reply};

use dftk_common::acl::user::Email;
use dftk_database::Repositories;

use crate::rejection::Oops;
use crate::{with_repo, ServerContext, MAX_BODY_LENGTH};

// FIXME see https://blog.joco.dev/posts/warp_auth_server_tutorial

pub fn build_auth_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let login = warp::path("login").and(
        warp::post()
            .and(with_repo(context.repos()))
            .and(warp::body::form())
            .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
            .and_then(do_login),
    );

    let logout = warp::path("logout").and(
        warp::post()
            .and(with_repo(context.repos()))
            .and(warp::body::content_length_limit(0))
            .and_then(do_logout),
    );

    login.or(logout).boxed()
}

async fn do_login(
    repo: Repositories,
    form: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let email = form.get("email").ok_or_else(|| Oops::missing("email"))?;
    let email = Email::from_str(email.as_str()).map_err(|err| Oops::bad("email", err))?;
    let password = form
        .get("password")
        .ok_or_else(|| Oops::missing("password"))?;

    let user_info = repo
        .user()
        .authenticate(&email, password.as_bytes())
        .await
        .map_err(Oops::auth)?;
    info!("User authenticated {:?}", user_info);
    let json = serde_json::to_string(&user_info).map_err(|err| Oops::other(err.into()))?;
    let value = base64::encode(json);

    let cookie: Cookie = Cookie::build("auth", value)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(Duration::days(1))
        .finish();

    let result = Response::builder()
        // Set cookie
        .header("Set-Cookie", cookie.to_string())
        .body("Welcome")
        .map_err(|err| Oops::other(err.into()))?;

    Ok(result)
}

async fn do_logout(_repo: Repositories) -> Result<impl warp::Reply, warp::Rejection> {
    Ok("TODO logout")
}

// FIXME filter authenticated
// check cookie
// parse JWT
// verify JWT
// check not rejected

// FIXME filter authorization with Operation
