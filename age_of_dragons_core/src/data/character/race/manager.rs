use crate::data::character::race::{Race, RaceId};
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
}
