use anyhow::{bail, Result};

/// A string that is a valid name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name(String);

impl Name {
    /// Creates a name from a string, if possible:
    ///
    /// ```
    ///# use age_of_dragons_core::data::name::Name;
    /// assert_eq!(Name::new("Test").unwrap().name(), "Test");
    /// assert!(Name::new("").is_err());
    /// assert!(Name::new("   ").is_err());
    /// ```
    ///
    /// It will also remove leading & trailing whitespaces:
    ///
    /// ```
    ///# use age_of_dragons_core::data::name::Name;
    /// assert_eq!(Name::new(" Test ").unwrap().name(), "Test");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();
        let trimmed = name.trim();

        if trimmed.is_empty() {
            bail!("The name '{}' is invalid!", name);
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}
