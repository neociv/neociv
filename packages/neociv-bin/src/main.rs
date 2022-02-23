use bevy::prelude::{App, ClearColor, Color, WindowDescriptor, Msaa, DefaultPlugins};

use neociv_game::NeocivGamePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Init
}

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

fn main() {
    let mut app = App::new();

    // Search for config and saves
    app.add_state(AppState::Init);

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor { vsync: false, ..Default::default() })
        .insert_resource(ClearColor(Color::hex("000000").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(NeocivGamePlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2);

    app.run();
}
