use crate::data::character::gender::Gender;
use crate::data::character::gender::Gender::{Female, Male};
use crate::data::character::relation::CharacterRelationType;
use crate::data::character::{Character, CharacterId};
use crate::data::SimulationData;
use CharacterRelationType::Mate;

/// Simulates [`characters`](Character) finding mates.
pub fn simulate_finding_mate(data: &mut SimulationData) {
    for (female, male) in calculate_new_mates(data) {
        data.relation_manager
            .add_relation_between(female, male, Mate);
    }
}

fn calculate_new_mates(data: &SimulationData) -> Vec<(CharacterId, CharacterId)> {
    data.character_manager
        .get_all()
        .iter()
        .filter(|character| is_valid_mate(data, character, Female))
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

fn is_valid_mate(data: &SimulationData, character: &Character, gender: Gender) -> bool {
    character.is_alive()
        && character.gender() == gender
        && data
            .race_manager
            .get_life_stage(character)
            .reproduction()
            .is_some()
}

fn is_valid_match(data: &SimulationData, character: &Character, candidate: &Character) -> bool {
    character.race_id() == candidate.race_id() && is_valid_mate(data, candidate, Male)
}
