/// Which [`Gender`]s are available for [`Character`]s of this [`Race`]?
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GenderOption {
    /// The [`Race`] has no gender, which limits the reproduction options.
    NoGender,
    /// The [`Race`] has males & females.
    TwoGenders,
}
