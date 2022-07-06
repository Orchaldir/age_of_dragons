use crate::data::character::relation::CharacterRelationType;
use crate::data::character::{Character, CharacterId};
use crate::data::SimulationData;
use CharacterRelationType::Mate;

/// Simulates [`characters`](Character) finding mates.
pub fn simulate_finding_mate(data: &mut SimulationData) {
    for (female, male) in calculate_new_mates(data) {
        println!("{:?} & {:?} become mates", female, male);
        data.relation_manager
            .add_relation_between(female, male, Mate);
    }
}

fn calculate_new_mates(data: &SimulationData) -> Vec<(CharacterId, CharacterId)> {
    data.character_manager
        .get_all()
        .iter()
        .filter(|character| can_become_mate(data, character))
        .filter_map(|character| {
            find_matching_character(data, character).map(|other| (character.id(), other))
        })
        .collect()
}

fn find_matching_character(data: &SimulationData, character: &Character) -> Option<CharacterId> {
    data.character_manager.get_all()[character.id().id()..]
        .iter()
        .find(|other| is_valid_match(data, character, other))
        .map(|other| other.id())
}

fn can_become_mate(data: &SimulationData, character: &Character) -> bool {
    character.is_alive()
        && data
            .race_manager
            .get_life_stage(character)
            .reproduction()
            .is_some()
        && !data.relation_manager.has_relation(character.id(), Mate)
}

fn is_valid_match(data: &SimulationData, character: &Character, candidate: &Character) -> bool {
    character.race_id() == candidate.race_id()
        && character.gender() != candidate.gender()
        && can_become_mate(data, candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::character::gender::Gender;
    use crate::data::character::gender::Gender::{Female, Male};
    use crate::data::character::race::tests::create_mortal_race;
    use crate::simulation::character::aging::simulate_aging;

    #[test]
    fn test_2_valid_characters_becoming_mates() {
        test(Female, Male, vec![Mate]);
        test(Male, Female, vec![Mate]);
    }

    #[test]
    fn test_same_gender_cant_become_mates() {
        test(Female, Female, vec![]);
        test(Male, Male, vec![]);
    }

    #[test]
    fn test_2_characters_cant_becoming_mates_multiple_times() {
        let mut data = SimulationData::default();
        let race_id = create_mortal_race(&mut data.race_manager, 1, 3);
        let id0 = data.create_character("C0", race_id, Female).unwrap();
        let id1 = data.create_character("C1", race_id, Male).unwrap();

        data.date.increase_year();
        data.date.increase_year();
        simulate_aging(&mut data);
        simulate_finding_mate(&mut data);

        assert_mate(&mut data, id0, id1, vec![Mate]);

        data.date.increase_year();
        simulate_finding_mate(&mut data);

        assert_mate(&mut data, id0, id1, vec![Mate]);
    }

    fn test(gender0: Gender, gender1: Gender, result: Vec<CharacterRelationType>) {
        let mut data = SimulationData::default();
        let race_id = create_mortal_race(&mut data.race_manager, 1, 3);
        let id0 = data.create_character("C0", race_id, gender0).unwrap();
        let id1 = data.create_character("C1", race_id, gender1).unwrap();

        // too young
        simulate_finding_mate(&mut data);

        assert_mate(&mut data, id0, id1, vec![]);

        // too young
        data.date.increase_year();
        simulate_aging(&mut data);
        simulate_finding_mate(&mut data);

        assert_mate(&mut data, id0, id1, vec![]);

        // correct age
        data.date.increase_year();
        simulate_aging(&mut data);
        simulate_finding_mate(&mut data);

        assert_mate(&mut data, id0, id1, result);
    }

    fn assert_mate(
        data: &SimulationData,
        id0: CharacterId,
        id1: CharacterId,
        result: Vec<CharacterRelationType>,
    ) {
        assert_eq!(
            result,
            data.relation_manager.get_relations_between(id0, id1)
        );
        assert_eq!(
            result,
            data.relation_manager.get_relations_between(id1, id0)
        );
    }
}
