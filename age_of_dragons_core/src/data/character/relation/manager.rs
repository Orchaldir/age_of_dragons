use crate::data::character::relation::CharacterRelation;
use crate::data::character::CharacterId;

/// Stores the [`relations`](CharacterRelation) between all [`Characters`](crate::data::character::Character).
#[derive(Default, Debug)]
pub struct CharacterRelationMgr {
    relations: Vec<Vec<CharacterRelation>>,
    no_relations: Vec<CharacterRelation>,
}

impl CharacterRelationMgr {
    /// Returns all the [`relations`](CharacterRelation) between 2 [`Characters`](crate::data::character::Character).
    pub fn get_relations_between(
        &self,
        id0: CharacterId,
        id1: CharacterId,
    ) -> Vec<&CharacterRelation> {
        self.relations
            .get(id0.id())
            .map(|relations| {
                relations
                    .iter()
                    .filter(|relation| relation.id == id1)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns all the [`relations`](CharacterRelation) of a [`Character`](crate::data::character::Character).
    pub fn get_relations_of(&self, id: CharacterId) -> &[CharacterRelation] {
        self.relations.get(id.id()).unwrap_or(&self.no_relations)
    }
}
