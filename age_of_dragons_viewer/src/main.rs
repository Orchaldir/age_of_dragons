#[macro_use]
extern crate rocket;

use crate::init::init_simulation;
use age_of_dragons_core::data::SimulationData;
use anyhow::Result;
use rocket::{routes, State};
use rocket_dyn_templates::{context, Template};

pub mod init;

#[get("/")]
fn home(data: &State<SimulationData>) -> Template {
    Template::render(
        "home",
        context! {
            races: data.race_manager.get_all().len(),
            characters: data.character_manager.get_all().len(),
            year: data.date.year(),
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(init_simulation())
        .mount("/", routes![home])
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}
