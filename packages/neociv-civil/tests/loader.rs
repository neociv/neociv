use neociv_civil::cvl;

#[test]
fn init() {
    assert!(cvl::init().is_ok());
}

#[test]
fn load_file_no_exist() {
    let lua = cvl::init().unwrap();
    assert!(cvl::load_file(&lua, "does-not-exist").is_err());
}

#[test]
fn load_file_lua() {
    let lua = cvl::init().unwrap();
    assert!(cvl::load_file(&lua, "./tests/resources/example.lua").is_ok());
}

#[test]
fn load_file_fnl() {
    let lua = cvl::init().unwrap();
    assert!(cvl::load_file(&lua, "./tests/resources/example.fnl").is_ok());
}

#[test]
fn load_file_cvl() {
    let lua = cvl::init().unwrap();
    assert!(cvl::load_file(&lua, "./tests/resources/example.cvl").is_ok());
}

#[test]
fn load_lua_str() {
    let lua = cvl::init().unwrap();
    let result = cvl::load_lua_str(&lua, "return (1 + 1)");
    assert!(result.is_ok());
}

#[test]
fn load_cvl_str() {
    let lua = cvl::init().unwrap();
    let result = cvl::load_cvl_str(&lua, "(+ 1 1)");
    assert!(result.is_ok());
}

#[test]
fn eval() {
    let lua = cvl::init().unwrap();
    // String
    assert_eq!(
        cvl::eval::<String>(&lua, "\"Hello World\"").unwrap(),
        "Hello World"
    );
    // Number
    assert_eq!(cvl::eval::<u8>(&lua, "42").unwrap(), 42);
    assert_eq!(cvl::eval::<u8>(&lua, "1 + 1").unwrap(), 2);
}

#[test]
fn compile_cvl() {
    let lua = cvl::init().unwrap();
    assert_eq!(cvl::compile_cvl(&lua, "(+ 1 1)").unwrap(), "return (1 + 1)");
    assert_eq!(
        cvl::compile_cvl(&lua, r#"(local foo (require "path/to/bar"))"#).unwrap(),
        "local foo = require(\"path/to/bar\")\nreturn nil"
    );
}

#[test]
fn eval_cvl() {
    let lua = cvl::init().unwrap();
    // String
    assert_eq!(
        cvl::eval_cvl::<String>(&lua, "\"Hello World\"").unwrap(),
        "Hello World"
    );
    // Number
    assert_eq!(cvl::eval_cvl::<u8>(&lua, "42").unwrap(), 42);
    assert_eq!(cvl::eval_cvl::<u8>(&lua, "(+ 1 1)").unwrap(), 2);
}
