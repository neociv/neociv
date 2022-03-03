extern crate neociv_app as app;
extern crate neociv_config as config;
extern crate neociv_game;


fn main() {
    let mut app = app::init_desktop_app(
        config::NeocivConfig::default()
    );

    app.run();
}

