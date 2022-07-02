use crate::data::character::relation::{CharacterRelation, CharacterRelationType};
use crate::data::character::CharacterId;

/// Stores the [`relations`](CharacterRelation) between all [`Characters`](crate::data::character::Character).
#[derive(Default, Debug)]
pub struct CharacterRelationMgr {
    relations: Vec<Vec<CharacterRelation>>,
    no_relations: Vec<CharacterRelation>,
}

impl CharacterRelationMgr {
    /// Returns all the [`relations`](CharacterRelation) between 2 [`Characters`](crate::data::character::Character).
    pub fn add_relation_between(
        &mut self,
        id0: CharacterId,
        id1: CharacterId,
        relation_type: CharacterRelationType,
    ) {
        self.check_size(id0, id1);
        self.add_relation(id0, id1, relation_type);
        self.add_relation(id1, id0, relation_type);
    }

    fn add_relation(
        &mut self,
        id0: CharacterId,
        id1: CharacterId,
        relation_type: CharacterRelationType,
    ) {
        if let Some(relations) = self.relations.get_mut(id0.id()) {
            relations.push(CharacterRelation {
                id: id1,
                relation_type,
            })
        }
    }

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

    /// Makes sure that the vector of relations is large enough to contain both characters.
    fn check_size(&mut self, id0: CharacterId, id1: CharacterId) {
        let min_size = id0.0.max(id1.0);

        if self.relations.len() <= min_size {
            let missing_characters = (min_size - self.relations.len()) + 1;

            for _i in 0..missing_characters {
                self.relations.push(Vec::new());
            }
        }
    }
}
