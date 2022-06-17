use crate::data::character::gender::Gender;
use crate::data::character::race::{Race, RaceId};
use crate::data::name::Name;
use anyhow::{bail, Result};

pub mod gender;
pub mod race;

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
    race_id: RaceId,
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
            race_id: RaceId::new(0),
            gender,
        }
    }

    /// Creates a character, if valid:
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::{Character, CharacterId};
    ///# use age_of_dragons_core::data::character::gender::Gender;
    ///# use age_of_dragons_core::data::character::race::{Race, RaceId};
    ///# use age_of_dragons_core::data::character::race::gender::GenderOption::*;
    ///# use age_of_dragons_core::data::character::race::stage::LifeStage;
    ///# use age_of_dragons_core::data::name::Name;
    /// let stage = LifeStage::simple();
    /// let race = Race::simple(32, TwoGenders, vec![stage]);
    /// let id = CharacterId::new(11);
    /// let name = Name::new("C0").unwrap();
    ///
    /// assert!(Character::validate(id, name.clone(), &race, Gender::Female).is_ok());
    /// assert!(Character::validate(id, name.clone(), &race, Gender::Male).is_ok());
    /// assert!(Character::validate(id, name, &race, Gender::NoGender).is_err());
    /// ```
    pub fn validate(id: CharacterId, name: Name, race: &Race, gender: Gender) -> Result<Self> {
        if !race.gender_option().is_valid(gender) {
            bail!(
                "Character {} is invalid, because {:?} doesn't match the race's {:?}!",
                id.0,
                gender,
                race.gender_option()
            );
        }

        Ok(Self {
            id,
            name,
            gender,
            race_id: race.id(),
        })
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
