use bevy::utils::HashMap;

// TODO: Obviously change these to the correct entry type for Bevy's sake

/// The globally available map of all resources that can be referenced by id strings. Meshes, Materials, Sounds,
/// Images, etc... all live here. All content from mods run through the... run... time will be stored here. The
/// registry will then handle interfacing with Bevy.
pub struct NeocivRegistry {
    mesh_map: HashMap<&str, ()>,
    mat_map: HashMap<&str, ()>,
    img_map: HashMap<&str, ()>,
    snd_map: HashMap<&str, ()>,
}

impl NeocivRegistry {
}
