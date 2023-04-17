use std::path;

use bevy::prelude::{AssetServer, Commands, Res};
use glob::glob;

use neociv_civil::runtime::NeocivRuntime;
use neociv_config::NeocivConfig;
use neociv_game::plugins::registry::registry::NeocivRegistry;

use super::loader::load;

pub fn loader_startup(
    mut commands: Commands,
    config: Res<NeocivConfig>,
    registry: Res<NeocivRegistry>,
    runtime: Res<NeocivRuntime>,
    asset_server: Res<AssetServer>,
) {
    println!("LOADING THE LOADER");

    // Glob path constructor
    let gpath = path::Path::new(&config.content.root)
        .join("*")
        .join("manifest.json")
        .display()
        .to_string();

    // Get a list of all directories with a manifest.json under the content root
    for entry in glob(gpath.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => load(
                path.display().to_string(),
                &registry,
                &runtime,
                &asset_server,
            ),
            Err(e) => panic!("{:?}", e),
        };
    }

    // Iterate over the content queue and load the resources
    /*
    let content_result = runtime.load_content(|tbl| {
        let id: String = tbl.get::<_, Lua
        Ok(())
    });

    match content_result {
        Ok(_) => println!("Loaded content"),
        Err(_) => panic!("Failed to load content"),
    }
    */

    commands.insert_resource(registry.clone());
}
