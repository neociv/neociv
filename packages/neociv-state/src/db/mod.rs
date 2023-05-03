use std::time::Duration;

use rusqlite::{backup, config::DbConfig, params, Connection};
use rusqlite_migration::{Migrations, M};

#[derive(Debug)]
pub enum DBError {
    ConnectionError(rusqlite::Error),
    MigrationError(rusqlite_migration::Error),
}

impl From<rusqlite::Error> for DBError {
    fn from(value: rusqlite::Error) -> Self {
        Self::ConnectionError(value)
    }
}

impl From<rusqlite_migration::Error> for DBError {
    fn from(value: rusqlite_migration::Error) -> Self {
        Self::MigrationError(value)
    }
}

impl From<(Connection, rusqlite::Error)> for DBError {
    fn from(value: (Connection, rusqlite::Error)) -> Self {
        Self::ConnectionError(value.1)
    }
}

pub fn connect_db(path: &str) -> Result<Connection, rusqlite::Error> {
    match path {
        ":memory:" => Connection::open_in_memory(),
        _ => Connection::open(path),
    }
}

pub fn migrate_db(conn: &mut Connection) -> Result<&mut Connection, rusqlite_migration::Error> {
    let migrations = Migrations::new(vec![
        M::up(include_str!("./migrations/000-setup.sql")),
        M::up(include_str!("./migrations/001-civs.sql")),
        M::up(include_str!("./migrations/002-grid.sql")),
        M::up(include_str!("./migrations/003-units.sql")),
        M::up(include_str!("./migrations/004-cities.sql")),
        M::up(include_str!("./migrations/005-currencies.sql")),
        M::up(include_str!("./migrations/006-trees.sql")),
    ]);

    match migrations.to_latest(conn) {
        Ok(_) => Ok(conn),
        Err(e) => Err(e),
    }
}

pub fn save_db(
    src: &Connection,
    dest: &str,
    progress: Option<fn(backup::Progress)>,
) -> Result<(), rusqlite::Error> {
    let mut dest = connect_db(dest)?;
    let backup = backup::Backup::new(src, &mut dest)?;
    backup.run_to_completion(5, Duration::from_millis(250), progress)
}

pub fn copy_db(
    src: &Connection,
    dest: &mut Connection,
    progress: Option<fn(backup::Progress)>,
) -> Result<(), rusqlite::Error> {
    erase_db(dest)?;
    backup::Backup::new(src, dest)?.run_to_completion(5, Duration::from_millis(250), progress)
}

// Erase a database completely but keep the connection open
pub fn erase_db(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE, true)?;
    conn.execute("VACUUM", [])?;
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE, false)?;
    Ok(())
}

/// This is the actual interface to loading as it will get the connection, perform the migration, and
/// only then will it hand back the connection.
pub fn load(path: &str) -> Result<Connection, DBError> {
    // Attempt to create the connection
    let mut conn = connect_db(path)?;

    // If the migrations are okay *then* return the database connection
    return match migrate_db(&mut conn) {
        Ok(_) => Ok(conn),
        Err(e) => Err(DBError::MigrationError(e)),
    };
}

/// High-level close that makes sure that the state is nicely cleaned up if need be.
/// Does not currently do anything other than `rusqlite::Connection::close`
pub fn close(conn: Connection) -> Result<(), (Connection, rusqlite::Error)> {
    conn.close()
}
