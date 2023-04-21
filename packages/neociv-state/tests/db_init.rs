use neociv_state::db;

#[test]
fn load_and_close() {
    let conn = db::load(":memory:");
    assert!(conn.is_ok());
    assert!(db::close(conn.unwrap()).is_ok());
}
