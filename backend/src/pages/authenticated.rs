use chrono::Utc;
use rocket_dyn_templates::{Template, context};

#[get("/authenticated")]
pub fn authenticated_page() -> Template {
    Template::render(
        "authenticated",
        context! {
            timestamp: Utc::now().to_string(),
        },
    )
}
