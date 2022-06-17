use crate::data::character::{Character, CharacterId};

/// Stores all the [`Character`]s.
#[derive(Default, Debug)]
pub struct CharacterMgr {
    characters: Vec<Character>,
}

impl CharacterMgr {
    /// Uses the function *f* to create a [`Character`] with the next [`CharacterId`].
    pub fn create<F>(&mut self, f: F) -> CharacterId
    where
        F: FnOnce(CharacterId) -> Character,
    {
        let id = CharacterId::new(self.characters.len());
        self.characters.push(f(id));
        id
    }

    pub fn get_all(&self) -> &Vec<Character> {
        &self.characters
    }

    pub fn get(&self, id: CharacterId) -> Option<&Character> {
        self.characters.get(id.0)
    }

    pub fn get_mut(&mut self, id: CharacterId) -> Option<&mut Character> {
        self.characters.get_mut(id.0)
    }
}
