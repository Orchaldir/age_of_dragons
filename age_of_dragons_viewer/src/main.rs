#[macro_use]
extern crate rocket;

use crate::init::init_simulation;
use age_of_dragons_core::data::character::race::RaceId;
use age_of_dragons_core::data::character::CharacterId;
use age_of_dragons_core::data::SimulationData;
use age_of_dragons_core::simulation::simulate_year;
use anyhow::Result;
use rocket::response::Redirect;
use rocket::{routes, State};
use rocket_dyn_templates::{context, Template};
use std::sync::Mutex;

pub mod init;

struct ViewerData {
    data: Mutex<SimulationData>,
}

#[get("/")]
fn home(data: &State<ViewerData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    Template::render(
        "home",
        context! {
            races: data.race_manager.get_all().len(),
            characters: data.character_manager.get_all().len(),
            year: data.date.year(),
        },
    )
}

#[get("/simulate")]
fn simulate(data: &State<ViewerData>) -> Redirect {
    let mut data = data.data.lock().expect("lock shared data");
    simulate_year(&mut data);
    Redirect::to(uri!(home()))
}

#[get("/character")]
fn characters(data: &State<ViewerData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
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
fn character(data: &State<ViewerData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .and_then(|character| {
            data.race_manager
                .get(character.race_id())
                .map(|race| (character, race))
        })
        .map(|(character, race)| {
            let stage = race.stages()[character.life_stage().index()]
                .name()
                .to_str();
            Template::render(
                "character",
                context! {
                    name: character.name().to_str(),
                    id: id,
                    race: race.name().to_str(),
                    race_id: race.id().id(),
                    stage: stage,
                    gender: format!("{:?}", character.gender()),
                    birth_date: character.birth_date().year(),
                    age: character.calculate_age(data.date).year(),
                    relations: visualize_relations(&data, character.id()),
                },
            )
        })
}

fn visualize_relations(data: &SimulationData, id: CharacterId) -> Vec<(usize, &str, String)> {
    data.relation_manager
        .get_relations_of(id)
        .iter()
        .map(|relation| {
            (
                relation.target().id(),
                data.character_manager
                    .get(relation.target())
                    .map(|other| other.name().to_str())
                    .unwrap_or("Unknown"),
                format!("{:?}", relation.relation_type()),
            )
        })
        .collect()
}

#[get("/race")]
fn races(data: &State<ViewerData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
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
fn race(data: &State<ViewerData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.race_manager.get(RaceId::new(id)).map(|race| {
        let stages: Vec<&str> = race
            .stages()
            .iter()
            .map(|stage| stage.name().to_str())
            .collect();
        Template::render(
            "race",
            context! {
                name: race.name().to_str(),
                id: id,
                gender: format!("{:?}", race.gender_option()),
                stages: stages,
            },
        )
    })
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(ViewerData {
            data: Mutex::new(init_simulation()),
        })
        .mount(
            "/",
            routes![home, simulate, characters, character, races, race],
        )
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}
