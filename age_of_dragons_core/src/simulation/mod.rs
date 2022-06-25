use crate::data::SimulationData;
use crate::simulation::character::aging::simulate_aging;

pub mod character;

/// Advances the world by a year and simulates everything that happened.
pub fn simulate_year(data: &mut SimulationData) {
    println!("Simulate year {}", data.date.year());

    simulate_aging(data);

    data.date.increase_year();
}
