use rusqlite::{Connection, Params, Row, ToSql};

pub mod errors;
pub mod macros;
pub mod types;
pub mod utils;

use crate::errors::Error as DBError;

#[derive(Debug)]
pub struct NeocivDB {
    connection: Connection,
    statements: types::PrepMap,
}

impl NeocivDB {
    /// Create a new NeocivDB instance with a fully migrated database and full suite of prepared
    /// statements.
    pub fn new(path: &str) -> Result<Self, DBError> {
        // Create the blank in-memory connection
        let mut connection = utils::connect(":memory:")?;

        // If a path is provided then read & load that into the in-memory db before closing it.
        if !path.eq(":memory:") {
            let src = utils::connect(path)?;
            utils::overwrite(&src, &mut connection, None)?;
            utils::close(src)?;
        }

        // Migrate the database regardless of if the optionally loaded database has already been
        // migrated.
        utils::migrate(&mut connection)?;

        // Create and store the cached statements.
        let statements = utils::prep(&connection)?;

        Ok(Self {
            connection,
            statements,
        })
    }

    /// Save the in-memory database
    pub fn save(&self, path: &str) -> types::SaveResult {
        utils::save(&self.connection, path)
    }

    /// Perform a one-time query quickly across rows
    #[inline]
    pub fn query_row<T, P, F>(&self, query: &str, params: P, f: F) -> Result<T, rusqlite::Error>
    where
        P: Params,
        F: FnOnce(&Row<'_>) -> Result<T, rusqlite::Error>,
    {
        self.connection.query_row(query, params, f)
    }

    /// Execute a prepared statement that is *not* expected to return a result beyond success / fail
    pub fn exec_stmt(&mut self, id: &str, params: &[&dyn ToSql]) -> types::ExecResult {
        if self.statements.contains_key(id) {
            match self
                .connection
                .execute(self.statements.get_mut(id).unwrap(), params)
            {
                Ok(s) => Ok(s),
                Err(e) => Err(errors::Error::SqliteError(e)),
            }
        } else {
            Err(DBError::UnknownStatementError)
        }
    }

    /*
    /// Execute a prepared query and get the result
    pub fn exec_query<T>(&mut self, id: &str, params: &[&dyn ToSql]) -> Result<T, DBError> {
        if self.statements.contains_key(id) {
            match self
                .connection
                .execute(self.statements.get_mut(id).unwrap(), params)
            {
                Ok(s) => Ok(s),
                Err(e) => Err(errors::Error::SqliteError(e)),
            }
        } else {
            Err(DBError::UnknownStatementError)
        }
    }
    */
}

impl Default for NeocivDB {
    fn default() -> NeocivDB {
        NeocivDB::new(":memory:").unwrap()
    }
}

impl From<&str> for NeocivDB {
    fn from(value: &str) -> NeocivDB {
        NeocivDB::new(value).unwrap()
    }
}
