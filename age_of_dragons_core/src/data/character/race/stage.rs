use crate::data::name::Name;

/// Members of most [`Races`](crate::data::character::race::Race) go through multiple life stages while growing up.
#[derive(Clone, Debug, PartialEq)]
pub struct LifeStage {
    name: Name,
}

impl LifeStage {
    /// Creates a life stage.
    pub fn new(name: Name) -> Self {
        Self { name }
    }

    /// A simple way to create a life stage for testing.
    pub fn simple() -> Self {
        Self {
            name: Name::new("Life Stage").unwrap(),
        }
    }
}
