use crate::data::character::race::gender::GenderOption;
use crate::data::character::race::stage::LifeStage;
use crate::data::name::Name;
use crate::data::time::Duration;
use anyhow::{bail, Context, Result};

pub mod gender;
pub mod manager;
pub mod reproduction;
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
        id: usize,
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
            if let Some(current) = stage.max_age() {
                if let Some(previous) = &previous_max_age {
                    if current <= previous {
                        bail!(
                            "Race {}'s life stage {} ends before previous stages!",
                            name,
                            i
                        );
                    }
                }
            } else if i < last_i {
                bail!(
                    "Race {}'s life stage {} has no max age, but is not last!",
                    name,
                    i
                );
            }

            if i != stage.id().index() {
                bail!("Race {}'s life stage {} has wrong index!", name, i);
            }

            previous_max_age = *stage.max_age();
        }

        let name = Name::new(name).with_context(|| format!("Failed to create race {}", id))?;

        Ok(Self {
            id: RaceId::new(id),
            name,
            gender_option,
            stages,
        })
    }

    /// A simple way to create a race for testing.
    pub fn simple(id: usize, gender_option: GenderOption) -> Self {
        Self {
            id: RaceId::new(id),
            name: Name::new(format!("Race {}", id)).unwrap(),
            gender_option,
            stages: vec![LifeStage::simple()],
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
    ///# use age_of_dragons_core::data::character::race::Race;
    ///# use age_of_dragons_core::data::character::race::gender::GenderOption::*;
    ///# use age_of_dragons_core::data::character::race::stage::LifeStage;
    ///# use age_of_dragons_core::data::name::Name;
    ///# use age_of_dragons_core::data::time::Duration;
    /// let stage0 = LifeStage::new("LS0", 0, Some(Duration::new(1)), None).unwrap();
    /// let stage1 = LifeStage::new("LS1", 1, Some(Duration::new(3)), None).unwrap();
    /// let race = Race::new(32, "R0", TwoGenders, vec![stage0.clone(), stage1.clone()]).unwrap();
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
pub mod tests {
    use super::*;
    use crate::data::character::race::gender::GenderOption::NoGender;
    use crate::data::character::race::manager::RaceMgr;
    use crate::data::character::race::reproduction::count::OffspringCount;
    use crate::data::character::race::reproduction::ReproductionOption;
    use crate::data::probability::Probability;
    use GenderOption::TwoGenders;

    #[test]
    fn test_new() {
        let stage0 = LifeStage::new("LF0", 0, Some(Duration::new(44)), None).unwrap();
        let stage1 = LifeStage::new("LF1", 1, None, None).unwrap();

        assert!(Race::new(0, "Test", TwoGenders, vec![stage0, stage1]).is_ok());
    }

    #[test]
    fn test_new_with_invalid_name() {
        let stage = LifeStage::new("LF", 1, None, None).unwrap();

        assert!(Race::new(0, "", TwoGenders, vec![stage]).is_err());
    }

    #[test]
    fn test_new_without_stages() {
        assert!(Race::new(0, "Test", TwoGenders, vec![]).is_err());
    }

    #[test]
    fn test_new_with_early_stage_is_endless() {
        let stage0 = LifeStage::new("LF0", 0, None, None).unwrap();
        let stage1 = LifeStage::new("LF1", 1, None, None).unwrap();

        assert!(Race::new(0, "Test", TwoGenders, vec![stage0, stage1]).is_err());
    }

    #[test]
    fn test_new_with_early_stage_ends_after_later_stage() {
        let stage0 = LifeStage::new("LF0", 0, Some(Duration::new(20)), None).unwrap();
        let stage1 = LifeStage::new("LF1", 1, Some(Duration::new(10)), None).unwrap();

        assert!(Race::new(0, "Test", TwoGenders, vec![stage0, stage1]).is_err());
    }

    #[test]
    fn test_new_with_wrong_index() {
        let stage0 = LifeStage::new("LF0", 1, Some(Duration::new(44)), None).unwrap();
        let stage1 = LifeStage::new("LF1", 0, None, None).unwrap();

        assert!(Race::new(0, "Test", TwoGenders, vec![stage0, stage1]).is_err());
    }

    pub fn create_mortal_race(manager: &mut RaceMgr, age0: u32, age1: u32) -> RaceId {
        let probability = Probability::new(1, 5).unwrap();
        let count = OffspringCount::new_fixed_count(1).unwrap();
        let reproduction = ReproductionOption::new(probability, count);
        let stage0 = LifeStage::new("Child", 0, Some(Duration::new(age0)), None).unwrap();
        let stage1 =
            LifeStage::new("Adult", 1, Some(Duration::new(age1)), Some(reproduction)).unwrap();
        let stages = vec![stage0, stage1];

        manager
            .create(|id| Race::new(id.id(), "Mortal Race", TwoGenders, stages))
            .unwrap()
    }

    pub fn create_immortal_race(id: RaceId) -> Result<Race> {
        let stage = LifeStage::new("Immortal", 0, None, None).unwrap();
        let stages = vec![stage];
        Race::new(id.id(), "Immortal Race", NoGender, stages)
    }
}
