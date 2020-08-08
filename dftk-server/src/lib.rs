#[macro_use]
extern crate log;

use std::convert::Infallible;
use std::net::SocketAddr;

use anyhow::Result;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use dftk_conference_hall::ConferenceHallConfig;
use dftk_database::{MongodbConfig, Repositories};
use dftk_hugo_site::SiteConfig;

use crate::authentication::build_auth_routes;
use crate::rejection::handle_rejection;

pub mod authentication;
pub mod rejection;

#[cfg(feature = "rest")]
pub mod rest;

#[cfg(feature = "graphql")]
pub mod graphql;

const MAX_BODY_LENGTH: u64 = 1024 * 16; // 16kb

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
    pub graphql_path: String,
    pub rest_path: String,
}

impl ServerConfig {
    pub fn new(host: String, port: u32, graphql_path: String, rest_path: String) -> Self {
        ServerConfig {
            host,
            port,
            graphql_path,
            rest_path,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        let host = "127.0.0.1".into();
        let port = 8080;
        let graphql_path = "graphql".into();
        let rest_path = "api".into();

        ServerConfig {
            host,
            port,
            graphql_path,
            rest_path,
        }
    }
}

#[derive(Clone)]
pub struct ServerContext {
    site_config: SiteConfig,
    ch_config: ConferenceHallConfig,
    mongo_config: MongodbConfig,
    server_config: ServerConfig,
    repos: Repositories,
}

impl ServerContext {
    pub async fn build(
        site_config: SiteConfig,
        ch_config: ConferenceHallConfig,
        mongo_config: MongodbConfig,
        server_config: ServerConfig,
    ) -> Result<Self> {
        let repos = Repositories::build(&mongo_config).await?;
        let result = ServerContext {
            site_config,
            ch_config,
            mongo_config,
            server_config,
            repos,
        };

        Ok(result)
    }

    pub fn site_config(&self) -> SiteConfig {
        self.site_config.clone()
    }
    pub fn ch_config(&self) -> ConferenceHallConfig {
        self.ch_config.clone()
    }
    pub fn mongo_config(&self) -> MongodbConfig {
        self.mongo_config.clone()
    }
    pub fn server_config(&self) -> ServerConfig {
        self.server_config.clone()
    }
    pub fn repos(&self) -> Repositories {
        self.repos.clone()
    }
}

fn with_repo(
    repos: Repositories,
) -> impl Filter<Extract = (Repositories,), Error = Infallible> + Clone {
    warp::any().map(move || repos.clone())
}

fn with_context(
    context: ServerContext,
) -> impl Filter<Extract = (ServerContext,), Error = Infallible> + Clone {
    warp::any().map(move || context.clone())
}

// FIXME authentication authorization

// GraphQL routes
#[cfg(not(feature = "graphql"))]
fn graphql_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    warp::path(context.server_config().graphql_path)
        .map(|| warp::reply::with_status("GraphQL is disabled", StatusCode::METHOD_NOT_ALLOWED))
        .boxed()
}

#[cfg(feature = "graphql")]
fn graphql_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    use crate::graphql::build_graphql_routes;

    warp::path(context.server_config().graphql_path)
        .and(build_graphql_routes(&context))
        .boxed()
}

// Rest routes
#[cfg(not(feature = "rest"))]
fn rest_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    warp::path(context.server_config().rest_path)
        .map(|| warp::reply::with_status("REST is disabled", StatusCode::METHOD_NOT_ALLOWED))
        .boxed()
}

#[cfg(feature = "rest")]
fn rest_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    use crate::rest::build_rest_routes;
    warp::path(context.server_config().rest_path)
        .and(build_rest_routes(&context))
        .boxed()
}

// Auth routes
fn auth_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    warp::path("auth").and(build_auth_routes(&context)).boxed()
}

// All routes
fn routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let cors = warp::cors()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type"])
        .allow_any_origin();

    auth_routes(context)
        .or(graphql_routes(context))
        .or(rest_routes(context))
        .with(cors)
        .recover(handle_rejection)
        .boxed()
}

pub async fn run_server(context: ServerContext) -> Result<()> {
    let ServerConfig { host, port, .. } = context.server_config();

    // Routes
    let routes = routes(&context);

    let address = format!("{}:{}", host, port);
    let addr: SocketAddr = address.parse()?;

    warp::serve(routes).run(addr).await;

    Ok(())
}
