use crate::data::character::CharacterId;
use crate::data::SimulationData;

/// Calculates which [`Character`](crate::data::character::Character) are old enough to change to
/// the next [`LifeStage`](crate::data::character::race::stage::LifeStage).
fn calculate_aging(data: &SimulationData) -> Vec<(CharacterId, usize)> {
    data.character_manager
        .get_all()
        .iter()
        .filter_map(|character| {
            if character.is_dead() {
                return None;
            }

            let race = data
                .race_manager
                .get(character.race_id())
                .expect("Character's race is unknown!");

            let stage = race
                .stages()
                .get(character.life_stage())
                .expect("Character's life stage is unknown!");

            if let Some(max_age) = stage.max_age() {
                let age = character.calculate_age(data.date);

                if age > *max_age {
                    let new_life_stage = character.life_stage() + 1;
                    return Some((character.id(), new_life_stage));
                }
            }

            None
        })
        .collect()
}
