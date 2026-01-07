use std::{io, path::{Path, PathBuf}};

use rocket::{fs::NamedFile, response::Redirect};
use rocket_dyn_templates::Template;

mod oauth;
mod pages;
mod schwab;

#[macro_use] extern crate rocket;

const BUILD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/build");

#[get("/")]
fn index() -> Redirect {
    Redirect::permanent("/index.html")
}

#[get("/<file..>")]
async fn build_dir(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new(BUILD_DIR).join(file)).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/u", routes![oauth::schwab_login, oauth::schwab_callback, schwab::user])
        .mount("/debug", routes![pages::login_page, pages::index_page, pages::authenticated_page])
        .mount("/", routes![index, build_dir])
        .attach(oauth::fairing())
        .attach(Template::fairing())
}