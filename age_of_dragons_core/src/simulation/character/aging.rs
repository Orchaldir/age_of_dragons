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
    use crate::data::character::race::gender::GenderOption;
    use crate::data::character::race::stage::LifeStage;
    use crate::data::character::race::{Race, RaceId};
    use crate::data::time::{Date, Duration};
    use anyhow::Result;

    #[test]
    fn test_mortal_race() {
        let mut data = SimulationData::default();
        let race_id = data
            .race_manager
            .create(|id| create_mortal(id, 1, 3))
            .unwrap();
        let id = data
            .character_manager
            .create(|id| {
                Ok(Character::simple(
                    id.id(),
                    race_id,
                    Female,
                    Date::new(0),
                    None,
                ))
            })
            .unwrap();

        simulate_aging(&mut data);

        assert_aging(&mut data, id, true, 0);

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&mut data, id, true, 0);

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&mut data, id, true, 1);

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&mut data, id, true, 1);

        data.date.increase_year();
        simulate_aging(&mut data);

        assert_aging(&mut data, id, false, 1);
    }

    fn assert_aging(data: &SimulationData, id: CharacterId, is_alive: bool, life_stage: usize) {
        let character = data.character_manager.get(id).unwrap();
        assert_eq!(character.is_alive(), is_alive);
        assert_eq!(character.life_stage(), LifeStageId::new(life_stage));
    }

    fn create_mortal(id: RaceId, age0: u32, age1: u32) -> Result<Race> {
        let stage0 = LifeStage::new("Child", 0, Some(Duration::new(age0))).unwrap();
        let stage1 = LifeStage::new("Adult", 1, Some(Duration::new(age1))).unwrap();
        let stages = vec![stage0, stage1];
        Race::new(id.id(), "Mortal Race", GenderOption::TwoGenders, stages)
    }
}
