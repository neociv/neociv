use std::fmt;
use strum::Display;

#[derive(Display, Debug, PartialEq)]
pub enum StateErrorCode {
    DuplicateCivId,
    InvalidCivId,
    UnknownCivId,
}

pub struct StateError {
    pub code: StateErrorCode,
    pub message: String,
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for StateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateError {{ code: \"{}\", message: \"{}\" }}", self.code, self.message)
    }
}

#[macro_export]
macro_rules! state_error {
    ($code:expr, $msg:expr) => {
        StateError { code: $code, message: $msg }
    }
}
pub(crate) use state_error;

#[macro_export]
macro_rules! err_dup_civ {
    ($id:expr) => {
        return Err(state_error!(StateErrorCode::DuplicateCivId, format!("CivId \"{}\" already exists", $id)));
    }
}
pub(crate) use err_dup_civ;

#[macro_export]
macro_rules! err_invalid_civ {
    ($id:expr) => {
        return Err(state_error!(StateErrorCode::InvalidCivId, format!("CivId \"{}\" is invalid", $id)));
    }
}
pub(crate) use err_invalid_civ;

#[macro_export]
macro_rules! err_unknown_civ {
    ($id:expr) => {
        return Err(state_error!(StateErrorCode::UnknownCivId, format!("CivId \"{}\" is unknown", $id)));
    }
}
pub(crate) use err_unknown_civ;
