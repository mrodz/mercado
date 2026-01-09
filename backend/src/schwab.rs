use rocket::{http::CookieJar, serde::json::Json};
use rocket_oauth2::OAuth2;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::oauth::{AUTH_COOKIE_NAME, Credentials, Schwab};

const TRADER_API: &str = "https://api.schwabapi.com/trader/v1";

#[get("/user")]
pub async fn user(oauth2: OAuth2<Schwab>, cookies: &CookieJar<'_>) -> Json<SchwabUsers> {
    let auth = cookies.get_private(AUTH_COOKIE_NAME).expect("auth cookie not found");
    let mut credentials = Credentials::decode(auth.value()).unwrap();

    Json::from(SchwabUsers(get_user(&mut credentials, &oauth2).await.expect("could not get user")))
}

#[post("/refresh_token")]
pub async fn refresh_token_debug(oauth2: OAuth2<Schwab>, cookies: &CookieJar<'_>) -> Value {
    let auth = cookies.get_private(AUTH_COOKIE_NAME).expect("auth cookie not found");
    let mut credentials = Credentials::decode(auth.value()).unwrap();
    credentials.refresh_access_token(&oauth2).await.unwrap();
    serde_json::json!({
        "success": true,
    })
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchwabUsers(Vec<SchwabAccount>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchwabAccount {
    #[serde(rename(serialize = "accountNumber", deserialize = "accountNumber"))]
    account_number: String,
    #[serde(rename(serialize = "primaryAccount", deserialize = "primaryAccount"))]
    primary_account: bool,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    account_type: String,
    #[serde(rename(serialize = "nickName", deserialize = "nickName"))]
    nick_name: String,
    #[serde(rename(serialize = "displayAcctId", deserialize = "displayAcctId"))]
    display_id: String,
    #[serde(rename(serialize = "autoPositionEffect", deserialize = "autoPositionEffect"))]
    auto_position_effect: bool,
    #[serde(rename(serialize = "accountColor", deserialize = "accountColor"))]
    account_color: String,
    #[serde(rename(serialize = "lotSelectionMethod", deserialize = "lotSelectionMethod"))]
    lot_selection_method: String,
    #[serde(rename(serialize = "hasFuturesAccount", deserialize = "hasFuturesAccount"))]
    has_futures_account: bool,
    #[serde(rename(serialize = "hasForexAccount", deserialize = "hasForexAccount"))]
    has_forex_account: bool,
}

pub async fn get_user(credentials: &mut Credentials, oauth2: &OAuth2<Schwab>) -> Result<Vec<SchwabAccount>, reqwest::Error> {
    let client = reqwest::Client::new();

    let req = client.get(format!("{TRADER_API}/userPreference"))
        .header("Authorization", format!("Bearer {}", credentials.ensure_access_token(oauth2).await.unwrap()))
        .header("accept", "application/json");

    let response = req
        .send()
        .await?
        .text()
        .await?;

    let users: Value = serde_json::from_str(&response).expect("malformed JSON");
    let accounts = users.get("accounts").expect("accounts");
    assert!(accounts.is_array());

    let mut result = vec![];

    for account in accounts.as_array().unwrap() {
        let account = serde_json::from_value::<SchwabAccount>(account.clone()).expect("malformed account");
        result.push(account);
    }

    Ok(result)
}