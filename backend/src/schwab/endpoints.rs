use std::collections::HashSet;

use rocket::form::FromForm;
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio::select;
use rocket::{State, http::CookieJar, serde::json::Json};
use rocket_oauth2::OAuth2;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ws::{Message, WebSocket};

use crate::{
    errors::ApplicationError,
    oauth::{AUTH_COOKIE_NAME, Credentials, Schwab},
    quotes::QuotesState,
    schwab::{SchwabUsers, get_user, schema::QuoteResponse},
};

#[get("/user")]
pub async fn user(
    oauth2: OAuth2<Schwab>,
    cookies: &CookieJar<'_>,
) -> Result<Json<SchwabUsers>, ApplicationError> {
    let auth = cookies
        .get_private(AUTH_COOKIE_NAME)
        .ok_or(ApplicationError::MissingAuthentication)?;

    let mut credentials =
        Credentials::decode(auth.value()).map_err(ApplicationError::InvalidCredentials)?;

    let user = get_user(&mut credentials, &oauth2).await?;

    Ok(Json::from(SchwabUsers(user)))
}

#[post("/refresh_token")]
pub async fn refresh_token_debug(
    oauth2: OAuth2<Schwab>,
    cookies: &CookieJar<'_>,
) -> Result<Value, ApplicationError> {
    let auth = cookies
        .get_private(AUTH_COOKIE_NAME)
        .ok_or(ApplicationError::MissingAuthentication)?;

    let mut credentials =
        Credentials::decode(auth.value()).map_err(ApplicationError::InvalidCredentials)?;

    credentials
        .refresh_access_token(&oauth2)
        .await
        .map_err(ApplicationError::InvalidCredentials)?;

    Ok(serde_json::json!({
            "success": true,
    }))
}

#[derive(FromForm)]
pub struct QuotesQuery {
    pub symbols: Option<String>,
}

#[get("/quotes?<q..>")]
pub async fn quotes(
    oauth2: OAuth2<Schwab>,
    cookies: &CookieJar<'_>,
    qm: &State<QuotesState>,
    q: QuotesQuery,
) -> Result<Json<QuoteResponse>, ApplicationError> {
    let symbols_raw = q
        .symbols
        .ok_or_else(|| ApplicationError::MissingQueryParameters(vec!["q".to_owned()]))?;

    let symbols = symbols_raw
        .split(',')
        .map(str::trim)
        .map(str::to_owned)
        .filter(|s| !s.is_empty())
        .collect();

    let auth = cookies
        .get_private(AUTH_COOKIE_NAME)
        .ok_or(ApplicationError::MissingAuthentication)?;

    let mut credentials =
        Credentials::decode(auth.value()).map_err(ApplicationError::InvalidCredentials)?;

    if credentials.is_expired() {
        credentials
            .refresh_access_token(&oauth2)
            .await
            .map_err(ApplicationError::InvalidCredentials)?;
    }

    qm.set_credentials(credentials).await;

    qm.extend_quotes(symbols).await;

    let mut subscription = qm.subscribe().await;

    // in the loop, check if credentials are expired BEFORE recieving.
    let response = subscription
        .recv()
        .await
        .map_err(ApplicationError::ChannelBroadcastFailed)?
        .map_err(ApplicationError::Polling)?;

    Ok(Json::from(response))
}

fn ws_err(e: ApplicationError) -> Message {
    Message::Text(format!(r#"{{"error":{{"{e:?}":"{e}"}}}}"#))
}

fn ws_subscribed() -> Message {
    Message::Text(serde_json::to_string(&WsMsg::Subscribed).unwrap())
}

fn ws_ok(client_msg: ClientMsg) -> Message {
    Message::Text(serde_json::to_string(&WsMsg::Ok(client_msg)).unwrap())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde", tag = "type", rename_all = "snake_case")]
enum ClientMsg {
    Add { symbols: Vec<String> },
    Remove { symbols: Vec<String> },
    Subscribe { symbols: Vec<String> },
    Ping(Vec<u8>),
}

#[derive(Debug, Serialize)]
#[serde(tag = "event", rename_all = "lowercase")]
enum WsMsg {
    Subscribed,
    Ok(ClientMsg),
    Quote { data: QuoteResponse }, // or your real quote type
    // Error { error: BTreeMap<String, String> },
}

#[get("/quotes/stream")]
pub async fn quotes_stream<'a, 'b: 'a>(
    oauth2: OAuth2<Schwab>,
    cookies: &'b CookieJar<'a>,
    qm: &'a State<QuotesState>,
    ws: WebSocket,
) -> ws::Channel<'a> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let Some(auth) = cookies.get_private(AUTH_COOKIE_NAME) else {
                let _ = stream.send(ws_err(ApplicationError::MissingAuthentication)).await;
                return Ok(());
            };

            let mut credentials = Credentials::decode(auth.value()).unwrap();

            if credentials.is_expired() {
                if let Err(e) = credentials.refresh_access_token(&oauth2).await {
                    let _ = stream.send(ws_err(ApplicationError::InvalidCredentials(e))).await;
                    return Ok(());
                }
            }

            qm.set_credentials(credentials.clone()).await;

            let mut subscription = qm.subscribe().await;

            let mut tickers = HashSet::new();

            let _ = stream.send(ws_subscribed()).await;
            
            loop {
                select! {
                    incoming = stream.next() => {
                        let Some(incoming) = incoming else {
                            // client disconnected
                            break;
                        };

                        match incoming {
                            Ok(Message::Text(txt)) => match serde_json::from_str::<ClientMsg>(&txt).map(|message| (ws_ok(message.clone()), message)) {
                                Ok((_, ClientMsg::Ping(pong))) => {
                                    let _ = stream.send(Message::Pong(pong)).await;
                                }
                                Ok((m, ClientMsg::Add { symbols })) => {
                                    tickers.extend(symbols);
                                    let _ = stream.send(m).await;
                                }
                                Ok((m, ClientMsg::Remove { symbols })) => {
                                    for e in &symbols {
                                        tickers.remove(e);
                                    }
                                    let _ = stream.send(m).await;
                                }
                                Ok((m, ClientMsg::Subscribe { symbols })) => {
                                    tickers.clear();
                                    tickers.extend(symbols);
                                    let _ = stream.send(m).await;
                                }
                                Err(_) => {
                                    let _ = stream.send(ws_err(ApplicationError::InvalidWebSocketPayload)).await;
                                }
                            }
                            Ok(Message::Close(_)) => {
                                break;
                            },
                            Ok(_) => {}
                            Err(e) => {
                                // transport error
                                // typically occurs when server is shutting down
                                eprintln!("ws recv error: {e}");
                                break;
                            }
                        }

                        qm.set_quotes(tickers.iter().cloned().collect()).await;
                    }

                    msg = subscription.recv() => {
                        let Ok(message) = msg else {
                            break;
                        };

                        if credentials.is_expired() {
                            credentials.refresh_access_token(&oauth2).await.unwrap();
                        }

                        let response = message.map_err(ApplicationError::Polling);

                        let value = match response {
                            Ok(message) => serde_json::to_string(&WsMsg::Quote { data: message }).expect("Ok message should be serializable"),
                            Err(message) => serde_json::to_string(&message).expect("Err message should be serializable"),
                        };

                        let _ = stream
                            .send(Message::Text(value))
                            .await;

                        qm.set_quotes(tickers.iter().cloned().collect()).await;
                    }
                }
            }

            Ok(())
        })
    })
}
