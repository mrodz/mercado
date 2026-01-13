use std::fmt::Debug;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chrono::{DateTime, TimeDelta, Utc};
use rocket::{
    fairing::Fairing,
    http::{Cookie, CookieJar, SameSite},
    response::Redirect,
};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) static AUTH_COOKIE_NAME: &str = "__Host-auth";

pub struct Schwab;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsPackaged {
    credentials: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credentials {
    access_token: String,
    refresh_token: String,
    /// seconds
    expires_in: i64,
    /// serialized in seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    minted: DateTime<Utc>,
}

impl Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn censor(s: &str) -> String {
            let mut chars = s.chars();

            let prefix: String = chars.by_ref().take(4).collect();
            let rest_len = chars.count();

            if rest_len == 0 {
                prefix
            } else {
                format!("{prefix}{}", "*".repeat(rest_len))
            }
        }

        f.debug_struct("Credentials")
            .field("access_token", &censor(&self.access_token))
            .field("refresh_token", &censor(&self.refresh_token))
            .field("expires_in", &self.expires_in)
            .field("minted", &self.minted)
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("could not decode credential: {0}")]
    Decode(#[from] base64::DecodeError),
    #[error("encountered invalid UTF 8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("invalid JSON for Credentials: {0}")]
    SerDe(#[from] serde_json::Error),
    #[error("error in oauth flow: {0}")]
    Oauth2(#[from] rocket_oauth2::Error),
}

impl From<TokenResponse<Schwab>> for Credentials {
    fn from(value: TokenResponse<Schwab>) -> Self {
        Self::from(&value)
    }
}

impl From<&TokenResponse<Schwab>> for Credentials {
    fn from(value: &TokenResponse<Schwab>) -> Self {
        let now = Utc::now();

        let access_token = value.access_token().to_owned();
        let refresh_token = value
            .refresh_token()
            .expect("TokenResponse should have refresh_token")
            .to_owned();
        let expires_in = value
            .expires_in()
            .expect("TokenResponse should have expires_in")
            .to_owned();

        Self::new(access_token, refresh_token, expires_in, now)
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
    pub fn new(
        access_token: String,
        refresh_token: String,
        expires_in: i64,
        minted: DateTime<Utc>,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            expires_in,
            minted,
        }
    }

    pub fn access_token(&self) -> Option<&str> {
        if self.is_expired() {
            None
        } else {
            Some(&self.access_token)
        }
    }

    pub async fn refresh_access_token(
        &mut self,
        oauth2: &OAuth2<Schwab>,
    ) -> Result<(), CredentialsError> {
        let token_response = oauth2.refresh(&self.refresh_token).await?;
        let as_credentials = Credentials::from(token_response);

        self.minted = Utc::now();
        self.access_token = as_credentials
            .access_token()
            .expect("fresh credentials should not be expired")
            .to_owned();
        self.refresh_token = as_credentials.refresh_token().to_owned();
        self.expires_in = as_credentials.expires_in;

        Ok(())
    }

    pub async fn ensure_access_token(
        &mut self,
        oauth2: &OAuth2<Schwab>,
    ) -> Result<String, CredentialsError> {
        if let Some(access_token) = self.access_token() {
            Ok(access_token.to_owned())
        } else {
            self.refresh_access_token(oauth2).await?;
            Ok(self
                .access_token()
                .expect("fresh credentials should not be expired")
                .to_owned())
        }
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn expires_in(&self) -> TimeDelta {
        let expiry = self.minted + TimeDelta::seconds(self.expires_in);
        expiry - Utc::now()
    }

    pub fn is_expired_skewed(&self, delta: TimeDelta) -> bool {
        self.expires_in() <= delta
    }

    pub fn is_expired(&self) -> bool {
        self.is_expired_skewed(TimeDelta::minutes(5))
    }

    pub fn encode(credentials: &Self) -> String {
        let obj = serde_json::to_string(credentials).expect("serializing should never fail");
        BASE64_URL_SAFE_NO_PAD.encode(&obj)
    }

    pub fn decode(encoded: &str) -> Result<Self, CredentialsError> {
        let decoded = String::from_utf8(BASE64_URL_SAFE_NO_PAD.decode(encoded)?)?;
        let result = serde_json::from_str::<Self>(&decoded)?;

        Ok(result)
    }
}

#[get("/login/schwab")]
pub fn schwab_login(oauth2: OAuth2<Schwab>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["api"]).unwrap()
}

#[get("/auth/schwab")]
pub fn schwab_callback(token: TokenResponse<Schwab>, cookies: &CookieJar<'_>) -> Redirect {
    let credentials = Credentials::from(&token);
    let encoded = Credentials::encode(&credentials);

    let cookie = Cookie::build((AUTH_COOKIE_NAME, Credentials::encode(&credentials)))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .build();

    cookies.add_private(cookie);

    Redirect::to(format!("/debug/authenticated#{encoded}"))
}

pub fn fairing() -> impl Fairing {
    OAuth2::<Schwab>::fairing("schwab")
}
