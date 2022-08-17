use rlua::{Error as LuaError, FromLuaMulti};
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CvlError {
    LuaError(LuaError),
    FileNotFound,
    UnknownFileType,
}

impl Error for CvlError {}

impl Display for CvlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait NeocivLuaRuntime {
    fn init() -> Result<Self, LuaError>
    where
        Self: Sized;

    fn dofile(&self, filepath: &str) -> Result<&Self, CvlError>
    where
        Self: Sized;

    fn dofile_lua(&self, filepath: &str) -> Result<&Self, LuaError>
    where
        Self: Sized;

    fn eval_lua<R: for<'lua> FromLuaMulti<'lua>>(&self, lua_str: &str) -> Result<R, LuaError>
    where
        Self: Sized;

    fn compile_fnl(&self, fnl_str: &str) -> Result<String, LuaError>;

    fn dofile_fnl(&self, filepath: &str) -> Result<&Self, LuaError>
    where
        Self: Sized;

    fn eval_fnl<R: for<'lua> FromLuaMulti<'lua>>(&self, fnl_str: &str) -> Result<R, LuaError>
    where
        Self: Sized;
}
