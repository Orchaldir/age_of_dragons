use crate::data::character::race::stage::LifeStageId;
use crate::data::character::{Character, CharacterId};
use crate::data::SimulationData;

/// Simulates the aging of [`Characters`](Character). They can grow up and die of old age.
pub fn simulate_aging(data: &mut SimulationData) {
    for effect in calculate_aging_effects(data) {
        match effect {
            AgingEffect::ChangeLifeStage(id, stage) => {
                let character = data
                    .character_manager
                    .get_mut(id)
                    .expect("Character growing up doesn't exist!");
                character.set_life_stage(stage);
            }
            AgingEffect::Death(id) => {
                let character = data
                    .character_manager
                    .get_mut(id)
                    .expect("Character dying from old age doesn't exist!");
                character.set_death_date(data.date);
            }
        }
    }
}

enum AgingEffect {
    /// The [`Characters`](Character) is old enough for
    /// the next [`LifeStage`](crate::data::character::race::stage::LifeStage).
    ChangeLifeStage(CharacterId, LifeStageId),
    /// The [`Characters`](crate::data::character::Character) is old enough to die from old age.
    Death(CharacterId),
}

/// Calculates which [`Characters`](Character) are effected by aging.
fn calculate_aging_effects(data: &SimulationData) -> Vec<AgingEffect> {
    data.character_manager
        .get_all()
        .iter()
        .filter_map(|character| calculate_aging_effect(data, character))
        .collect()
}

/// Calculates if a [`Character`] is effected by aging.
fn calculate_aging_effect(data: &SimulationData, character: &Character) -> Option<AgingEffect> {
    if character.is_dead() {
        return None;
    }

    let race = data
        .race_manager
        .get(character.race_id())
        .expect("Character's race is unknown!");

    let stage = race
        .stages()
        .get(character.life_stage().index())
        .expect("Character's life stage is unknown!");

    if let Some(max_age) = stage.max_age() {
        let age = character.calculate_age(data.date);

        if age > *max_age {
            let new_life_stage = character.life_stage().next();
            let is_last_stage = new_life_stage.index() == race.stages().len();

            return if is_last_stage {
                Some(AgingEffect::Death(character.id()))
            } else {
                Some(AgingEffect::ChangeLifeStage(character.id(), new_life_stage))
            };
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::character::gender::Gender::Female;
    use crate::data::character::race::tests::{create_immortal_race, create_mortal_race};
    use crate::data::time::Duration;

    #[test]
    fn test_mortal_race() {
        let mut data = SimulationData::default();
        let race_id = data
            .race_manager
            .create(|id| create_mortal_race(id, 1, 3))
            .unwrap();
        let id = data
            .create_character(|id, date| {
                Ok(Character::simple(id.id(), race_id, Female, date, None))
            })
            .unwrap();

        simulate_aging(&mut data);

        // 1.life stage

        assert_aging(&data, id, 0, true, 0);

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&data, id, 1, true, 0);

        // 2.life stage

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&data, id, 2, true, 1);

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&data, id, 3, true, 1);

        // character died

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&data, id, 4, false, 1);

        // dead characters don't age

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&data, id, 4, false, 1);
    }

    #[test]
    fn test_immortal_race_never_dies() {
        let mut data = SimulationData::default();
        let race_id = data
            .race_manager
            .create(|id| create_immortal_race(id))
            .unwrap();
        let id = data
            .create_character(|id, date| {
                Ok(Character::simple(id.id(), race_id, Female, date, None))
            })
            .unwrap();

        for i in 0..100 {
            simulate_aging(&mut data);

            assert_aging(&data, id, i, true, 0);

            data.date.increase_year();
        }
    }

    fn assert_aging(
        data: &SimulationData,
        id: CharacterId,
        age: u32,
        is_alive: bool,
        life_stage: usize,
    ) {
        let character = data.character_manager.get(id).unwrap();
        assert_eq!(character.calculate_age(data.date), Duration::new(age));
        assert_eq!(character.is_alive(), is_alive);
        assert_eq!(character.life_stage(), LifeStageId::new(life_stage));
    }
}
