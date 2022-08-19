use neociv_civil::runtime::NeocivRuntime;
use rlua::{Nil as LuaNil};

#[test]
fn from_lua() {
    let cvl = NeocivRuntime::default();
    let load_result = cvl.dofile_lua::<()>("./tests/resources/key-events.lua");
    //panic!("{:?}", load_result.err());
    assert!(load_result.is_ok());
}

#[test]
fn from_cvl() {
    let cvl = NeocivRuntime::default();
    let load_result = cvl.dofile::<()>("./tests/resources/key-events.cvl");
    //panic!("{:?}", load_result.err());
    assert!(load_result.is_ok());
}
