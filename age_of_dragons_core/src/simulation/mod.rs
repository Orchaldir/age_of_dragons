use crate::data::SimulationData;

/// Advances the world by a year and simulates everything that happened.
pub fn simulate_year(data: &mut SimulationData) {
    println!("Simulate year {}", data.date.year());

    data.date.increase_year();
}
