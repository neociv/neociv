use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

#[derive(Debug)]
pub enum DBError {
    ConnectionError(rusqlite::Error),
    MigrationError(rusqlite_migration::Error),
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

/// This is the actual interface to loading as it will get the connection, perform the migration, and
/// only then will it hand back the connection.
pub fn load(path: &str) -> Result<Connection, DBError> {
    // Attempt to create the connection
    let conn = connect_db(path);

    // Confirm the connection is good
    if conn.is_ok() {
        // Run the migrations on the connection - create a mutable Connection reference
        let mut mconn = conn.unwrap();

        // If the migrations are okay *then* return the database connection
        return match migrate_db(&mut mconn) {
            Ok(_) => Ok(mconn),
            Err(e) => Err(DBError::MigrationError(e)),
        };
    } else {
        return Err(DBError::ConnectionError(conn.unwrap_err()));
    }
}

/// High-level close that makes sure that the state is nicely cleaned up if need be.
/// Does not currently do anything other than `rusqlite::Connection::close`
pub fn close(conn: Connection) -> Result<(), (Connection, rusqlite::Error)> {
    conn.close()
}
