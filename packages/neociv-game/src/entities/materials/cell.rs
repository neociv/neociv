use bevy::{prelude::Shader, render::render_resource::AsBindGroup};

use crate::entities::NeocivEntity;

#[derive(AsBindGroup, Debug, Clone, Copy)]
pub struct CellMaterial {
    #[uniform(0)]
    x: u32,

    #[uniform(1)]
    y: u32,
}

#[derive(Clone, Debug)]
pub struct CellMaterialContent {
    //material: CellMaterial,
    pub frag: Shader,
}

impl CellMaterialContent {
    pub fn new(frag: String) -> Self {
        Self {
            frag: Shader::from_wgsl(frag),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CellMaterialEntity {
    pub id: String,
    pub content: CellMaterialContent,
}

impl NeocivEntity<CellMaterialContent> for CellMaterialEntity {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn content(&self) -> &CellMaterialContent {
        &self.content
    }
}
