use std::path::Path;

use rocket::{fs::FileServer, response::content::RawHtml};
use rocket_dyn_templates::Template;

use crate::quotes::QuotesState;

mod errors;
mod oauth;
mod pages;
mod quotes;
mod schwab;

#[macro_use]
extern crate rocket;

const BUILD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/build");

/// See
#[get("/<_..>", rank = 10)]
async fn spa_fallback() -> Option<RawHtml<String>> {
    let index_path = Path::new(BUILD_DIR).join("index.html");
    let html = rocket::tokio::fs::read_to_string(index_path).await.ok()?;
    Some(RawHtml(html))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/u",
            routes![
                oauth::schwab_login,
                oauth::schwab_callback,
                schwab::endpoints::user,
                schwab::endpoints::quotes_stream,
                schwab::endpoints::quotes
            ],
        )
        .mount(
            "/debug",
            routes![
                pages::login_page,
                pages::index_page,
                pages::authenticated_page,
                schwab::endpoints::refresh_token_debug
            ],
        )
        .mount("/", FileServer::from(BUILD_DIR).rank(9))
        .mount("/", routes![spa_fallback])
        .attach(oauth::fairing())
        .attach(Template::fairing())
        .manage(QuotesState::new())
}
