use log::info;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use dftk_common::models::site::SiteInfo;
use dftk_conference_hall::read_event;
use dftk_database::Repositories;
use dftk_hugo_site::generate;

use crate::rejection::Oops;
use crate::{with_context, with_repo, ServerContext, MAX_BODY_LENGTH};

///
/// Provide site routes
///
/// `GET  site`: get site
///
/// `POST site/synchronize`: fetch site info, talks and speakers from Conference Hall and update the database
///
/// `POST site/generate`: generate Hugo Site files (speaker, sessions, team, sponsors, schedule)
///
/// `GET  site/info`: get site info
///
/// `POST site/info`: set site info
///

pub fn build_site_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let site = warp::path::end().and(
        warp::get()
            .and(with_repo(context.repos()))
            .and_then(get_site),
    );

    let synchronize = warp::path("synchronize").and(
        warp::post()
            .and(with_context(context.clone()))
            .and(warp::body::content_length_limit(0))
            .and_then(synchronize_site),
    );

    let generate = warp::path("generate").and(
        warp::post()
            .and(with_context(context.clone()))
            .and(warp::body::content_length_limit(0))
            .and_then(generate_site),
    );

    let site_info = warp::path("info").and(
        warp::get() //
            .and(with_repo(context.repos()))
            .and_then(get_site_info),
    );

    let update_site_info = warp::path("info").and(
        warp::post()
            .and(with_repo(context.repos()))
            .and(warp::body::content_length_limit(MAX_BODY_LENGTH))
            .and(warp::body::json())
            .and_then(set_site_info),
    );

    site.or(synchronize)
        .or(generate)
        .or(site_info)
        .or(update_site_info)
        .boxed()
}

async fn get_site(repos: Repositories) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Getting site");
    let site = repos.load_site().await.map_err(Oops::db)?;
    let result = warp::reply::json(&site);

    Ok(result)
}

async fn synchronize_site(context: ServerContext) -> Result<impl warp::Reply, warp::Rejection> {
    let site = read_event(&context.ch_config).await.map_err(Oops::ch)?;
    let result = context.repos.synchronize(site).await.map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn generate_site(context: ServerContext) -> Result<impl warp::Reply, warp::Rejection> {
    let site = context.repos.load_site().await.map_err(Oops::ch)?;
    let result = generate(&context.site_config(), site)
        .await
        .map_err(Oops::db)?;
    let result = warp::reply::json(&result);

    Ok(result)
}

async fn get_site_info(repos: Repositories) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Getting site info");
    let site_info = repos.info().find_first().await.map_err(Oops::db)?;
    let result = warp::reply::json(&site_info);

    Ok(result)
}

async fn set_site_info(
    repos: Repositories,
    site_info: SiteInfo,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Update site info {:?}", site_info);
    repos.info().remove_all().await.map_err(Oops::db)?;
    repos.info().insert(&site_info).await.map_err(Oops::db)?;

    let result = warp::reply::reply();
    Ok(result)
}
