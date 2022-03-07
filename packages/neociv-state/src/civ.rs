use regex::*;

use crate::alignments::Alignments;

/// CivIds are namespaced strings delimited by "." and act as an identifier whenever a Civ needs to
/// be referenced from another context. This should be the only means of identifying a Civ.
///
/// # Examples
/// neociv.contrib.aus
/// expanse.belters.ilus
/// mod.civs.example
pub type CivId = String;

/// CivKeys are identical to CivIds except that they have an index attached to them. In practice
/// this will be indexed to the sub-index of pre-existing Civs with this CivId. A game with all
/// unique Civs will have keys that all end in "[0]" as they are all the first entry for the given
/// CivId.
///
/// # Examples
/// neociv.contrib.aus[0]
/// expanse.belters[0]
/// mod.civs.example[0]
pub type CivKey = String;

lazy_static! {
    /// Regex to validate CivIds
    pub static ref VALID_CIV_ID: Regex = Regex::new(r"\w+\.\w(?:\.\w)*").unwrap();
}

lazy_static! {
    /// Regex to validate CivKeys
    pub static ref VALID_CIV_KEY: Regex = Regex::new(r"\w+\.\w(?:\.\w)*\[\d+\]$").unwrap();
}

#[derive(Clone, Default, Debug)]
pub struct Civ {
    pub id: CivId,
    pub title: String,
    pub alignments: Alignments,
}
