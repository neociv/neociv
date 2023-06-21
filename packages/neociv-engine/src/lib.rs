use neociv_civil::runtime::NeocivRuntime;
use neociv_db::NeocivDB;
use neociv_state::{errors::StateError, state::NeocivState};

pub mod state;

use crate::state::*;

pub struct NeocivEngine {
    state: NeocivState,
    runtime: NeocivRuntime,
    db: NeocivDB,
}

impl Default for NeocivEngine {
    fn default() -> Self {
        Self {
            state: NeocivState::default(),
            runtime: NeocivRuntime::default(),
            db: NeocivDB::default(),
        }
    }
}

impl From<&str> for NeocivEngine {
    fn from(path: &str) -> Self {
        Self {
            db: path.into(),
            ..Default::default()
        }
    }
}

impl NeocivEngine {
    pub fn update_state(&mut self) -> Result<&Self, StateError> {
        self.state = update_state(&self.state, &self.db)?;
        Ok(self)
    }

    pub fn update_state_prop(&mut self, prop: &str) -> Result<&Self, StateError> {
        self.state = update_state_prop(prop, &self.state, &self.db)?;
        Ok(self)
    }

    pub fn update_state_props(&mut self, props: Vec<&str>) -> Result<&Self, StateError> {
        self.state = update_state_props(props, &self.state, &self.db)?;
        Ok(self)
    }
}
