use crate::data::character::race::gender::GenderOption;
use crate::data::character::race::stage::LifeStage;
use crate::data::name::Name;
use crate::data::time::Duration;

pub mod gender;
pub mod stage;

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

/// The race of a [`Character`](crate::data::character::Character).
/// Examples are dragon, thunder bird & giant spider.
#[derive(Clone, Debug, PartialEq)]
pub struct Race {
    id: RaceId,
    name: Name,
    gender_option: GenderOption,
    stages: Vec<LifeStage>,
}

impl Race {
    /// Creates a race.
    pub fn new(
        id: RaceId,
        name: Name,
        gender_option: GenderOption,
        stages: Vec<LifeStage>,
    ) -> Self {
        Self {
            id,
            name,
            gender_option,
            stages,
        }
    }

    /// A simple way to create a race for testing.
    pub fn simple(id: usize, gender_option: GenderOption, stages: Vec<LifeStage>) -> Self {
        Self {
            id: RaceId::new(id),
            name: Name::new(format!("Race {}", id)).unwrap(),
            gender_option,
            stages,
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

    pub fn stages(&self) -> &[LifeStage] {
        &self.stages
    }

    /// Calculates the [`LifeStage`] of a [`Character`](crate::data::character::Character) based on its age.
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::race::{Race, RaceId};
    ///# use age_of_dragons_core::data::character::race::gender::GenderOption;
    ///# use age_of_dragons_core::data::character::race::stage::LifeStage;
    ///# use age_of_dragons_core::data::name::Name;
    ///# use age_of_dragons_core::data::time::Duration;
    /// let stage0 = LifeStage::new(Name::new("LS0").unwrap(), 0, Some(Duration::new(1)));
    /// let stage1 = LifeStage::new(Name::new("LS1").unwrap(), 1, Some(Duration::new(3)));
    /// let race = Race::simple(32, GenderOption::TwoGenders, vec![stage0.clone(), stage1.clone()]);
    ///
    /// assert_eq!(race.calculate_life_stage(&Duration::new(0)), Some(&stage0));
    /// assert_eq!(race.calculate_life_stage(&Duration::new(1)), Some(&stage1));
    /// assert_eq!(race.calculate_life_stage(&Duration::new(2)), Some(&stage1));
    /// assert_eq!(race.calculate_life_stage(&Duration::new(3)), None);
    /// ```
    pub fn calculate_life_stage(&self, age: &Duration) -> Option<&LifeStage> {
        for stage in &self.stages {
            if let Some(duration) = stage.duration() {
                if age < duration {
                    return Some(stage);
                }
            } else {
                return Some(stage);
            }
        }

        None
    }
}
