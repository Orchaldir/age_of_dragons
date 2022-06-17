use crate::data::character::race::gender::GenderOption;
use crate::data::character::race::stage::LifeStage;
use crate::data::name::Name;
use crate::data::time::Duration;
use anyhow::{bail, Context, Result};

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
    /// Creates a race, if valid.
    pub fn new<S: Into<String>>(
        id: RaceId,
        name: S,
        gender_option: GenderOption,
        stages: Vec<LifeStage>,
    ) -> Result<Self> {
        let name = name.into();

        if stages.is_empty() {
            bail!("Race {} has no life stages!", name);
        }

        let last_i = stages.len() - 1;
        let mut previous_max_age: Option<Duration> = None;

        for (i, stage) in stages.iter().enumerate() {
            if stage.max_age().is_none() && i < last_i {
                bail!(
                    "Race {}'s life stage {} has no max age, but is not last!",
                    name,
                    i
                );
            } else if let Some(previous) = &previous_max_age {
                if let Some(current) = stage.max_age() {
                    if current <= previous {
                        bail!(
                            "Race {}'s life stage {} ends before previous stages!",
                            name,
                            i
                        );
                    }
                }
            } else if i != stage.index() {
                bail!("Race {}'s life stage {} has wrong index!", name, i);
            }

            previous_max_age = *stage.max_age();
        }

        let name = Name::new(name).with_context(|| format!("Failed to create race {}", id.0))?;

        Ok(Self {
            id,
            name,
            gender_option,
            stages,
        })
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
    /// assert_eq!(race.calculate_life_stage(&Duration::new(1)), Some(&stage0));
    /// assert_eq!(race.calculate_life_stage(&Duration::new(2)), Some(&stage1));
    /// assert_eq!(race.calculate_life_stage(&Duration::new(3)), Some(&stage1));
    /// assert_eq!(race.calculate_life_stage(&Duration::new(4)), None);
    /// ```
    pub fn calculate_life_stage(&self, age: &Duration) -> Option<&LifeStage> {
        for stage in &self.stages {
            if let Some(max_age) = stage.max_age() {
                if age <= max_age {
                    return Some(stage);
                }
            } else {
                return Some(stage);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use GenderOption::TwoGenders;

    #[test]
    fn test_new() {
        let stage0 = LifeStage::new(Name::new("LF0").unwrap(), 0, Some(Duration::new(44)));
        let stage1 = LifeStage::new(Name::new("LF1").unwrap(), 1, None);

        assert!(Race::new(RaceId::new(0), "Test", TwoGenders, vec![stage0, stage1]).is_ok());
    }

    #[test]
    fn test_new_without_stages() {
        assert!(Race::new(RaceId::new(0), "Test", TwoGenders, vec![]).is_err());
    }

    #[test]
    fn test_new_with_early_stage_is_endless() {
        let stage0 = LifeStage::new(Name::new("LF0").unwrap(), 0, None);
        let stage1 = LifeStage::new(Name::new("LF1").unwrap(), 1, None);

        assert!(Race::new(RaceId::new(0), "Test", TwoGenders, vec![stage0, stage1]).is_err());
    }

    #[test]
    fn test_new_with_early_stage_ends_after_later_stage() {
        let stage0 = LifeStage::new(Name::new("LF0").unwrap(), 0, Some(Duration::new(20)));
        let stage1 = LifeStage::new(Name::new("LF1").unwrap(), 1, Some(Duration::new(10)));

        assert!(Race::new(RaceId::new(0), "Test", TwoGenders, vec![stage0, stage1]).is_err());
    }

    #[test]
    fn test_new_with_wrong_index() {
        let stage0 = LifeStage::new(Name::new("LF0").unwrap(), 1, Some(Duration::new(44)));
        let stage1 = LifeStage::new(Name::new("LF1").unwrap(), 0, None);

        assert!(Race::new(RaceId::new(0), "Test", TwoGenders, vec![stage0, stage1]).is_err());
    }
}
