use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rocket::{fairing::Fairing, http::{Cookie, CookieJar, SameSite}, response::Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};

pub struct Schwab;

#[get("/login/schwab")]
pub fn schwab_login(oauth2: OAuth2<Schwab>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["marketdata"]).unwrap()
}

#[get("/auth/schwab")]
pub fn schwab_callback(token: TokenResponse<Schwab>, cookies: &CookieJar<'_>) -> Redirect
{
    cookies.add_private(
        Cookie::build(("token", token.access_token().to_string()))
            .same_site(SameSite::Lax)
            .build()
    );

    let obj = serde_json::json!({
        "access_token": token.access_token(),
        "refresh_token": token.refresh_token(),
        "expires_in": token.expires_in(),
    }).to_string();


    let encoded = BASE64_URL_SAFE_NO_PAD.encode(&obj);

    Redirect::to(format!("/authenticated#{encoded}"))
}

pub fn fairing() -> impl Fairing {
    OAuth2::<Schwab>::fairing("schwab")
}
