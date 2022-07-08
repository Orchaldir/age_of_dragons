use crate::data::character::gender::Gender;
use crate::data::character::manager::CharacterMgr;
use crate::data::character::race::manager::RaceMgr;
use crate::data::character::race::RaceId;
use crate::data::character::relation::manager::CharacterRelationMgr;
use crate::data::character::{Character, CharacterId};
use crate::data::time::Date;
use anyhow::Result;

pub mod character;
pub mod name;
pub mod probability;
pub mod time;

#[derive(Default, Debug)]
pub struct SimulationData {
    pub race_manager: RaceMgr,
    pub character_manager: CharacterMgr,
    pub relation_manager: CharacterRelationMgr,
    pub date: Date,
}

impl SimulationData {
    /// Creates a [`Character`] with the next [`CharacterId`] and the current date as birthdate.
    pub fn create_character<S: Into<String>>(
        &mut self,
        name: S,
        race_id: RaceId,
        gender: Gender,
    ) -> Result<CharacterId> {
        let race = self
            .race_manager
            .get(race_id)
            .expect("Cannot create character with unknown race!");
        self.character_manager
            .create(|id| Character::new(id.id(), name, race, gender, self.date, None))
    }
}
