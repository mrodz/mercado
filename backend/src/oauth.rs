use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rocket::{fairing::Fairing, http::CookieJar, response::Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct Schwab;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsPackaged {
    credentials: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("could not decode credential")]
    DecodeError(#[from] base64::DecodeError),
    #[error("encountered invalid UTF 8")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("invalid JSON for Credentials")]
    SerDeError(#[from] serde_json::Error),
}

impl From<TokenResponse<Schwab>> for Credentials {
    fn from(value: TokenResponse<Schwab>) -> Self {
        Self::from(&value)
    }
}

impl From<&TokenResponse<Schwab>> for Credentials {
    fn from(value: &TokenResponse<Schwab>) -> Self {
        let access_token = value.access_token().to_owned();
        let refresh_token = value.refresh_token().expect("TokenResponse should have refresh_token").to_owned();
        let expires_in = value.expires_in().expect("TokenResponse should have expires_in").to_owned();

        Self::new(access_token, refresh_token, expires_in)
    }
}

impl TryFrom<CredentialsPackaged> for Credentials {
    type Error = CredentialsError;

    fn try_from(value: CredentialsPackaged) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&CredentialsPackaged> for Credentials {
    type Error = CredentialsError;

    fn try_from(value: &CredentialsPackaged) -> Result<Self, Self::Error> {
        Credentials::decode(&value.credentials)
    }
}

impl Credentials {
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            refresh_token,
            expires_in,
        }
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn expires_in(&self) -> i64 {
        self.expires_in
    }

    pub fn encode(credentials: &Self) -> String {
        let obj = serde_json::to_string(credentials).expect("serializing should never fail");
        let encoded = BASE64_URL_SAFE_NO_PAD.encode(&obj);

        encoded
    }

    pub fn decode(encoded: &str) -> Result<Self, CredentialsError> {
        let decoded= String::from_utf8(BASE64_URL_SAFE_NO_PAD.decode(encoded)?)?;
        let result = serde_json::from_str::<Self>(&decoded)?;

        Ok(result)
    }
}


#[get("/login/schwab")]
pub fn schwab_login(oauth2: OAuth2<Schwab>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["api"]).unwrap()
}

#[get("/auth/schwab")]
pub fn schwab_callback(token: TokenResponse<Schwab>) -> Redirect
{
    let credentials = Credentials::from(&token);
    let encoded = Credentials::encode(&credentials);

    Redirect::to(format!("/authenticated#{encoded}"))
}

pub fn fairing() -> impl Fairing {
    OAuth2::<Schwab>::fairing("schwab")
}
