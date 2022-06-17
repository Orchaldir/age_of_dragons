use crate::data::name::Name;
use crate::data::time::Duration;

/// Members of most [`Races`](crate::data::character::race::Race) go through multiple life stages while growing up.
#[derive(Clone, Debug, PartialEq)]
pub struct LifeStage {
    name: Name,
    /// The life stage lasts forever, if the duration is None.
    duration: Option<Duration>,
}

impl LifeStage {
    /// Creates a life stage.
    pub fn new(name: Name, duration: Option<Duration>) -> Self {
        Self { name, duration }
    }

    /// A simple way to create a life stage for testing.
    pub fn simple() -> Self {
        Self {
            name: Name::new("Life Stage").unwrap(),
            duration: None,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn duration(&self) -> &Option<Duration> {
        &self.duration
    }
}
