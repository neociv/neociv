use crate::alignments::Alignments;

/// CivIds are namespaced strings delimited by "." and act as an identifier whenever a Civ needs to
/// be referenced from another context. This should be the only means of identifying a Civ.
///
/// # Examples
/// neociv.contrib.aus
/// expanse.belters.ilus
/// mod.civs.example
pub type CivId = String;

#[derive(Clone, Default)]
pub struct Civ {
    pub id: CivId,
    pub title: String,
    pub alignments: Alignments,
}
