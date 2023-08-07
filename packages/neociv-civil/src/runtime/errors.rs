use neociv_state::errors::StateError;
use rlua::Error as LuaError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum NeocivRuntimeError {
    LuaError(LuaError),
    StateError(StateError),
    FileNotFound,
    UnknownFileType,
}

impl Error for NeocivRuntimeError {}

impl Display for NeocivRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<LuaError> for NeocivRuntimeError {
    fn from(err: LuaError) -> Self {
        NeocivRuntimeError::LuaError(err)
    }
}

impl From<StateError> for NeocivRuntimeError {
    fn from(err: StateError) -> Self {
        NeocivRuntimeError::StateError(err)
    }
}
