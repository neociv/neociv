use std::{collections::HashMap, error, borrow::BorrowMut};

use rusqlite::{Connection, Statement, ToSql};

pub mod errors;
pub mod types;
pub mod utils;

pub struct NeocivDB<'db> {
    connection: Connection,
    preps: HashMap<String, Statement<'db>>,
}

impl<'db> NeocivDB<'db> {
    fn new(path: &str) -> Self {
        let mut s = Self {
            connection: utils::connect(path).unwrap(),
            preps: HashMap::<String, Statement<'db>>::default(),
        };
        s.migrate().unwrap();
        return s;
    }

    fn migrate(&mut self) -> types::MigrationResult {
        utils::migrate(&mut self.connection)
        // TODO: Recreate prepared statements
    }

    fn close(self) -> types::CloseResult {
        utils::close(self.connection)
    }

    fn erase(&mut self) -> types::EraseResult {
        self.preps.clear();
        utils::erase(&mut self.connection)
    }

    fn save(&self, path: &str) -> types::SaveResult {
        utils::save(&self.connection, path)
    }

    fn prep_stmt(&'db mut self, id: String, sql: &str) -> types::PrepareResult {
        // Mildly cheeky workaround here - it doesn't matter if a prepared statement
        // is overwritten (thus returning Some from the insert) so regardless we
        // store the statement happily.
        match self.preps.insert(id, self.connection.prepare(sql)?) {
            _ => Ok(()),
        }
    }

    fn exec_stmt(&mut self, id: &str, params: &[&dyn ToSql]) -> types::ExecResult {
        if self.preps.contains_key(id) {
            match self.preps.get_mut(id).unwrap().execute(params) {
                Ok(s) => Ok(s),
                Err(e) => Err(errors::Error::SqliteError(e)),
            }
        } else {
            Err(errors::Error::UnknownStatementError)
        }
    }

    /*fn query_stmt(&'db mut self) -> void {}*/
}

impl<'db> Default for NeocivDB<'db> {
    fn default() -> Self {
        NeocivDB::new(":memory:")
    }
}

impl<'db> From<&str> for NeocivDB<'db> {
    fn from(value: &str) -> Self {
        NeocivDB::new(value)
    }
}
