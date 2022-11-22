use bevy::utils::HashMap;

// TODO: Obviously change these to the correct entry type for Bevy's sake

pub type NeocivMesh = ();
pub type NeocivMat = ();
pub type NeocivImg = ();
pub type NeocivSnd = ();

/// The globally available map of all resources that can be referenced by id strings. Meshes, Materials, Sounds,
/// Images, etc... all live here. All content from mods run through the... run... time will be stored here. The
/// registry will then handle interfacing with Bevy.
pub struct NeocivRegistry {
    mesh_map: HashMap<String, NeocivMesh>,
    mat_map: HashMap<String, NeocivMat>,
    img_map: HashMap<String, NeocivImg>,
    snd_map: HashMap<String, NeocivSnd>,
}

impl NeocivRegistry {
    fn has(key: String) -> bool {
        return false;
    }
}
