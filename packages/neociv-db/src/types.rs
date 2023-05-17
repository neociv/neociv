use rusqlite::{backup, Connection, Error as RusqliteError};

use crate::{errors::Error, NeocivDB};

pub type Progress = Option<fn(backup::Progress)>;
pub type ConnectionResult = Result<Connection, RusqliteError>;
pub type CloseResult = Result<(), (Connection, RusqliteError)>;
pub type GenericResult = Result<(), Error>;
pub type SaveResult = GenericResult;
pub type CopyResult = GenericResult;
pub type EraseResult = GenericResult;
pub type OverwriteResult = GenericResult;
pub type MigrationResult = GenericResult;
pub type PrepareResult<'db> = Result<(), Error>;
pub type ExecResult = Result<usize, Error>;
