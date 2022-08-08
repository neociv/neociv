use neociv_civil::cvl;

#[test]
fn load_file() {
    let lua = cvl::init();
    let result = cvl::load_file(&lua, String::from("hello"));
    assert!(result.is_ok());
}

#[test]
fn load_lua_str() {
    let lua = cvl::init();
    let result = cvl::load_lua_str(&lua, String::from("1+1"));
    assert!(result.is_ok());
}

#[test]
fn load_cvl_str() {
    let lua = cvl::init();
    let result = cvl::load_cvl_str(&lua, String::from("(+ 1 1)"));
    assert!(result.is_ok());
}
