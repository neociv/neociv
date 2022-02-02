/// CivIds are namespaced strings delimited by "." and act as an identifier whenever a Civ needs to
/// be referenced from another context. This should be the only means of identifying a Civ.
///
/// # Examples
/// neociv.contrib.example
/// some.mod.example
pub type CivId = String;

#[derive(Clone)]
pub struct Civ {
    pub id: CivId,
    pub title: String,
}
