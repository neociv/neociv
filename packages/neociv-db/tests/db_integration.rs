use neociv_db::{exec_stmt, NeocivDB};

#[test]
fn create_new() {
    assert!(NeocivDB::new(":memory:").is_ok())
}

#[test]
fn create_civ() {
    let mut db = NeocivDB::default();
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());

    let res: Result<i32, rusqlite::Error> = db.query_row(
        "SELECT COUNT(*) FROM civ_def WHERE id = ?1",
        ["org.neociv.contrib.example"],
        |row| row.get(0),
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn create_civ_entry() {
    let mut db = NeocivDB::default();

    // Create the definition
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());

    let res: Result<i32, rusqlite::Error> = db.query_row(
        "SELECT COUNT(*) FROM civ_def WHERE id = ?1",
        ["org.neociv.contrib.example"],
        |row| row.get(0),
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);

    // Create the entry
    assert!(exec_stmt!(db, "add_civ", "org.neociv.contrib.example").is_ok());

    let res2: Result<i32, rusqlite::Error> = db.query_row(
        "SELECT COUNT(*) FROM civs WHERE id = ?1 AND civ_id = ?2 AND idx = ?3",
        [
            "org.neociv.contrib.example[0]",
            "org.neociv.contrib.example",
            "0",
        ],
        |row| row.get(0),
    );

    assert!(res2.is_ok());
    assert_eq!(res2.unwrap(), 1);
}

#[test]
fn create_civ_entry_multi() {
    let mut db = NeocivDB::default();

    // Create the definition
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());

    let res: Result<i32, rusqlite::Error> = db.query_row(
        "SELECT COUNT(*) FROM civ_def WHERE id = ?1",
        ["org.neociv.contrib.example"],
        |row| row.get(0),
    );

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1);

    // Create the entry
    assert!(exec_stmt!(db, "add_civ", "org.neociv.contrib.example").is_ok());

    let res2: Result<i32, rusqlite::Error> = db.query_row(
        "SELECT COUNT(*) FROM civs WHERE id = ?1 AND civ_id = ?2 AND idx = ?3",
        [
            "org.neociv.contrib.example[0]",
            "org.neociv.contrib.example",
            "0",
        ],
        |row| row.get(0),
    );

    assert!(res2.is_ok());
    assert_eq!(res2.unwrap(), 1);

    // Create an additional but otherwise separate entry for the same civ
    assert!(exec_stmt!(db, "add_civ", "org.neociv.contrib.example").is_ok());

    let res3: Result<i32, rusqlite::Error> = db.query_row(
        "SELECT COUNT(*) FROM civs WHERE id = ?1 AND civ_id = ?2 AND idx = ?3",
        [
            "org.neociv.contrib.example[1]",
            "org.neociv.contrib.example",
            "1",
        ],
        |row| row.get(0),
    );

    assert!(res3.is_ok());
    assert_eq!(res3.unwrap(), 1);
}

#[test]
fn create_civ_fail() {
    let mut db = NeocivDB::default();
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());
    assert!(exec_stmt!(db, "add_civ", "does.not.exist", "NoExist").is_err());
}
