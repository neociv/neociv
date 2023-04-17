use std::path::Path;

use bevy::prelude::AssetServer;
use neociv_civil::runtime::NeocivRuntime;
use neociv_game::plugins::registry::registry::NeocivRegistry;

pub fn load(manifest_path: String, reg: &NeocivRegistry, runtime: &NeocivRuntime, asset_server: &AssetServer) {
    // Get the base directory of the manifest and locate package/init.(lua|fnl|cvl) to run
    let base_dir = Path::new(manifest_path.as_str()).parent().unwrap();
    let pkg_dir = base_dir.join("package");

    for ext in vec!["lua", "fnl", "cvl"] {
        let init_path = pkg_dir.join(format!("init.{}", ext));

        if init_path.exists() {
            println!("Running {}", init_path.display());

            // Run the provided content init file
            match runtime.dofile::<()>(init_path.display().to_string().as_str()) {
                Ok(_) => println!("Going great"),
                Err(ex) => panic!("{:?}", ex),
            }

            // Break out of the loop as an init file has been found
            break;
        }
    }
}
