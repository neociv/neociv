use neociv_civil::runtime::NeocivRuntime;

#[test]
fn dofile_noexist() {
    let cvl = NeocivRuntime::default();
    assert!(cvl.dofile::<()>("does-not-exist").is_err());
}

#[test]
fn load_file_lua() {
    let cvl = NeocivRuntime::default();
    assert!(cvl.dofile::<()>("./tests/resources/example.lua").is_ok());
}

#[test]
fn load_file_fnl() {
    let cvl = NeocivRuntime::default();
    assert!(cvl.dofile::<()>("./tests/resources/example.fnl").is_ok());
}

#[test]
fn load_file_cvl() {
    let cvl = NeocivRuntime::default();
    assert!(cvl.dofile::<()>("./tests/resources/example.cvl").is_ok());
}

#[test]
fn eval_lua() {
    let cvl = NeocivRuntime::default();
    // String
    assert_eq!(
        cvl.eval_lua::<String>("\"Hello World\"").unwrap(),
        "Hello World"
    );
    // Number
    assert_eq!(cvl.eval_lua::<u8>("42").unwrap(), 42);
    assert_eq!(cvl.eval_lua::<u8>("1 + 1").unwrap(), 2);
}

#[test]
fn compile_fnl() {
    let cvl = NeocivRuntime::default();
    assert_eq!(cvl.compile_fnl("(+ 1 1)").unwrap(), "return (1 + 1)");
    assert_eq!(
        cvl.compile_fnl(r#"(local foo (require "path/to/bar"))"#)
            .unwrap(),
        "local foo = require(\"path/to/bar\")\nreturn nil"
    );
}

#[test]
fn eval_fnl() {
    let cvl = NeocivRuntime::default();
    // String
    assert_eq!(
        cvl.eval_fnl::<String>("\"Hello World\"").unwrap(),
        "Hello World"
    );
    // Number
    assert_eq!(cvl.eval_fnl::<u8>("42").unwrap(), 42);
    assert_eq!(cvl.eval_fnl::<u8>("(+ 1 1)").unwrap(), 2);
}
