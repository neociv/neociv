use bevy::prelude::{Handle, Shader};

pub mod cell;

pub struct ShaderResource {
    frag: Handle<Shader>,
}

/*
impl FromResources for ShaderResource {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get_mut::<AssetServer>().unwrap();

        // Does nothing
        asset_server.watch_for_changes().unwrap();

        ShaderResource {
            vertex: asset_server.load::<Shader, _>("shaders/hot.vert"),
            fragment: asset_server.load::<Shader, _>("shaders/hot.frag"),
        }
    }
}
}
*/
