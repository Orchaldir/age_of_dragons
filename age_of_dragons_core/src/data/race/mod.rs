use crate::data::name::Name;

/// The id of a [`Race`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RaceId(usize);

impl RaceId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

/// The race of a [`Character`].
/// Examples are dragon, thunder bird & giant spider.
#[derive(Clone, Debug, PartialEq)]
pub struct Race {
    id: RaceId,
    name: Name,
}

impl Race {
    /// Creates a race with a default [`Name`].
    ///
    /// ```
    ///# use age_of_dragons_core::data::race::{Race, RaceId};
    /// let id = RaceId::new(32);
    /// let race = Race::new(id);
    ///
    /// assert_eq!(race.id(), id);
    /// assert_eq!(race.name().name(), "Race 32");
    /// ```
    pub fn new(id: RaceId) -> Self {
        Self {
            id,
            name: Name::new(format!("Race {}", id.0)).unwrap(),
        }
    }

    /// Creates a race with a valid [`Name`].
    pub fn with_name(id: RaceId, name: Name) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> RaceId {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
