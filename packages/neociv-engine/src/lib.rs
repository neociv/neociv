use neociv_civil::runtime::NeocivRuntime;
use neociv_db::NeocivDB;
use neociv_state::state::NeocivState;

pub struct NeocivEngine<'engine> {
    state: NeocivState,
    runtime: NeocivRuntime,
    db: NeocivDB<'engine>,
}

impl<'engine> NeocivEngine<'engine> {
    /*
    fn run_action<'action>(&mut self, action: impl EngineAction<'action>) {
        // TODO: Create a prepared statement
        // TODO: Pass the params and execute
        // TODO: Return success
    }
    */
}

impl<'engine> Default for NeocivEngine<'engine> {
    fn default() -> Self {
        Self {
            state: NeocivState::default(),
            runtime: NeocivRuntime::default(),
            db: NeocivDB::new(":memory:").unwrap(),
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
