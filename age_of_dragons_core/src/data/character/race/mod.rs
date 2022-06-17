use crate::data::character::race::gender::GenderOption;
use crate::data::character::race::stage::LifeStage;
use crate::data::name::Name;

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
}
