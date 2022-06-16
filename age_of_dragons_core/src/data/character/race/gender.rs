use crate::data::character::gender::Gender;

/// Which [`Gender`]s are available for [`Character`]s of this [`Race`]?
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GenderOption {
    /// The [`Race`] has no gender, which limits the reproduction options.
    NoGender,
    /// The [`Race`] has males & females.
    TwoGenders,
}

impl GenderOption {
    /// Is the [`Gender`] valid for this option?
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::race::gender::GenderOption::*;
    ///# use age_of_dragons_core::data::character::gender::Gender;
    ///
    /// assert!(!NoGender.is_valid(Gender::Female));
    /// assert!(!NoGender.is_valid(Gender::Male));
    /// assert!(NoGender.is_valid(Gender::NoGender));
    ///
    /// assert!(TwoGenders.is_valid(Gender::Female));
    /// assert!(TwoGenders.is_valid(Gender::Male));
    /// assert!(!TwoGenders.is_valid(Gender::NoGender));
    /// ```
    pub fn is_valid(&self, gender: Gender) -> bool {
        match self {
            GenderOption::NoGender => gender == Gender::NoGender,
            GenderOption::TwoGenders => gender == Gender::Female || gender == Gender::Male,
        }
    }
}
