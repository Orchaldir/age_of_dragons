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

#[get("/character")]
fn characters(data: &State<SimulationData>) -> Template {
    let total = data.character_manager.get_all().len();
    let alive = data
        .character_manager
        .get_all()
        .iter()
        .filter(|&c| c.is_alive())
        .count();
    let characters: Vec<(usize, &str)> = data
        .character_manager
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name().name()))
        .collect();

    Template::render(
        "characters",
        context! {
            alive: alive,
            total: total,
            characters: characters,
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(init_simulation())
        .mount("/", routes![home, characters])
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}
