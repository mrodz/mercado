use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index_page() -> Template {
    Template::render("index", context! {})
}
