use crate::data::character::CharacterId;

pub mod manager;

/// The type of a [`relationship`](CharacterRelation) between 2 [`Characters`](crate::data::character::Character).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CharacterRelationType {
    Mate,
}

/// A relationship between 2 [`Characters`](crate::data::character::Character).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CharacterRelation {
    id: CharacterId,
    relation_type: CharacterRelationType,
}

impl CharacterRelation {
    pub fn target(&self) -> CharacterId {
        self.id
    }

    pub fn relation_type(&self) -> CharacterRelationType {
        self.relation_type
    }
}
