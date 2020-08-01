#[cfg(feature = "rest")]
use serde::Deserialize;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::rest::categories::build_session_categories_routes;
use crate::rest::formats::build_session_formats_routes;
use crate::rest::sessions::build_sessions_routes;
use crate::rest::site::build_site_routes;
use crate::rest::speakers::build_speakers_routes;
use crate::rest::sponsors::build_sponsors_routes;
use crate::rest::sponsors_categories::build_sponsor_categoryies_routes;
use crate::rest::team::build_teams_routes;
use crate::rest::team_member_type::build_team_member_types_routes;
use crate::rest::users::build_users_routes;
use crate::ServerContext;

mod categories;
mod formats;
mod sessions;
mod site;
mod speakers;
mod sponsors;
mod sponsors_categories;
mod team;
mod team_member_type;
mod users;

#[derive(Deserialize, Debug, Clone)]
struct NameDescription {
    name: String,
    description: Option<String>,
}

pub fn build_rest_routes(context: &ServerContext) -> BoxedFilter<(impl Reply,)> {
    let users = warp::path("users").and(build_users_routes(context));
    let site = warp::path("site").and(
        build_site_routes(context)
            .or(build_session_categories_routes(context))
            .or(build_session_formats_routes(context))
            .or(build_sessions_routes(context))
            .or(build_speakers_routes(context))
            .or(build_teams_routes(context))
            .or(build_team_member_types_routes(context))
            .or(build_sponsors_routes(context))
            .or(build_sponsor_categoryies_routes(context)),
    );

    users.or(site).with(warp::log("rest::api")).boxed()
}
