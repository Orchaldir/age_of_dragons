use crate::data::character::gender::Gender;
use crate::data::name::Name;

pub mod gender;

/// The id of a [`Character`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CharacterId(usize);

impl CharacterId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

/// A character is an entity important enough to be fully simulated.
#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
    name: Name,
    gender: Gender,
}

impl Character {
    /// Creates a character with a default [`Name`].
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::{Character, CharacterId};
    ///# use age_of_dragons_core::data::character::gender::Gender::*;
    /// let id = CharacterId::new(11);
    /// let character = Character::new(id, Female);
    ///
    /// assert_eq!(character.id(), id);
    /// assert_eq!(character.name().name(), "Character 11");
    /// assert_eq!(character.gender(), Female);
    /// ```
    pub fn new(id: CharacterId, gender: Gender) -> Self {
        Self {
            id,
            name: Name::new(format!("Character {}", id.0)).unwrap(),
            gender,
        }
    }

    /// Creates a character with a valid [`Name`].
    pub fn with_name(id: CharacterId, name: Name, gender: Gender) -> Self {
        Self { id, name, gender }
    }

    pub fn id(&self) -> CharacterId {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }
}
