use crate::data::character::manager::CharacterMgr;
use crate::data::character::race::manager::RaceMgr;
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
    pub date: Date,
}

impl SimulationData {
    /// Uses the function *f* to create a [`Character`] with the next [`CharacterId`] and the current date as birthdate.
    pub fn create_character<F>(&mut self, f: F) -> Result<CharacterId>
    where
        F: FnOnce(CharacterId, Date) -> Result<Character>,
    {
        self.character_manager.create(|id| f(id, self.date))
    }
}
