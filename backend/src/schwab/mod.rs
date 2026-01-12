pub mod endpoints;
pub mod schema;

use reqwest::Client;
use rocket_oauth2::OAuth2;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::ApplicationError,
    oauth::{Credentials, Schwab},
    schwab::schema::{QuoteResponse, SchwabAccount},
};

const TRADER_API: &str = "https://api.schwabapi.com/trader/v1";
const MARKET_DATA_API: &str = "https://api.schwabapi.com/marketdata/v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchwabUsers(Vec<SchwabAccount>);

pub async fn get_user(
    credentials: &mut Credentials,
    oauth2: &OAuth2<Schwab>,
) -> Result<Vec<SchwabAccount>, ApplicationError> {
    let client = Client::new();

    let req = client
        .get(format!("{TRADER_API}/userPreference"))
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                credentials
                    .ensure_access_token(oauth2)
                    .await
                    .map_err(ApplicationError::InvalidCredentials)?
            ),
        )
        .header("accept", "application/json");

    let response = req
        .send()
        .await
        .map_err(ApplicationError::Network)?
        .text()
        .await
        .map_err(ApplicationError::Network)?;

    let users: Value = serde_json::from_str(&response).map_err(ApplicationError::InvalidJson)?;
    let accounts = users
        .get("accounts")
        .ok_or_else(|| ApplicationError::InvalidJsonLookup {
            index: "accounts".to_owned(),
            object: users.clone(),
        })?;

    assert!(accounts.is_array());

    let mut result = vec![];

    let accounts_as_array = accounts.as_array().ok_or_else(|| ApplicationError::InvalidJsonLookup {
            index: "enforce<array>(accounts)".to_owned(),
            object: users.clone(),
        })?;

    for account in accounts_as_array {
        let account = serde_json::from_value::<SchwabAccount>(account.clone())
            .map_err(ApplicationError::SchwabAccountDeserialization)?;
        result.push(account);
    }

    Ok(result)
}

pub async fn get_quote(
    credentials: Credentials,
    client: Option<Client>,
    quotes: Vec<String>,
) -> Result<QuoteResponse, ApplicationError> {
    let client = client.unwrap_or_default();

    let symbols = quotes.join(",");

    let req = client.get(format!("{MARKET_DATA_API}/quotes?symbols={symbols}&fields=quote,fundamental,extended,reference,regular&indicative=false"))
        .header("Authorization", format!("Bearer {}", credentials.access_token().ok_or(ApplicationError::MissingAuthentication)?))
        .header("accept", "application/json");

    let response = req
        .send()
        .await
        .map_err(ApplicationError::Network)?
        .text()
        .await
        .map_err(ApplicationError::Network)?;

    dbg!(&response);

    let parsed = serde_json::from_str::<QuoteResponse>(&response)
        .map_err(ApplicationError::QuoteResponseDeserialization)?;

    Ok(parsed)
}
