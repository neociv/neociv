use bevy::ecs::system::Resource;
use bevy::utils::HashMap;

use neociv_civil::desc::NeocivDesc;
use neociv_state::currency::{CurrencyDesc, CurrencyId};

use crate::entities::materials::cell::CellMaterialEntity;

// TODO: Obviously change these to the correct entry type for Bevy's sake

pub type NeocivMesh = ();
pub type NeocivMat = ();
pub type NeocivImg = ();
pub type NeocivSnd = ();

/// The globally available map of all resources that can be referenced by id strings. Meshes, Materials, Sounds,
/// Images, etc... all live here. All content from mods run through the... run... time will be stored here. The
/// registry will then handle interfacing with Bevy.
#[derive(Clone, Debug, Default, Resource)]
pub struct NeocivRegistry {
    pub currency_map: HashMap<CurrencyId, CurrencyDesc>,
    pub mesh_map: HashMap<String, NeocivMesh>,
    pub mat_map: HashMap<String, NeocivMat>,
    pub img_map: HashMap<String, NeocivImg>,
    pub snd_map: HashMap<String, NeocivSnd>,

    pub cell_materials: HashMap<String, CellMaterialEntity>,
}

impl NeocivRegistry {
    pub fn has(&self, key: String) -> bool {
        return false;
    }

    pub fn load(&self) -> Result<(), ()> {
        Ok(())
    }
}
