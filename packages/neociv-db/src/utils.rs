use std::time::Duration;

use rusqlite::{backup::Backup, config::DbConfig, Connection};
use rusqlite_migration::{Migrations, M};

use crate::errors::Error as DBError;
use crate::types;

/// Opens a connection to a database, supports both file paths and ":memory:" along
/// with query params.
pub fn connect(path: &str) -> Result<Connection, DBError> {
    match path {
        ":memory:" => Ok(Connection::open_in_memory().unwrap()),
        _ => Ok(Connection::open(path).unwrap()),
    }
}

/// Save the contents of a given database to a file. This will immediately close the file's
/// temporary connection upon completion.
pub fn save(src: &Connection, path: &str) -> types::SaveResult {
    match src.execute("VACUUM INTO ?1", [path]) {
        Ok(_) => Ok(()),
        Err(e) => Err(DBError::SaveError(e)),
    }
}

/// Copies the contents of a given database to another already open connection. This will *not*
/// close the destination's connection nor will it overwrite anything.
pub fn copy(
    src: &Connection,
    dest: &mut Connection,
    progress: types::Progress,
) -> types::CopyResult {
    let backup = Backup::new(src, dest)?;
    backup.run_to_completion(5, Duration::from_millis(250), progress)?;
    Ok(())
}

/// Completely erase the contents of a database but keep the connection *open*
pub fn erase(conn: &mut Connection) -> types::EraseResult {
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE, true)?;
    conn.execute("VACUUM", [])?;
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE, false)?;
    Ok(())
}

/// Migrate the database to the latest schema. This is the *single* source of truth and
/// the only migration function that will ever be called. Can be called safely even
/// on already migrated databases.
pub fn migrate(conn: &mut Connection) -> types::MigrationResult {
    match Migrations::new(vec![
        M::up(include_str!("./migrations/000-setup.sql")),
        M::up(include_str!("./migrations/001-meta.sql")),
        M::up(include_str!("./migrations/002-civs.sql")),
        M::up(include_str!("./migrations/003-grid.sql")),
        M::up(include_str!("./migrations/004-units.sql")),
        M::up(include_str!("./migrations/005-cities.sql")),
        M::up(include_str!("./migrations/006-currencies.sql")),
        M::up(include_str!("./migrations/007-trees.sql")),
    ])
    .to_latest(conn)
    {
        Ok(_) => Ok(conn),
        Err(e) => Err(DBError::MigrationError(e)),
    }
}

/// Erase a destination database and then copy over the contents of the source database.
pub fn overwrite(
    src: &Connection,
    dest: &mut Connection,
    progress: types::Progress,
) -> types::OverwriteResult {
    // Erase the destination DB
    erase(dest)?;

    // Copy over the source database to the (now) empty destination
    copy(src, dest, progress)?;

    Ok(())
}

/// Close a connection - this is extremely basic at the moment but will eventually contain integrity
/// and other checks to make sure the database is in a good state.
pub fn close(conn: Connection) -> types::CloseResult {
    conn.close()
}

/// Takes a given connection and returns a map of all prepared statements.
pub fn prep(conn: &Connection) -> Result<types::PrepMap, DBError> {
    let mut stmts = types::PrepMap::new();

    macro_rules! stmt {
        ($id:literal) => {{
            let stmt = include_str!(concat!("./statements/", $id, ".sql"));
            match conn.prepare_cached(stmt) {
                Ok(_) => Ok(stmts.insert($id.to_string(), stmt.to_string())),
                Err(e) => Err(DBError::PrepareSaveError(e)),
            }
        }?};
    }

    stmt!("def_civ");
    stmt!("add_civ");
    stmt!("remove_civ");
    stmt!("set_grid");

    Ok(stmts)
}
