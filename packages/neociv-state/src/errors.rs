use std::fmt;
use strum::Display;

#[derive(Display, Debug, PartialEq)]
pub enum StateErrorCode {
    UnknownEngineAction,
    DuplicateCivId,
    InvalidCivId,
    UnknownCivId,
    DuplicateCivKey,
    InvalidCivKey,
    UnknownCivKey,
    GridAlreadyDefined,
    CellOutOfBounds,
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
        write!(
            f,
            "StateError {{ code: \"{}\", message: \"{}\" }}",
            self.code, self.message
        )
    }
}

/// Quickly generate a StateError with a formatted message
#[macro_export]
macro_rules! state_error {
    ($code:expr, $($msg:tt),*) => {
        StateError { code: $code, message: format!($($msg),*) }
    }
}
pub(crate) use state_error;

#[macro_export]
macro_rules! err_unknown_engine_action {
    ($action: expr) => {
        state_error!(
            StateErrorCode::UnknownEngineAction,
            "Unknown Neociv Engine Action \"{}\"",
            $action
        )
    }
}
pub(crate) use err_unknown_engine_action;

#[macro_export]
macro_rules! err_dup_civ_id {
    ($id:expr) => {
        state_error!(
            StateErrorCode::DuplicateCivId,
            "CivId \"{}\" already exists",
            $id
        )
    };
}
pub(crate) use err_dup_civ_id;

#[macro_export]
macro_rules! err_invalid_civ_id {
    ($id:expr) => {
        state_error!(StateErrorCode::InvalidCivId, "CivId \"{}\" is invalid", $id)
    };
}
pub(crate) use err_invalid_civ_id;

#[macro_export]
macro_rules! err_unknown_civ_id {
    ($id:expr) => {
        state_error!(StateErrorCode::UnknownCivId, "CivId \"{}\" is unknown", $id)
    };
}
pub(crate) use err_unknown_civ_id;

#[macro_export]
macro_rules! err_dup_civ_key {
    ($key:expr) => {
        state_error!(
            StateErrorCode::DuplicateCivKey,
            "CivKey \"{}\" already exists",
            $key
        )
    };
}
pub(crate) use err_dup_civ_key;

#[macro_export]
macro_rules! err_invalid_civ_key {
    ($key:expr) => {
        state_error!(
            StateErrorCode::InvalidCivKey,
            "CivKey \"{}\" is invalid",
            $key
        )
    };
}
pub(crate) use err_invalid_civ_key;

#[macro_export]
macro_rules! err_unknown_civ_key {
    ($key:expr) => {
        state_error!(
            StateErrorCode::UnknownCivKey,
            "CivKey \"{}\" is unknown",
            $key
        )
    };
}
pub(crate) use err_unknown_civ_key;

#[macro_export]
macro_rules! err_grid_already_defined {
    () => {
        state_error!(
            StateErrorCode::GridAlreadyDefined,
            "Cannot resize a grid that already has cells"
        )
    };
}
pub(crate) use err_grid_already_defined;

#[macro_export]
macro_rules! err_cell_out_of_bounds {
    ($axis:expr, $val:expr, $max: expr) => {
        state_error!(
            StateErrorCode::CellOutOfBounds,
            "Cell {0} = {1} exceeds grid max {0} = {2}",
            $axis,
            $val,
            $max
        )
    };
}
pub(crate) use err_cell_out_of_bounds;
