#[macro_use]
extern crate rocket;

use crate::init::init_simulation;
use age_of_dragons_core::data::character::CharacterId;
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

#[get("/character/<id>")]
fn character(data: &State<SimulationData>, id: usize) -> Option<Template> {
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| {
            Template::render(
                "character",
                context! {
                    name: character.name().name(),
                    id: id,
                    race: "dfcds",
                    gender: format!("{:?}", character.gender()),
                    birth_date: character.birth_date().year(),
                    age: character.calculate_age(data.date).year(),
                },
            )
        })
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(init_simulation())
        .mount("/", routes![home, characters, character])
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}
