extern crate neociv_app as app;
extern crate neociv_config as config;
extern crate neociv_game as game;
extern crate neociv_state;

use neociv_state::engine;

fn main() {
    let mut app = app::init_desktop_app(
        config::NeocivConfig::default()
    );

    // Generate a random grid
    let mut state = engine::init();

    // Generate a 5 x 5 unowned grid
    state = match engine::set_grid_size(state, 5, 5) {
        Ok(s) => s,
        Err(ex) => panic!("{:?}", ex),
    };

    // Add the state resource
    app.insert_resource(state);

    // Add the game plugin
    app.add_plugin(game::NeocivGamePlugin);

    app.run();
}

