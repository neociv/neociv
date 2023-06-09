use neociv_db::NeocivDB;

#[test]
fn create_new() {
    assert!(NeocivDB::new(":memory:").is_ok())
}