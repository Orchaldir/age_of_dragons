use crate::generation::number::RandomNumberGenerator;
use anyhow::{bail, Result};

/// The probability of an event happening.
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

    /// Check if the event is happening.
    ///
    /// ```
    ///# use age_of_dragons_core::data::probability::Probability;
    ///# use age_of_dragons_core::generation::number::RandomNumberGenerator;
    /// let rng = RandomNumberGenerator::Mock {values: vec![0, 1, 2, 3, 4], default: 0};
    /// let probability = Probability::new(2, 4).unwrap();
    ///
    /// assert_eq!(probability.check(&rng, 0), 0 < 2);
    /// assert_eq!(probability.check(&rng, 1), 1 < 2);
    /// assert_eq!(probability.check(&rng, 2), 2 < 2);
    /// assert_eq!(probability.check(&rng, 3), 3 < 2);
    /// assert_eq!(probability.check(&rng, 4), 0 < 2);
    /// ```
    pub fn check(&self, rng: &RandomNumberGenerator, index: usize) -> bool {
        rng.generate(index, self.max) < self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_with_default() {
        let rng = RandomNumberGenerator::Mock {
            values: Vec::new(),
            default: 10,
        };

        assert!(!Probability::new(9, 100).unwrap().check(&rng, 0));
        assert!(!Probability::new(10, 100).unwrap().check(&rng, 0));
        assert!(Probability::new(11, 100).unwrap().check(&rng, 0));
    }
}
