use neociv_civil::cvl;

#[test]
fn lua_require_lua() {
    let lua = cvl::init().unwrap();
    assert_eq!(
        cvl::eval::<u8>(&lua, "require(\"./tests/resources/include1\").example()").unwrap(),
        42
    );
}

#[test]
fn lua_require_fnl() {
    let lua = cvl::init().unwrap();
    assert_eq!(
        cvl::eval::<u8>(&lua, "require(\"./tests/resources/include2\").example()").unwrap(),
        42
    );
}

#[test]
fn lua_require_cvl() {
    let lua = cvl::init().unwrap();
    assert_eq!(
        cvl::eval::<u8>(&lua, "require(\"./tests/resources/include3\").example()").unwrap(),
        42
    );
}

#[test]
fn fnl_require_lua() {
    let lua = cvl::init().unwrap();
    assert_eq!(
        cvl::eval_cvl::<u8>(
            &lua,
            "(local include1 (require \"./tests/resources/include1\"))(include1.example)"
        )
        .unwrap(),
        42
    );
}

#[test]
fn fnl_require_fnl() {
    let lua = cvl::init().unwrap();
    assert_eq!(
        cvl::eval_cvl::<u8>(
            &lua,
            "(local include2 (require \"./tests/resources/include2\"))(include2.example)"
        )
        .unwrap(),
        42
    );
}

#[test]
fn fnl_require_cvl() {
    let lua = cvl::init().unwrap();
    assert_eq!(
        cvl::eval_cvl::<u8>(
            &lua,
            "(local include3 (require \"./tests/resources/include3\"))(include3.example)"
        )
        .unwrap(),
        42
    );
}
