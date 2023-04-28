use rusqlite::{backup, Connection};

use crate::{db, state::NeocivState};

pub struct Engine {
    pub db: Connection,
    pub state: NeocivState,
    // TODO: Runtime
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            db: db::connect_db(":memory:").unwrap(),
            state: NeocivState::default(),
            // TODO: Runtime
        }
    }
}

impl Engine {
    /// Save the current state as a separate DB file
    pub fn save_state(
        &self,
        path: &str,
        progress: Option<fn(backup::Progress)>,
    ) -> Result<(), db::DBError> {
        match db::save_db(&self.db, path, progress) {
            Ok(_) => Ok(()),
            Err(e) => Err(db::DBError::from(e)),
        }
    }

    /// Load the state from a database file and inject it into ":memory:" - this will attempt to perform a migration
    /// on the loaded data.
    pub fn load_state(&mut self, path: &str, progress: Option<fn(backup::Progress)>) -> Result<(), db::DBError> {
        db::copy_db(&db::connect_db(path)?, &mut self.db, progress)?;
        Ok(())
    }

    /// Perform an action on the state
    pub fn run(&self) -> Result<(), ()> {
        Ok(())
    }

    /// Refreshes a particular property via triggering a transaction in the database to get new information
    pub fn refresh_props(&self, target: &str) -> Result<(), ()> {
        Ok(())
    }
}
