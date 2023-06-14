use neociv_db::{exec_stmt, NeocivDB};

#[test]
pub fn large_grid() {
    let mut db = NeocivDB::default();

    let xsize = 255;
    let ysize = 255;

    // Create the definition
    assert!(exec_stmt!(db, "set_grid", xsize, ysize).is_ok());
    assert_eq!(db.count("cells", []), xsize * ysize);
}
