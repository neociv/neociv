use neociv_civil::cvl;

#[test]
fn init() {
    assert!(cvl::init().is_ok());
}

#[test]
fn load_file() {
    let lua = cvl::init().unwrap();
    let result = cvl::load_file(&lua, String::from("hello"));
    assert!(result.is_ok());
}

#[test]
fn load_lua_str() {
    let lua = cvl::init().unwrap();
    let result = cvl::load_lua_str(&lua, String::from("return (1 + 1)"));
    assert!(result.is_ok());
}

#[test]
fn load_cvl_str() {
    let lua = cvl::init().unwrap();
    let result = cvl::load_cvl_str(&lua, String::from("(+ 1 1)"));
    assert!(result.is_ok());
}

#[test]
fn eval() {
    let lua = cvl::init().unwrap();
    // String
    assert_eq!(
        cvl::eval::<String>(&lua, String::from("return \"Hello World\"")).unwrap(),
        String::from("Hello World")
    );
    // Number
    assert_eq!(
        cvl::eval::<u8>(&lua, String::from("return 42")).unwrap(),
        42
    );
}

#[test]
fn compile_cvl() {
    let lua = cvl::init().unwrap();
    assert_eq!(
        cvl::compile_cvl(&lua, String::from("(+ 1 1)")).unwrap(),
        "return (1 + 1)"
    );
}

#[test]
fn eval_cvl() {
    let lua = cvl::init().unwrap();
}
