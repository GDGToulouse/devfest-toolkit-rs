use std::convert::Infallible;

use anyhow::Error;
use serde::Serialize;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::reject::{MethodNotAllowed, Reject};
use warp::{Rejection, Reply};

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

/// Bad things happen
#[derive(Debug)]
pub enum Oops {
    DatabaseIssue(String),
    ConferenceHallIssue(String),
    Authentication(String),
    MissingField(String),
    BadField(String),
    Other(String),
}

impl Oops {
    pub fn db(err: Error) -> Rejection {
        let message = format!("Database issue: {}", err);
        warp::reject::custom(Oops::DatabaseIssue(message))
    }
    pub fn ch(err: Error) -> Rejection {
        let message = format!("Error during call to Conference Hall: {}", err);
        warp::reject::custom(Oops::ConferenceHallIssue(message))
    }
    pub fn auth(err: Error) -> Rejection {
        let message = format!("Authentication issue: {}", err);
        warp::reject::custom(Oops::Authentication(message))
    }
    pub fn missing(field: &str) -> Rejection {
        let message = format!("Missing the field '{}'", field);
        warp::reject::custom(Oops::MissingField(message))
    }
    pub fn bad(field: &str, err: Error) -> Rejection {
        let message = format!("Invalid field '{}': {}", field, err);
        warp::reject::custom(Oops::BadField(message))
    }
    pub fn other(err: Error) -> Rejection {
        let message = format!("Oops! {}", err);
        warp::reject::custom(Oops::Other(message))
    }
}

impl Reject for Oops {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    info!("Oops: {:?}", err);

    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".into();
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("{}", e)
    } else if err.find::<MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".into();
    } else {
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
