use crate::data::character::CharacterId;
use crate::data::SimulationData;

/// Calculates which [`Character`](crate::data::character::Character) are old enough to change to
/// the next [`LifeStage`](crate::data::character::race::stage::LifeStage).
fn calculate_aging(data: &SimulationData) -> Vec<(CharacterId, usize)> {
    let mut aging = Vec::new();

    for character in data.character_manager.get_all() {
        if character.is_dead() {
            continue;
        }

        let race = data
            .race_manager
            .get(character.race_id())
            .expect("Characters race is unknown!");

        if let Some(stage) = race.stages().get(character.life_stage()) {
            if let Some(max_age) = stage.max_age() {
                let age = character.calculate_age(data.date);

                if age > *max_age {
                    let new_life_stage = character.life_stage() + 1;
                    aging.push((character.id(), new_life_stage));
                }
            }
        }
    }

    aging
}
