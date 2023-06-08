use rusqlite::Connection;

use crate::utils;

#[derive(Debug)]
pub struct NeocivDBConnection {
    pub connection: Connection,
}

impl NeocivDBConnection {
    pub fn new() -> Self {
        Self::new_from(":memory:")
    }

    pub fn new_from(path: &str) -> Self {
        Self {
            connection: utils::connect(path).unwrap(),
        }
    }
}
