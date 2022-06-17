/// A duration in years.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Duration(u32);

impl Duration {
    pub fn new(years: u32) -> Self {
        Duration(years)
    }
}
