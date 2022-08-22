use rlua::{Error as LuaError};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum NeocivRuntimeError {
    LuaError(LuaError),
    FileNotFound,
    UnknownFileType,
}

impl Error for NeocivRuntimeError {}

impl Display for NeocivRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}