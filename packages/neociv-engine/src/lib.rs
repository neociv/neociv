use neociv_civil::runtime::NeocivRuntime;
use neociv_db::NeocivDB;
use neociv_state::state::NeocivState;
use rlua::Value as LuaValue;

pub mod state;

use crate::state::*;

pub enum NeocivEngineError {
    StateError(neociv_state::errors::StateError),
    RuntimeError(rlua::Error),
}

impl From<neociv_state::errors::StateError> for NeocivEngineError {
    fn from(value: neociv_state::errors::StateError) -> Self {
        NeocivEngineError::StateError(value)
    }
}

impl From<rlua::Error> for NeocivEngineError {
    fn from(value: rlua::Error) -> Self {
        NeocivEngineError::RuntimeError(value)
    }
}

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
    pub fn update_state(&mut self) -> Result<&Self, NeocivEngineError> {
        self.update_state_prop("*")
    }

    pub fn update_state_prop(&mut self, prop: &str) -> Result<&Self, NeocivEngineError> {
        self.update_state_props(vec![prop])
    }

    pub fn update_state_props(&mut self, props: Vec<&str>) -> Result<&Self, NeocivEngineError> {
        self.state = update_state_props(props, &self.state, &self.db)?;
        self.runtime.inject_state(&self.state)?;
        Ok(self)
    }
}
