use rocket_dyn_templates::{Template, context};

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("login", context! {})
}
