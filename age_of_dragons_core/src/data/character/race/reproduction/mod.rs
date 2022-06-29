use crate::data::character::race::reproduction::count::OffspringCount;

pub mod count;

#[derive(Clone, Debug, PartialEq)]
pub struct ReproductionOption {
    offspring_count: OffspringCount,
}
