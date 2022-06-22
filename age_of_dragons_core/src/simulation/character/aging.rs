use crate::data::character::CharacterId;
use crate::data::SimulationData;

enum AgingEffect {
    /// The [`Characters`](crate::data::character::Character) is old enough for
    /// the next [`LifeStage`](crate::data::character::race::stage::LifeStage).
    ChangeLifeStage(CharacterId, usize),
    /// The [`Characters`](crate::data::character::Character) is old enough to die from old age.
    Death(CharacterId),
}

/// Calculates which [`Characters`](crate::data::character::Character) are effected by aging.
fn calculate_aging_effects(data: &SimulationData) -> Vec<AgingEffect> {
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
                    let is_last_stage = new_life_stage == race.stages().len();

                    if is_last_stage {
                        return Some(AgingEffect::Death(character.id()));
                    } else {
                        return Some(AgingEffect::ChangeLifeStage(character.id(), new_life_stage));
                    }
                }
            }

            None
        })
        .collect()
}
