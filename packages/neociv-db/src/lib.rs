use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap};

use rusqlite::{Connection, Statement, ToSql};

pub mod errors;
pub mod macros;
pub mod types;
pub mod utils;

use crate::errors::Error as DBError;

pub struct NeocivDB<'db> {
    connection: Connection,
    preps: HashMap<String, Statement<'db>>,
}

impl<'db> NeocivDB<'db> {
    pub fn new(path: &str) -> Result<NeocivDB<'db>, DBError> {
        let s = Self {
            connection: utils::connect(path)?,
            preps: HashMap::<String, Statement<'db>>::default(),
        };

        {
            // Create a container to cleanly for each setup function
            let rcdb = RefCell::<&NeocivDB>::new(&s);

            // Migrate the database to the latest version - this should occur even if already
            // migrated to ensure complicity with schema.
            rcdb.borrow_mut().migrate()?;

            // Setup statements
            rcdb.borrow_mut().setup_stmts()?;
        }

        return Ok(s);
    }

    fn migrate(&mut self) -> Result<(), DBError> {
        utils::migrate(&mut self.connection)?;
        Ok(())
    }

    /*
    fn close(&mut self) -> types::CloseResult {
        self.preps.clear();
        utils::close(self.connection.into())
    }

    fn erase(&mut self) -> types::EraseResult {
        self.preps.clear();
        let conn = &mut self.connection;
        utils::erase(conn)
    }

    pub fn save(&self, path: &str) -> types::SaveResult {
        utils::save(&self.connection, path)
    }
    */

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

    /// Execute a prepared statement that is *not* expected to return a result beyond success / fail
    pub fn exec_stmt(&mut self, id: &str, params: &[&dyn ToSql]) -> types::ExecResult {
        if self.preps.contains_key(id) {
            match self.preps.get_mut(id).unwrap().execute(params) {
                Ok(s) => Ok(s),
                Err(e) => Err(errors::Error::SqliteError(e)),
            }
        } else {
            Err(errors::Error::UnknownStatementError)
        }
    }

    fn setup_stmts(&'db mut self) -> Result<(), DBError> {
        let preps = &mut self.preps;

        // Clear out the prepared statements
        preps.clear();

        macro_rules! stmt {
            ($id: literal) => {
                preps
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
