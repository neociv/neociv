use neociv_db::utils::{close, connect};

#[test]
fn connect_and_close() {
    let conn = connect(":memory:");
    assert!(conn.is_ok());
    assert!(close(conn.unwrap()).is_ok());
}
