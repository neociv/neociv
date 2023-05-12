use neociv_civil::runtime::NeocivRuntime;
use neociv_db::NeocivDB;
use neociv_state::state::NeocivState;

pub struct NeocivEngine {
    state: NeocivState,
    runtime: NeocivRuntime,
    db: NeocivDB,
}

impl NeocivEngine {}

impl Default for NeocivEngine {
    fn default() -> Self {
        Self {
            state: NeocivState::default(),
            runtime: NeocivRuntime::default(),
            db: NeocivDB::default(),
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
