use bevy::prelude::*;

use neociv_config::NeocivConfig;

/// Initialise and setup a basic app window
pub fn init_desktop_app(config: NeocivConfig) -> App {
    let mut app = App::new();

    // Graphics options
    app.insert_resource(Msaa { samples: config.video.msaa_samples });

    // Basic window descriptor
    app.insert_resource(WindowDescriptor {
        vsync: config.video.vsync,
        ..Default::default()
    });

    // Black screen by default
    app.insert_resource(ClearColor(Color::hex("000000").unwrap()));

    // Load the default plugins
    app.add_plugins(DefaultPlugins);

    return app;
}