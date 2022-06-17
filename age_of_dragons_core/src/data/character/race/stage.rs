use crate::data::name::Name;
use crate::data::time::Duration;

/// Members of most [`Races`](crate::data::character::race::Race) go through multiple life stages while growing up.
#[derive(Clone, Debug, PartialEq)]
pub struct LifeStage {
    name: Name,
    index: usize,
    /// The life stage lasts forever, if it has no max age.
    max_age: Option<Duration>,
}

impl LifeStage {
    /// Creates a life stage.
    pub fn new(name: Name, index: usize, max_age: Option<Duration>) -> Self {
        Self {
            name,
            index,
            max_age,
        }
    }

    /// A simple way to create a life stage for testing.
    pub fn simple() -> Self {
        Self {
            name: Name::new("Life Stage").unwrap(),
            index: 0,
            max_age: None,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn max_age(&self) -> &Option<Duration> {
        &self.max_age
    }
}