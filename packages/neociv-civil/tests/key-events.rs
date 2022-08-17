use neociv_civil::runtime::lua::{init_lua, NeocivLuaRuntime};

#[test]
fn from_lua() {
    let lua = init_lua().unwrap();
    let load_result = lua.dofile_lua("./tests/resources/key-events.lua");
    //panic!("{:?}", load_result.err());
    assert!(load_result.is_ok());
}

#[test]
fn from_cvl() {
    let lua = init_lua().unwrap();
    let load_result = lua.dofile_fnl("./tests/resources/key-events.cvl");
    //panic!("{:?}", load_result.err());
    assert!(load_result.is_ok());
}

