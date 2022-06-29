use crate::data::character::race::reproduction::count::OffspringCount;
use crate::data::probability::Probability;

pub mod count;

/// How can a [`Race`](crate::data::character::race::Race) produce?
#[derive(Clone, Debug, PartialEq)]
pub struct ReproductionOption {
    probability: Probability,
    offspring_count: OffspringCount,
}

impl ReproductionOption {
    pub fn new(probability: Probability, offspring_count: OffspringCount) -> ReproductionOption {
        ReproductionOption {
            probability,
            offspring_count,
        }
    }
}
