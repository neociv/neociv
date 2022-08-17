use neociv_civil::runtime::lua::{init_lua, NeocivLuaRuntime};

#[test]
fn init() {
    assert!(init_lua().is_ok());
}

#[test]
fn load_file_no_exist() {
    let lua = init_lua().unwrap();
    assert!(lua.dofile("does-not-exist").is_err());
}

#[test]
fn load_file_lua() {
    let lua = init_lua().unwrap();
    assert!(lua.dofile("./tests/resources/example.lua").is_ok());
}

#[test]
fn load_file_fnl() {
    let lua = init_lua().unwrap();
    assert!(lua.dofile("./tests/resources/example.fnl").is_ok());
}

#[test]
fn load_file_cvl() {
    let lua = init_lua().unwrap();
    assert!(lua.dofile("./tests/resources/example.cvl").is_ok());
}

#[test]
fn eval_lua() {
    let lua = init_lua().unwrap();
    // String
    assert_eq!(
        lua.eval_lua::<String>("\"Hello World\"").unwrap(),
        "Hello World"
    );
    // Number
    assert_eq!(lua.eval_lua::<u8>("42").unwrap(), 42);
    assert_eq!(lua.eval_lua::<u8>("1 + 1").unwrap(), 2);
}

#[test]
fn compile_fnl() {
    let lua = init_lua().unwrap();
    assert_eq!(lua.compile_fnl("(+ 1 1)").unwrap(), "return (1 + 1)");
    assert_eq!(
        lua.compile_fnl(r#"(local foo (require "path/to/bar"))"#)
            .unwrap(),
        "local foo = require(\"path/to/bar\")\nreturn nil"
    );
}

#[test]
fn eval_fnl() {
    let lua = init_lua().unwrap();
    // String
    assert_eq!(
        lua.eval_fnl::<String>("\"Hello World\"").unwrap(),
        "Hello World"
    );
    // Number
    assert_eq!(lua.eval_fnl::<u8>("42").unwrap(), 42);
    assert_eq!(lua.eval_fnl::<u8>("(+ 1 1)").unwrap(), 2);
}
