extern crate neociv_app as app;
extern crate neociv_config as config;
extern crate neociv_game as game;
extern crate neociv_state;

use neociv_state::engine;
use bevy::math::Vec3;

fn main() {
    let mut app = app::init_desktop_app(config::NeocivConfig::default());

    // Generate a random grid
    let mut state = engine::init();

    // Generate an unowned default grid
    state = engine::set_grid_size(state, 25, 25).unwrap();

    // Move the camera
    state = engine::mod_camera_position(state, bevy::math::Vec3::new(0.0, 0.0, 100.0)).unwrap();

    // Add the state resource - the runtime setup will detect and instead use this instance
    app.insert_resource(state);

    // Add the game plugin
    app.add_plugin(game::NeocivGamePlugin);

    app.add_plugin(app::plugins::console::ConsolePlugin);

    app.run();
}
