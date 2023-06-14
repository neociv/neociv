use neociv_db::{exec_stmt, NeocivDB};

#[test]
fn create_new() {
    assert!(NeocivDB::new(":memory:").is_ok())
}

#[test]
fn create_civ() {
    let mut db = NeocivDB::default();
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());
    assert_eq!(
        db.count("civ_def WHERE id = ?1", ["org.neociv.contrib.example"]),
        1
    );
}

#[test]
fn create_civ_entry() {
    let mut db = NeocivDB::default();

    // Create the definition
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());
    assert_eq!(
        db.count("civ_def WHERE id = ?1", ["org.neociv.contrib.example"]),
        1
    );

    // Create the entry
    assert!(exec_stmt!(db, "add_civ", "org.neociv.contrib.example").is_ok());
    assert_eq!(
        db.count(
            "civs WHERE id = ?1 AND civ_id = ?2 AND idx = ?3",
            [
                "org.neociv.contrib.example[0]",
                "org.neociv.contrib.example",
                "0",
            ]
        ),
        1
    );
}

#[test]
fn create_civ_entry_multi() {
    let mut db = NeocivDB::default();

    // Create the definition
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());

    // Confirm the definition exists
    assert_eq!(
        db.count("civ_def WHERE id = ?1", ["org.neociv.contrib.example"]),
        1
    );

    // Create the entry
    assert!(exec_stmt!(db, "add_civ", "org.neociv.contrib.example").is_ok());

    // Confirm the entry exists
    assert_eq!(
        db.count(
            "civs WHERE id = ?1 AND civ_id = ?2 AND idx = ?3",
            [
                "org.neociv.contrib.example[0]",
                "org.neociv.contrib.example",
                "0",
            ]
        ),
        1
    );

    // Create an additional but otherwise separate entry for the same civ
    assert!(exec_stmt!(db, "add_civ", "org.neociv.contrib.example").is_ok());

    // Confirm the entry exists
    assert_eq!(
        db.count(
            "civs WHERE id = ?1 AND civ_id = ?2 AND idx = ?3",
            [
                "org.neociv.contrib.example[1]",
                "org.neociv.contrib.example",
                "1",
            ]
        ),
        1
    );

    // Confirm that two entries now exist for the same civ def
    assert_eq!(
        db.count("civs WHERE civ_id = ?1", ["org.neociv.contrib.example"]),
        2
    );
}

#[test]
fn create_civ_fail() {
    let mut db = NeocivDB::default();
    assert!(exec_stmt!(db, "def_civ", "org.neociv.contrib.example", "Example").is_ok());
    assert!(exec_stmt!(db, "add_civ", "does.not.exist", "NoExist").is_err());
}
