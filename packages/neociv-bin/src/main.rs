//use neociv_state::state::NeocivState;
use neociv_game::NeocivGamePlugin;
use neociv_app::init_desktop_app;
use neociv_config::NeocivConfig;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Init,
}

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

fn main() {
    let mut app = init_desktop_app(NeocivConfig::default());

    // Search for config and saves
    app.add_state(AppState::Init);

    // TODO: Add load state startup system here
    app.add_plugin(NeocivGamePlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2);

    app.run();
}
