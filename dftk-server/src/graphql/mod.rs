#[cfg(feature = "graphql")]
use std::convert::Infallible;

use async_graphql::extensions::ApolloTracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, QueryBuilder, Schema};
use async_graphql_warp::GQLResponse;
use warp::filters::BoxedFilter;
use warp::http::Response;
use warp::{Filter, Reply};

use crate::graphql::mutation::MutationSite;
use crate::graphql::query::QuerySite;
use crate::ServerContext;

mod mutation;
mod query;

mod categories;
mod formats;
mod info;
mod languages;
mod schedule;
mod sessions;
mod socials;
mod speakers;
mod sponsors;
mod teams;
mod user;

pub type SiteSchema = Schema<QuerySite, MutationSite, EmptySubscription>;

pub fn build_schema(context: &ServerContext) -> SiteSchema {
    Schema::build(QuerySite, MutationSite, EmptySubscription)
        .data(context.clone())
        .data(context.ch_config())
        .data(context.site_config())
        .data(context.repos())
        .extension(ApolloTracing::default) // Enable ApolloTracing extension
        .finish()
}

pub fn build_graphql_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let schema = build_schema(&context);
    let graphql_path = context.server_config().graphql_path;

    let graphql_path = format!("/{}", graphql_path);
    let graphql_playground = warp::path::end() // End
        .and(warp::get()) // Get
        .map(move || {
            let playground = GraphQLPlaygroundConfig::new(graphql_path.as_str());
            Response::builder()
                .header("content-type", "text/html")
                .body(playground_source(playground))
        });

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(GQLResponse::from(resp))
        },
    );

    graphql_playground
        .or(graphql_post)
        .with(warp::log("graphql::api"))
        .boxed()
}
