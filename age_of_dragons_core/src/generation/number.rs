use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hasher;

#[derive(Clone, Debug, PartialEq)]
pub enum RandomNumberGenerator {
    Hash {
        year: u32,
        usage: u32,
    },
    Mock {
        values: HashMap<usize, u64>,
        default: u64,
    },
}

impl RandomNumberGenerator {
    pub fn new_hash(year: u32, usage: u32) -> Self {
        Self::Hash { year, usage }
    }

    /// Generates a number between 0 and exclusive the maximum.
    pub fn generate(&self, index: usize, max: u32) -> u32 {
        (self.next(index) % (max as u64)) as u32
    }

    fn next(&self, index: usize) -> u64 {
        match self {
            Self::Hash { year, usage } => {
                let mut hasher = DefaultHasher::new();
                hasher.write_u32(*year);
                hasher.write_usize(index);
                hasher.write_u32(*usage);
                hasher.finish()
            }
            Self::Mock { values, default } => *values.get(&index).unwrap_or(default),
        }
    }
}
