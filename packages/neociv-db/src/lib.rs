use rusqlite::{Connection, Statement, ToSql};
use rusqlite_migration::{Migrations, M};

use std::collections::HashMap;

pub mod connection;
pub mod errors;
pub mod macros;
pub mod types;
pub mod utils;

use crate::connection::NeocivDBConnection;

#[derive(Debug)]
pub struct NeocivDB {}

impl NeocivDB {
    /// Create a new NeocivDB instance with a fully migrated database and full suite of prepared
    /// statements.
    pub fn new<'db>() -> Connection {
        // Create the blank in-memory connection
        let mut connection = Connection::open_in_memory().unwrap();

        // Migrate the connection's database
        {
            let c = &mut connection;
            utils::migrate(c).unwrap();
        }

        {
            let c = &mut connection;
            Self::prep(c);
        }

        connection
    }

    pub fn prep<'db>(conn: &'db Connection) -> types::PrepMap<'db> {
        utils::preps(conn).unwrap()
    }

    /// Execute a prepared statement that is *not* expected to return a result beyond success / fail
    pub fn exec_stmt(
        preps: &mut types::PrepMap<'static>,
        id: &str,
        params: &[&dyn ToSql],
    ) -> types::ExecResult {
        if preps.contains_key(id) {
            match preps.get_mut(id).unwrap().execute(params) {
                Ok(s) => Ok(s),
                Err(e) => Err(errors::Error::SqliteError(e)),
            }
        } else {
            Err(errors::Error::UnknownStatementError)
        }
    }
}

/*
impl<'db> Default for &mut NeocivDB<'db> {
    fn default() -> &'db mut NeocivDB<'db> {
        NeocivDB::new(":memory:")
    }
}
*/

/*
impl<'db> From<&str> for &'db NeocivDB<'db> {
    fn from(value: &str) -> &'db NeocivDB<'db> {
        NeocivDB::new(value)
    }
}
*/
