use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::oauth::{Credentials, CredentialsPackaged};

const TRADER_API: &str = "https://api.schwabapi.com/trader/v1";

#[post("/user", data = "<credentials>")]
pub async fn user(credentials: Json<CredentialsPackaged>) -> Json<SchwabUsers> {
    let credentials = credentials.into_inner();
    let credentials = Credentials::try_from(&credentials).expect("bad credentials");

    Json::from(SchwabUsers(get_user(credentials).await.expect("could not get user")))
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

pub async fn get_user(credentials: Credentials) -> Result<Vec<SchwabAccount>, reqwest::Error> {
    let client = reqwest::Client::new();

    let req = client.get(format!("{TRADER_API}/userPreference"))
        .header("Authorization", format!("Bearer {}", credentials.access_token()))
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