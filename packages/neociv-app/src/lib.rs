use bevy::prelude::*;
use bevy::window::WindowMode;

use neociv_config::NeocivConfig;

pub mod plugins;

/// Initialise and setup a basic app window
pub fn init_desktop_app(config: NeocivConfig) -> App {
    let mut app = App::new();

    // Graphics options
    app.insert_resource(Msaa {
        samples: config.video.msaa_samples,
    });

    // Basic window descriptor
    app.insert_resource(WindowDescriptor {
        width: config.window.resolution_width as f32,
        height: config.window.resolution_height as f32,
        mode: if config.window.fullscreen {
            WindowMode::Fullscreen
        } else {
            WindowMode::Windowed
        },
        ..Default::default()
    });

    // Black screen by default
    app.insert_resource(ClearColor(Color::hex("000000").unwrap()));

    // Load the default plugins
    app.add_plugins(DefaultPlugins);

    return app;
}
