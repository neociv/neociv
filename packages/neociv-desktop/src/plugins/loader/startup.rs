use std::path;

use bevy::prelude::{Commands, Res};
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
            Ok(path) => load(path.display().to_string(), &registry, &runtime),
            Err(e) => panic!("{:?}", e),
        };
    }

    commands.insert_resource(registry.clone());
}
