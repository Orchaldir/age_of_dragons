/// Currently dates are limited to years.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Date(u32);

impl Date {
    pub fn new(years: u32) -> Self {
        Date(years)
    }

    pub fn year(&self) -> u32 {
        self.0
    }

    /// Calculates the [`Duration`] between 2 dates.
    ///
    /// ```
    ///# use age_of_dragons_core::data::time::{Date, Duration};
    /// let date0 = Date::new(20);
    /// let date1 = Date::new(5);
    /// let duration = Duration::new(15);
    ///
    /// assert_eq!(date0.get_duration_since(date1), duration);
    /// assert_eq!(date1.get_duration_since(date0), duration);
    /// ```
    pub fn get_duration_since(&self, date: Date) -> Duration {
        Duration::new(self.0.abs_diff(date.0))
    }

    pub fn increase_year(&mut self) {
        self.0 += 1;
    }
}

/// A duration in years.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration(u32);

impl Duration {
    pub fn new(years: u32) -> Self {
        Duration(years)
    }

    pub fn year(&self) -> u32 {
        self.0
    }
}
