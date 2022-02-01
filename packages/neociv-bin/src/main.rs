use bevy::prelude::{App, ClearColor, Color, WindowDescriptor, Msaa};
use bevy::window::WindowPlugin;
use bevy::winit::WinitPlugin;

use neociv_game::NeocivGamePlugin;

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::hex("000000").unwrap()))
        .add_plugin(NeocivGamePlugin)
        .add_plugin(WindowPlugin::default())
        .add_plugin(WinitPlugin::default());

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2);

    app.run();
}
