use neociv_db::{exec_stmt, NeocivDB};

#[test]
pub fn large_grid() {
    let mut db = NeocivDB::default();

    let xsize = 255;
    let ysize = 255;

    // Create the definition
    assert!(exec_stmt!(db, "set_grid", xsize, ysize).is_ok());

    let res: Result<i32, rusqlite::Error> =
        db.query_row("SELECT COUNT(*) FROM cells", [], |row| row.get(0));

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), xsize * ysize);
}

