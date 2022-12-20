//use neociv_state::state::NeocivState;
use neociv_desktop::init_desktop_app;
use neociv_config::NeocivConfig;
use neociv_game::NeocivGamePlugin;
use neociv_state::state::NeocivState;

use neociv_desktop::plugins::console::ConsolePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Init,
    Menu,
    Game,
}

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

fn main() {
    let mut app = init_desktop_app(NeocivConfig::default());

    // Search for config and saves
    app.add_state(AppState::Init);

    // NB: This is just a blank default until we figure out proper loading
    app.insert_resource(NeocivState::default());

    // TODO: Add load state startup system here
    app.add_plugin(NeocivGamePlugin);

    // TODO: Put this behind some options
    app.add_plugin(ConsolePlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2);

    app.add_state(AppState::Game);

    app.run();
}
