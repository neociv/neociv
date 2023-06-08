use rusqlite::{backup, Connection, Error as RusqliteError, Statement};
use std::collections::HashMap;

use crate::errors::Error as DBError;

pub type PrepMap<'a> = HashMap<String, Statement<'a>>;
pub type Progress = Option<fn(backup::Progress)>;
pub type ConnectionResult = Result<Connection, RusqliteError>;
pub type CloseResult = Result<(), (Connection, RusqliteError)>;
pub type GenericResult = Result<(), DBError>;
pub type SaveResult = GenericResult;
pub type CopyResult = GenericResult;
pub type EraseResult = GenericResult;
pub type OverwriteResult = GenericResult;
pub type MigrationResult<'db> = Result<&'db mut Connection, DBError>;
pub type ExecResult = Result<usize, DBError>;
