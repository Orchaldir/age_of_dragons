#[macro_use]
extern crate rocket;

use anyhow::Result;
use rocket::{routes, State};
use rocket_dyn_templates::{context, Template};

struct ViewerData {}

#[get("/")]
fn home(data: &State<ViewerData>) -> Template {
    Template::render(
        "home",
        context! {
            characters: 6,
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(ViewerData {})
        .mount("/", routes![home])
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}
