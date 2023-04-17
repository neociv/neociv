use neociv_state::db;

#[test]
fn connect() {
    assert!(db::connect_db(":memory:").is_ok());
}

#[test]
fn init() {
    let conn = &mut db::connect_db(":memory:").unwrap();
    assert!(db::migrate_db(conn).is_ok());
}
