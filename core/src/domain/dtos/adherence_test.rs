use mycelium_base::dtos::UntaggedParent;
use serde::{Deserialize, Serialize};

use super::clade::Clade;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AdherenceTest {
    /// The unique identifier for the clade to be tested.
    pub clade: UntaggedParent<Clade, i32>,

    /// The number of kmer matches with the desired clade.
    pub one: i32,

    /// The number of kmer matches with sibling clades.
    pub rest: i32,
}

impl std::fmt::Display for AdherenceTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AdherenceTest: {} - {} - {}",
            self.clade, self.one, self.rest
        )
    }
}
