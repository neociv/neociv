use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

pub fn connect_db(path: &str) -> Result<Connection, rusqlite::Error> {
    match path {
        ":memory:" => Connection::open_in_memory(),
        _ => Connection::open(path),
    }
}

pub fn migrate_db(conn: &mut Connection) -> Result<(), rusqlite_migration::Error> {
    let migrations = Migrations::new(vec![
        M::up(include_str!("./migrations/000-setup.sql")),
        M::up(include_str!("./migrations/001-civs.sql")),
        M::up(include_str!("./migrations/002-grid.sql")),
        M::up(include_str!("./migrations/003-units.sql")),
        M::up(include_str!("./migrations/004-cities.sql")),
        M::up(include_str!("./migrations/005-trees.sql")),
    ]);

    migrations.to_latest(conn)
}
