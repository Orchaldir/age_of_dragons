use crate::data::character::gender::Gender;
use crate::data::character::race::stage::LifeStageId;
use crate::data::character::race::{Race, RaceId};
use crate::data::name::Name;
use crate::data::time::{Date, Duration};
use anyhow::{bail, Context, Result};

pub mod gender;
pub mod manager;
pub mod race;
pub mod relation;

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
    life_stage: LifeStageId,
    gender: Gender,
    birth_date: Date,
    /// The death date is only available, if the character is death.
    death_date: Option<Date>,
}

impl Character {
    /// Creates a character, if valid:
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::Character;
    ///# use age_of_dragons_core::data::character::gender::Gender::*;
    ///# use age_of_dragons_core::data::character::race::Race;
    ///# use age_of_dragons_core::data::character::race::gender::GenderOption::*;
    ///# use age_of_dragons_core::data::time::Date;
    /// let race = Race::simple(32, TwoGenders);
    /// let date = Date::new(20);
    ///
    /// assert!(Character::new(11, "C0", &race, Female, date, None).is_ok());
    /// assert!(Character::new(11, "C0", &race, Male, date, None).is_ok());
    /// assert!(Character::new(11, "C0", &race, Genderless, date, None).is_err());
    /// ```
    pub fn new<S: Into<String>>(
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
        } else if death_date.map(|d| d < birth_date).unwrap_or(false) {
            bail!(
                "The character {}'s death {:?} happened before  its birth {:?}!",
                id,
                death_date.unwrap(),
                birth_date
            );
        }

        let name = Name::new(name).with_context(|| format!("Failed to create character {}", id))?;

        Ok(Self {
            id: CharacterId::new(id),
            name,
            gender,
            race_id: race.id(),
            life_stage: LifeStageId::new(0),
            birth_date,
            death_date,
        })
    }

    /// A simple way to create a character for testing.
    pub fn simple(
        id: usize,
        race_id: RaceId,
        gender: Gender,
        birth_date: Date,
        death_date: Option<Date>,
    ) -> Self {
        Self {
            id: CharacterId::new(id),
            name: Name::new(format!("Chacarer {}", id)).unwrap(),
            gender,
            race_id,
            life_stage: LifeStageId::new(0),
            birth_date,
            death_date,
        }
    }

    pub fn id(&self) -> CharacterId {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn race_id(&self) -> RaceId {
        self.race_id
    }

    pub fn life_stage(&self) -> LifeStageId {
        self.life_stage
    }

    pub fn set_life_stage(&mut self, life_stage: LifeStageId) {
        self.life_stage = life_stage;
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

    pub fn set_death_date(&mut self, date: Date) {
        self.death_date = Some(date);
    }

    pub fn is_alive(&self) -> bool {
        self.death_date.is_none()
    }

    pub fn is_dead(&self) -> bool {
        self.death_date.is_some()
    }

    /// Calculates the current age of an alive character or the age they reached before dying otherwise.
    pub fn calculate_age(&self, now: Date) -> Duration {
        if let Some(death_date) = self.death_date {
            death_date
        } else if now < self.birth_date {
            panic!("Failed to calculate age before birth!")
        } else {
            now
        }
        .get_duration_since(self.birth_date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::character::gender::Gender::Female;
    use crate::data::character::race::gender::GenderOption::TwoGenders;

    #[test]
    fn test_new_with_invalid_name() {
        let race = Race::simple(32, TwoGenders);

        assert!(Character::new(0, "", &race, Female, Date::new(20), None).is_err());
    }

    #[test]
    fn test_new_with_death_before_birth() {
        let race = Race::simple(32, TwoGenders);

        assert!(
            Character::new(0, "C0", &race, Female, Date::new(20), Some(Date::new(10))).is_err()
        );
    }

    #[test]
    fn test_alive_character() {
        let race = Race::simple(32, TwoGenders);
        let character = Character::new(0, "C0", &race, Female, Date::new(20), None).unwrap();

        assert_eq!(character.death_date(), None);
        assert!(character.is_alive());
        assert!(!character.is_dead());
        assert_eq!(character.calculate_age(Date::new(20)), Duration::new(0));
        assert_eq!(character.calculate_age(Date::new(21)), Duration::new(1));
        assert_eq!(character.calculate_age(Date::new(22)), Duration::new(2));
    }

    #[test]
    fn test_dead_character() {
        let race = Race::simple(32, TwoGenders);
        let date = Date::new(45);
        let character = Character::new(0, "C0", &race, Female, Date::new(20), Some(date)).unwrap();

        assert_eq!(character.death_date(), Some(date));
        assert!(!character.is_alive());
        assert!(character.is_dead());

        let age = Duration::new(25);

        for year in 0..100 {
            assert_eq!(character.calculate_age(Date::new(year)), age);
        }
    }

    #[test]
    #[should_panic]
    fn test_calculate_age_before_birth() {
        let race = Race::simple(32, TwoGenders);
        let character = Character::new(0, "C0", &race, Female, Date::new(20), None).unwrap();

        character.calculate_age(Date::new(10));
    }
}
