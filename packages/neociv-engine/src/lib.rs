use neociv_civil::runtime::NeocivRuntime;
use neociv_db::NeocivDB;
use neociv_state::state::NeocivState;

pub struct NeocivEngine {
    state: NeocivState,
    runtime: NeocivRuntime,
}

impl NeocivEngine {
    /*
    fn run_action<'action>(&mut self, action: impl EngineAction<'action>) {
        // TODO: Create a prepared statement
        // TODO: Pass the params and execute
        // TODO: Return success
    }
    */
}

impl Default for NeocivEngine {
    fn default() -> Self {
        Self {
            state: NeocivState::default(),
            runtime: NeocivRuntime::default(),
        }
    }
}

/*
impl From<&str> for NeocivEngine {
    fn from(path: &str) -> Self {
        Self {}
    }
}
*/
