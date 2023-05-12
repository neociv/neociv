use rusqlite::Connection;

pub mod errors;
pub mod types;
pub mod utils;

pub struct NeocivDB {
    connection: Connection,
}

impl NeocivDB {
    fn new(path: &str) -> Self {
        Self::from(path)
    }

    fn migrate(&mut self) -> types::MigrationResult {
        utils::migrate(&mut self.connection)
    }

    fn close(self) -> types::CloseResult {
        utils::close(self.connection)
    }

    fn erase(&mut self) -> types::EraseResult {
        utils::erase(&mut self.connection)
    }

    fn save(self, path: &str) -> types::SaveResult {
        utils::save(&self.connection, path)
    }
}

impl Default for NeocivDB {
    fn default() -> Self {
        NeocivDB::from(":memory:")
    }
}

impl From<&str> for NeocivDB {
    fn from(value: &str) -> Self {
        let mut s = Self {
            connection: utils::connect(value).unwrap(),
        };
        s.migrate().unwrap();
        return s;
    }
}
