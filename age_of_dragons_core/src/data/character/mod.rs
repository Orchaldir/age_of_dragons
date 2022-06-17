use crate::data::character::gender::Gender;
use crate::data::character::race::{Race, RaceId};
use crate::data::name::Name;
use crate::data::time::{Date, Duration};
use anyhow::{bail, Context, Result};

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
    life_stage: usize,
    gender: Gender,
    birth_date: Date,
    death_date: Option<Date>,
}

impl Character {
    /// Creates a character with a default [`Name`].
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::{Character, CharacterId};
    ///# use age_of_dragons_core::data::character::gender::Gender::*;
    /// let character = Character::new(11, Female);
    ///
    /// assert_eq!(character.id(), CharacterId::new(11));
    /// assert_eq!(character.name().name(), "Character 11");
    /// assert_eq!(character.gender(), Female);
    /// ```
    pub fn new(id: usize, gender: Gender) -> Self {
        Self {
            id: CharacterId::new(id),
            name: Name::new(format!("Character {}", id)).unwrap(),
            race_id: RaceId::new(0),
            life_stage: 0,
            gender,
            birth_date: Date::default(),
            death_date: None,
        }
    }

    /// Creates a character, if valid:
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::Character;
    ///# use age_of_dragons_core::data::character::gender::Gender::*;
    ///# use age_of_dragons_core::data::character::race::Race;
    ///# use age_of_dragons_core::data::character::race::gender::GenderOption;
    ///# use age_of_dragons_core::data::character::race::stage::LifeStage;
    ///# use age_of_dragons_core::data::time::Date;
    /// let stage = LifeStage::simple();
    /// let race = Race::simple(32, GenderOption::TwoGenders, vec![stage]);
    /// let date = Date::new(20);
    ///
    /// assert!(Character::validate(11, "C0", &race, Female, date, None).is_ok());
    /// assert!(Character::validate(11, "C0", &race, Male, date, None).is_ok());
    /// assert!(Character::validate(11, "C0", &race, Genderless, date, None).is_err());
    /// ```
    pub fn validate<S: Into<String>>(
        id: usize,
        name: S,
        race: &Race,
        gender: Gender,
        birth_date: Date,
        death_date: Option<Date>,
    ) -> Result<Self> {
        let name = name.into();

        if !race.gender_option().is_valid(gender) {
            bail!(
                "Character {} is invalid, because {:?} doesn't match the race's {:?}!",
                id,
                gender,
                race.gender_option()
            );
        }

        let name = Name::new(name).with_context(|| format!("Failed to create character {}", id))?;

        Ok(Self {
            id: CharacterId::new(id),
            name,
            gender,
            race_id: race.id(),
            life_stage: 0,
            birth_date,
            death_date,
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

    pub fn birth_date(&self) -> Date {
        self.birth_date
    }

    pub fn death_date(&self) -> Option<Date> {
        self.death_date
    }

    pub fn is_alive(&self) -> bool {
        self.death_date.is_none()
    }

    pub fn is_dead(&self) -> bool {
        self.death_date.is_some()
    }

    pub fn calculate_age(&self, date: Date) -> Duration {
        if let Some(death_date) = self.death_date {
            death_date
        } else {
            date
        }
        .get_duration_since(self.birth_date)
    }
}
