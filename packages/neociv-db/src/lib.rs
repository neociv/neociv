use std::{borrow::Borrow, collections::HashMap};

use rusqlite::{Connection, Statement, ToSql};

pub mod errors;
pub mod macros;
pub mod types;
pub mod utils;

pub struct NeocivDB<'db> {
    connection: Connection,
    preps: HashMap<String, Statement<'db>>,
}

impl<'db> NeocivDB<'db> {
    fn new(path: &str) -> NeocivDB<'db> {
        let s = Self {
            connection: utils::connect(path).unwrap(),
            preps: HashMap::<String, Statement<'db>>::default(),
        };
        let x = &mut s;
        x.migrate().unwrap();
        x.setup_stmts().unwrap();
        return s;
    }

    fn migrate(&mut self) -> types::MigrationResult {
        utils::migrate(&mut self.connection)
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

    /*
    fn prep_stmt(&'db mut self, id: String, sql: &str) -> types::PrepareResult<'db> {
        // Mildly cheeky workaround here - it doesn't matter if a prepared statement
        // is overwritten (thus returning Some from the insert) so regardless we
        // store the statement happily.
        match self.preps.insert(id, self.borrow_mut().connection.prepare(sql)?) {
            _ => Ok(self),
        }
    }
    */

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

    fn setup_stmts(&'db mut self) -> types::PrepareResult<'db> {
        macro_rules! stmt {
            ($id: literal) => {
                self.preps
                    .insert(
                        $id.to_string(),
                        self.connection.prepare(include_str!(concat!(
                            "./statements/",
                            $id,
                            ".sql"
                        )))?,
                    )
                    .unwrap()
            };
        }

        stmt!("add_civ");
        stmt!("remove_civ");
        Ok(())
    }
}

impl<'db> Default for NeocivDB<'db> {
    fn default() -> NeocivDB<'db> {
        NeocivDB::new(":memory:")
    }
}

/*
impl<'db> From<&str> for &'db NeocivDB<'db> {
    fn from(value: &str) -> &'db NeocivDB<'db> {
        NeocivDB::new(value)
    }
}
*/
