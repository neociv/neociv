use neociv_civil::cvl;

#[test]
fn from_lua() {
    let lua = cvl::init().unwrap();
    let load_result = cvl::load_file(&lua, "./tests/resources/key-events.lua");
    //panic!("{:?}", load_result.err());
    assert!(load_result.is_ok());
}

#[test]
fn from_cvl() {
    let lua = cvl::init().unwrap();
    let load_result = cvl::load_file(&lua, "./tests/resources/key-events.cvl");
    //panic!("{:?}", load_result.err());
    assert!(load_result.is_ok());
}

