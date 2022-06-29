use anyhow::{bail, Result};

/// How many offspring can a [`Race`](crate::data::character::race::Race) produce at once?
#[derive(Clone, Debug, PartialEq)]
pub enum OffspringCount {
    Fixed(u32),
    Range { min: u32, max: u32 },
}

impl OffspringCount {
    /// Creates a fixed offspring count, if the count is greater 0:
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::race::reproduction::count::OffspringCount;
    /// assert!(OffspringCount::new_fixed_count(0).is_err());
    /// assert!(OffspringCount::new_fixed_count(1).is_ok());
    /// assert!(OffspringCount::new_fixed_count(2).is_ok());
    /// assert!(OffspringCount::new_fixed_count(3).is_ok());
    /// ```
    pub fn new_fixed_count(count: u32) -> Result<OffspringCount> {
        if count == 0 {
            bail!("The fixed offspring count must be greater than 0!");
        }

        Ok(OffspringCount::Fixed(count))
    }

    /// Creates a range of offspring count,that has equal probability:
    ///
    /// ```
    ///# use age_of_dragons_core::data::character::race::reproduction::count::OffspringCount;
    /// assert!(OffspringCount::new_range(0, 1).is_err());
    /// assert!(OffspringCount::new_range(2, 1).is_err());
    /// assert!(OffspringCount::new_range(2, 2).is_err());
    /// assert!(OffspringCount::new_range(3, 4).is_ok());
    /// assert!(OffspringCount::new_range(3, 5).is_ok());
    /// ```
    pub fn new_range(min: u32, max: u32) -> Result<OffspringCount> {
        if min == 0 {
            bail!("The minimum must be greater than 0!");
        } else if max <= min {
            bail!(
                "The maximum {} must be greater than the minimum {}!",
                max,
                min
            );
        }

        Ok(OffspringCount::Range { min, max })
    }
}
