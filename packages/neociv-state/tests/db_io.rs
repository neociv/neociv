use neociv_state::db;

#[test]
fn copy() {
    let src = db::connect_db(":memory:").unwrap();
    let mut dest = db::connect_db(":memory:").unwrap();
    assert!(db::copy_db(&src, &mut dest, None).is_ok());
    assert!(db::close(src).is_ok());
    assert!(db::close(dest).is_ok());
}

#[test]
fn erase() {
    let conn = &mut db::connect_db(":memory:").unwrap();
    conn.execute("CREATE TABLE example (id VARCHAR PRIMARY KEY)", [])
        .unwrap();

    // Confirm that the new table in the destination exists
    let check_created: Result<i32, rusqlite::Error> = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE name = 'example'",
        [],
        |row| row.get(0),
    );

    assert!(check_created.is_ok());
    assert_eq!(check_created.unwrap(), 1);
    assert!(!conn.is_busy());

    // Erase the entire database
    let check_erase = db::erase_db(conn);

    assert!(check_erase.is_ok());
    assert!(!conn.is_busy());

    // Confirm that the table no longer exists
    let check_erased: Result<i32, rusqlite::Error> = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE name = 'example'",
        [],
        |row| row.get(0),
    );

    assert!(check_erased.is_ok());
    assert_eq!(check_erased.unwrap(), 0);
    assert!(!conn.is_busy());
}

#[test]
fn copy_confirm_overwrite() {
    let src = db::connect_db(":memory:").unwrap();

    // Create a table in the source that we assume will be in the destination
    assert!(src
        .execute("CREATE TABLE src_example ( id VARCHAR PRIMARY_KEY )", [])
        .is_ok());

    let dest = &mut db::connect_db(":memory:").unwrap();

    // Create a table in the destination that we assume will be overwritten
    assert!(dest
        .execute("CREATE TABLE dest_example ( id VARCHAR PRIMARY_KEY )", [])
        .is_ok());

    // Perform the copy / overwrite
    assert!(db::copy_db(&src, dest, None).is_ok());

    // Close the source
    assert!(db::close(src).is_ok());

    // Confirm that the new table in the destination exists
    let check_copied: Result<i32, rusqlite::Error> = dest.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE name = 'src_example'",
        [],
        |row| row.get(0),
    );

    assert!(check_copied.is_ok());
    assert_eq!(check_copied.unwrap(), 1);
    assert!(!dest.is_busy());

    // Confirm that the existing table in the destination was overwritten
    let check_over: Result<i32, rusqlite::Error> = dest.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE name = 'dest_example'",
        [],
        |row| row.get(0),
    );

    assert!(check_over.is_ok());
    assert_eq!(check_over.unwrap(), 0);
}
