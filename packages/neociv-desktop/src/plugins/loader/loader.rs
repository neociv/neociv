use glob;

use neociv_civil::runtime::NeocivRuntime;
use neociv_game::plugins::registry::registry::NeocivRegistry;

pub fn load(manifest_path: String, reg: &NeocivRegistry, runtime: &NeocivRuntime) {
    // Get the base directory of the manifest and locate package/index.(lua|fnl|cvl) to run
}
