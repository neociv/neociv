use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

use neociv_config::NeocivConfig;
use neociv_state::state::NeocivState;

pub mod plugins;

/// Initialise and setup a basic app window
pub fn init_desktop_app(config: NeocivConfig, state: Option<NeocivState>) -> App {
    let mut app = App::new();

    // Black screen by default
    app.insert_resource(ClearColor(Color::hex("000000").unwrap()));

    // Graphics options
    app.insert_resource(Msaa {
        samples: config.video.msaa_samples,
    });

    // Load the default plugins and setup the window
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: config.window.resolution_width as f32,
            height: config.window.resolution_height as f32,
            mode: if config.window.fullscreen {
                WindowMode::Fullscreen
            } else {
                WindowMode::Windowed
            },
            present_mode: if config.video.vsync {
                PresentMode::AutoVsync
            } else {
                PresentMode::AutoNoVsync
            },
            ..default()
        },
        ..default()
    }));

    // Insert the config as a resource for other systems
    app.insert_resource(config);

    return app;
}

pub fn init_desktop_game(app: &App, config: NeocivConfig) -> &App {
    return app;
}
