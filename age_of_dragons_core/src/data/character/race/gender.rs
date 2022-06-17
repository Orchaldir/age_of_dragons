use crate::data::character::gender::Gender;

/// Which [`Gender`]s are available for members of this [`Race`](crate::data::character::race::Race)?
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GenderOption {
    /// The [`Race`](crate::data::character::race::Race) has no gender, which limits the reproduction options.
    NoGender,
    /// The [`Race`](crate::data::character::race::Race) has males & females.
    TwoGenders,
}

impl GenderOption {
    /// Is the [`Gender`] valid for this option?
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::race::gender::GenderOption::*;
    ///# use age_of_dragons_core::data::character::gender::Gender::*;
    ///
    /// assert!(!NoGender.is_valid(Female));
    /// assert!(!NoGender.is_valid(Male));
    /// assert!(NoGender.is_valid(Genderless));
    ///
    /// assert!(TwoGenders.is_valid(Female));
    /// assert!(TwoGenders.is_valid(Male));
    /// assert!(!TwoGenders.is_valid(Genderless));
    /// ```
    pub fn is_valid(&self, gender: Gender) -> bool {
        match self {
            GenderOption::NoGender => gender == Gender::Genderless,
            GenderOption::TwoGenders => gender == Gender::Female || gender == Gender::Male,
        }
    }
}
