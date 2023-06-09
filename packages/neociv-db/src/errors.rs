use rusqlite::{Connection, Error as RusqliteError};
use rusqlite_migration::Error as RusqliteMigrationError;

#[derive(Debug)]
pub enum Error {
    Unknown,
    PrepareSaveError(RusqliteError),
    UnknownStatementError,
    SaveError(RusqliteError),
    SqliteError(RusqliteError),
    MigrationError(RusqliteMigrationError),
}

impl From<RusqliteError> for Error {
    fn from(value: RusqliteError) -> Self {
        match value.sqlite_error_code() {
            Some(_) => Self::SqliteError(value),
            None => Self::Unknown,
        }
    }
}

impl From<RusqliteMigrationError> for Error {
    fn from(value: RusqliteMigrationError) -> Self {
        Self::MigrationError(value)
    }
}

impl From<(Connection, RusqliteError)> for Error {
    fn from(value: (Connection, RusqliteError)) -> Self {
        Self::from(value.1)
    }
}
