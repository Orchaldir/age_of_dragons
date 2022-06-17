use age_of_dragons_core::data::character::gender::Gender;
use age_of_dragons_core::data::character::gender::Gender::{Female, Male};
use age_of_dragons_core::data::character::manager::CharacterMgr;
use age_of_dragons_core::data::character::race::gender::GenderOption;
use age_of_dragons_core::data::character::race::manager::RaceMgr;
use age_of_dragons_core::data::character::race::stage::LifeStage;
use age_of_dragons_core::data::character::race::{Race, RaceId};
use age_of_dragons_core::data::character::{Character, CharacterId};
use age_of_dragons_core::data::time::{Date, Duration};
use age_of_dragons_core::data::SimulationData;
use anyhow::Result;

pub fn init_simulation() -> SimulationData {
    let race_manager = init_races();
    let dragon = race_manager.get(RaceId::new(0)).unwrap();
    let character_manager = init_characters(dragon);

    SimulationData {
        race_manager,
        character_manager,
        date: Date::new(0),
    }
}

fn init_races() -> RaceMgr {
    let mut manager = RaceMgr::default();

    manager.create(init_dragon).unwrap();

    manager
}

fn init_dragon(id: RaceId) -> Result<Race> {
    let stage0 = LifeStage::new("Wyrmling", 0, Some(Duration::new(4))).unwrap();
    let stage1 = LifeStage::new("Young", 1, Some(Duration::new(99))).unwrap();
    let stage2 = LifeStage::new("Adult", 2, Some(Duration::new(799))).unwrap();
    let stage3 = LifeStage::new("Ancient", 3, Some(Duration::new(999))).unwrap();
    let stage4 = LifeStage::new("Wyrm", 4, None).unwrap();
    let stages = vec![stage0, stage1, stage2, stage3, stage4];
    Race::new(id.id(), "Dragon", GenderOption::TwoGenders, stages)
}

fn init_characters(race: &Race) -> CharacterMgr {
    let mut manager = CharacterMgr::default();

    manager
        .create(|id| init_character(id, race, "D0", Male))
        .unwrap();
    manager
        .create(|id| init_character(id, race, "D1", Female))
        .unwrap();

    manager
}

fn init_character(id: CharacterId, race: &Race, name: &str, gender: Gender) -> Result<Character> {
    Character::new(id.id(), name, race, gender, Date::new(0), None)
}
