use crate::data::character::manager::CharacterMgr;
use crate::data::character::race::manager::RaceMgr;
use crate::data::time::Date;

pub mod character;
pub mod name;
pub mod time;

#[derive(Default, Debug)]
pub struct SimulationData {
    pub race_manager: RaceMgr,
    pub character_manager: CharacterMgr,
    pub date: Date,
}
