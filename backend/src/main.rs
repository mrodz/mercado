use rocket_dyn_templates::Template;

mod oauth;
mod pages;
mod schwab;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/u", routes![oauth::schwab_login, oauth::schwab_callback, schwab::user])
        .mount("/", routes![pages::login_page, pages::index_page, pages::authenticated_page])
        .attach(oauth::fairing())
        .attach(Template::fairing())
}