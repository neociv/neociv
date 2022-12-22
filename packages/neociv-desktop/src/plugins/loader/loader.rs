use std::path::Path;

use glob::glob;

use neociv_civil::runtime::NeocivRuntime;
use neociv_game::plugins::registry::registry::NeocivRegistry;

pub fn load(manifest_path: String, reg: &NeocivRegistry, runtime: &NeocivRuntime) {
    // Get the base directory of the manifest and locate package/index.(lua|fnl|cvl) to run
    let base_dir = Path::new(manifest_path.as_str()).parent().unwrap();
    let pkg_dir = base_dir.join("package");

    for ext in vec!["lua", "fnl", "cvl"] {
        let index_path = pkg_dir.join(format!("index.{}", ext));

        if index_path.exists() {
            println!("Running {}", index_path.display());

            // Run the provided content index file
            match runtime.dofile::<()>(index_path.display().to_string().as_str()) {
                Ok(_) => println!("Going great"),
                Err(ex) => panic!("{:?}", ex),
            }

            // Break out of the loop as an index file has been found
            break;
        }
    }
}
