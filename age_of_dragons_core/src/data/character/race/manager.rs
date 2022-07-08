use crate::data::character::race::stage::LifeStage;
use crate::data::character::race::{Race, RaceId};
use crate::data::character::Character;
use anyhow::Result;

/// Stores all the [`Race`]s.
#[derive(Default, Debug)]
pub struct RaceMgr {
    races: Vec<Race>,
}

impl RaceMgr {
    /// Uses the function *f* to create a [`Race`] with the next [`RaceId`].
    pub fn create<F>(&mut self, f: F) -> Result<RaceId>
    where
        F: FnOnce(RaceId) -> Result<Race>,
    {
        let id = RaceId::new(self.races.len());
        self.races.push(f(id)?);
        Ok(id)
    }

    pub fn get_all(&self) -> &Vec<Race> {
        &self.races
    }

    pub fn get(&self, id: RaceId) -> Option<&Race> {
        self.races.get(id.0)
    }

    /// Returns the [`LifeStage`] of the [`Character`].
    ///
    /// # Panic
    ///
    /// The function panics if the character's [`Race`] or [`LifeStage`] don't exist.
    pub fn get_life_stage(&self, character: &Character) -> &LifeStage {
        self.get(character.race_id)
            .expect("Character has invalid race!")
            .stages
            .get(character.life_stage.index())
            .expect("Character has invalid life stage!")
    }
}
