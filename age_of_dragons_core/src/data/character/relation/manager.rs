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
        self.check_size_for_both(id0, id1);
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

    /// Returns all [`relations`](CharacterRelation) between 2 [`Characters`](crate::data::character::Character).
    pub fn get_relations_between(
        &self,
        id0: CharacterId,
        id1: CharacterId,
    ) -> Vec<CharacterRelationType> {
        self.relations
            .get(id0.id())
            .map(|relations| {
                relations
                    .iter()
                    .filter(|relation| relation.id == id1)
                    .map(|relation| relation.relation_type)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns all [`relations`](CharacterRelation) of a [`Character`](crate::data::character::Character).
    pub fn get_relations_of(&self, id: CharacterId) -> &[CharacterRelation] {
        self.relations.get(id.id()).unwrap_or(&self.no_relations)
    }

    /// Has the [`character`](crate::data::character::Character) a [`relation`](CharacterRelation) of a specific [`type`](CharacterRelationType)?
    pub fn has_relation(&self, id: CharacterId, relation_type: CharacterRelationType) -> bool {
        self.relations
            .get(id.id())
            .filter(|relations| {
                relations
                    .iter()
                    .any(|relation| relation.relation_type == relation_type)
            })
            .is_some()
    }

    /// Makes sure that the vector of relations is large enough to contain both characters.
    fn check_size_for_both(&mut self, id0: CharacterId, id1: CharacterId) {
        self.check_size(id0.0.max(id1.0));
    }

    /// Makes sure that the vector of relations is large enough to contain the character.
    fn check_size(&mut self, min_size: usize) {
        if self.relations.len() <= min_size {
            let missing_characters = (min_size - self.relations.len()) + 1;

            for _i in 0..missing_characters {
                self.relations.push(Vec::new());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_relations_with_unknown_id() {
        let mut manager = CharacterRelationMgr::default();
        let id0 = CharacterId::new(0);
        let id1 = CharacterId::new(1);
        manager.check_size(0);

        assert!(manager.get_relations_between(id0, id1).is_empty());
        assert!(manager.get_relations_between(id1, id0).is_empty());

        assert!(manager.get_relations_of(id0).is_empty());
        assert!(manager.get_relations_of(id1).is_empty());
    }
}
