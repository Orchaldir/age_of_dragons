use anyhow::{bail, Result};

/// The probability to roll *threshold* or less on a die with *max* sides.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Probability {
    threshold: u32,
    max: u32,
}

impl Probability {
    /// Creates a probability, if possible:
    ///
    /// ```
    ///# use age_of_dragons_core::data::probability::Probability;
    /// assert!(Probability::new(0, 1).is_err());
    /// assert!(Probability::new(2, 1).is_err());
    /// assert!(Probability::new(2, 2).is_ok());
    /// assert!(Probability::new(3, 4).is_ok());
    /// assert!(Probability::new(3, 5).is_ok());
    /// ```
    pub fn new(threshold: u32, max: u32) -> Result<Self> {
        if threshold == 0 {
            bail!("The threshold must be greater than 0!");
        } else if max < threshold {
            bail!(
                "The maximum {} must not be smaller than the threshold {}!",
                max,
                threshold
            );
        }

        Ok(Probability { threshold, max })
    }
}
