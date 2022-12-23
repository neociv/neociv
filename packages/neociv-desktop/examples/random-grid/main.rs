extern crate neociv_desktop as app;
extern crate neociv_config as config;
extern crate neociv_game as game;
extern crate neociv_state;

use neociv_state::engine;
use bevy::math::Vec3;

fn main() {
    let mut app = app::init_desktop_app(config::NeocivConfig::default(), None);

    #[cfg(debug_assertions)]
    app.insert_resource(bevy::ecs::schedule::ReportExecutionOrderAmbiguities);

    // Generate a random grid
    let mut state = engine::init();

    // Generate an unowned default grid
    state = engine::set_grid_size(state, 25, 25).unwrap();

    // Move the camera
    state = engine::mod_camera_position(state, Vec3::new(0.0, 0.0, 100.0)).unwrap();

    // Add the state resource - the runtime setup will detect and instead use this instance
    app.insert_resource(state);

    // Add the game plugin
    app.add_plugin(game::NeocivGamePlugin);

    // Content loader plugin loads after the runtime startup
    app.add_plugin(app::plugins::loader::ContentLoaderPlugin);

    app.add_plugin(app::plugins::console::ConsolePlugin);

    app.run();
}
