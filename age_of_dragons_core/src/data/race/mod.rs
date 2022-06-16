use crate::data::name::Name;
use crate::data::race::gender::GenderOption;

pub mod gender;

/// The id of a [`Race`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RaceId(usize);

impl RaceId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

/// The race of a [`Character`].
/// Examples are dragon, thunder bird & giant spider.
#[derive(Clone, Debug, PartialEq)]
pub struct Race {
    id: RaceId,
    name: Name,
    gender_option: GenderOption,
}

impl Race {
    /// Creates a race with a default [`Name`].
    ///
    /// ```
    ///# use age_of_dragons_core::data::race::{Race, RaceId};
    ///# use age_of_dragons_core::data::race::gender::GenderOption::*;
    /// let id = RaceId::new(32);
    /// let race = Race::new(id, TwoGenders);
    ///
    /// assert_eq!(race.id(), id);
    /// assert_eq!(race.name().name(), "Race 32");
    /// assert_eq!(race.gender_option(), TwoGenders);
    /// ```
    pub fn new(id: RaceId, gender_option: GenderOption) -> Self {
        Self {
            id,
            name: Name::new(format!("Race {}", id.0)).unwrap(),
            gender_option,
        }
    }

    /// Creates a race with a valid [`Name`].
    pub fn with_name(id: RaceId, name: Name, gender_option: GenderOption) -> Self {
        Self {
            id,
            name,
            gender_option,
        }
    }

    pub fn id(&self) -> RaceId {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn gender_option(&self) -> GenderOption {
        self.gender_option
    }
}
