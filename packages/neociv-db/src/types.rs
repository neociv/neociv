use rusqlite::{backup, Connection, Error as RusqliteError, Statement};

use crate::errors::Error;

pub type Progress = Option<fn(backup::Progress)>;
pub type ConnectionResult = Result<Connection, RusqliteError>;
pub type CloseResult = Result<(), (Connection, RusqliteError)>;
pub type GenericResult = Result<(), Error>;
pub type SaveResult = GenericResult;
pub type CopyResult = GenericResult;
pub type EraseResult = GenericResult;
pub type OverwriteResult = GenericResult;
pub type MigrationResult = GenericResult;
pub type PrepareResult = GenericResult;
pub type ExecResult = Result<usize, Error>;