use std::sync::Arc;

use error_responder::ErrorResponder;
use rocket::tokio::sync::broadcast;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

use crate::oauth::CredentialsError;

#[derive(ErrorResponder, Error, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum ApplicationError {
    #[error("missing authentication cookie")]
    #[respond("Unauthorized")]
    MissingAuthentication,

    #[error("invalid WebSocket payload; expected JSON control message of the form: \
         {{\"type\":\"add|remove\",\"symbols\":[\"AAPL\",\"MSFT\",...]}}")]
    #[respond("BadRequest")]
    InvalidWebSocketPayload,

    #[error("invalid credentials string: {0}")]
    #[respond("Unauthorized")]
    InvalidCredentials(#[from(CredentialsError)] #[serde(skip)] CredentialsError),

    #[error("network error: {0}")]
    #[respond("InternalServerError")]
    Network(#[from(reqwest::Error)] #[serde(skip)] reqwest::Error),

    #[error("expected json: {0}")]
    #[respond("InternalServerError")]
    InvalidJson(#[from(serde_json::Error)] #[serde(skip)] serde_json::Error),

    #[error("{index} not found in {object}")]
    #[respond("InternalServerError")]
    InvalidJsonLookup { index: String, object: Value },

    #[error("SchwabAccount deserialization failed: {0}")]
    #[respond("InternalServerError")]
    SchwabAccountDeserialization(#[from(serde_json::Error)] #[serde(skip)] serde_json::Error),

    #[error("QuoteResponse deserialization failed: {0}")]
    #[respond("InternalServerError")]
    QuoteResponseDeserialization(#[from(serde_json::Error)] #[serde(skip)] serde_json::Error),

    #[error("channel failed: {0}")]
    #[respond("InternalServerError")]
    ChannelBroadcastFailed(#[from(broadcast::error::RecvError)] #[serde(skip)] broadcast::error::RecvError),

    /// Allows for many channels to share an ApplicationError from broadcast
    #[error("shared error in polling: {0}")]
    #[respond("InternalServerError")]
    Polling(Arc<ApplicationError>),

    #[error("missing required query parameters: {0:?}")]
    #[respond("BadRequest")]
    MissingQueryParameters(Vec<String>),
}