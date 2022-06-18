#[macro_use]
extern crate rocket;

use crate::init::init_simulation;
use age_of_dragons_core::data::character::race::RaceId;
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
        .map(|c| (c.id().id(), c.name().to_str()))
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
        .and_then(|character| {
            data.race_manager
                .get(character.race_id())
                .map(|race| (character, race))
        })
        .map(|(character, race)| {
            Template::render(
                "character",
                context! {
                    name: character.name().to_str(),
                    id: id,
                    race: race.name().to_str(),
                    race_id: race.id().id(),
                    gender: format!("{:?}", character.gender()),
                    birth_date: character.birth_date().year(),
                    age: character.calculate_age(data.date).year(),
                },
            )
        })
}

#[get("/race")]
fn races(data: &State<SimulationData>) -> Template {
    let races: Vec<(usize, &str)> = data
        .race_manager
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name().to_str()))
        .collect();

    Template::render(
        "races",
        context! {
            number: races.len(),
            races: races,
        },
    )
}

#[get("/race/<id>")]
fn race(data: &State<SimulationData>, id: usize) -> Option<Template> {
    data.race_manager.get(RaceId::new(id)).map(|race| {
        Template::render(
            "race",
            context! {
                name: race.name().to_str(),
                id: id,
                gender: format!("{:?}", race.gender_option()),
            },
        )
    })
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(init_simulation())
        .mount("/", routes![home, characters, character, races, race])
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}
