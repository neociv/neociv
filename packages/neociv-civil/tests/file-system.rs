use neociv_civil::runtime::NeocivRuntime;

#[test]
fn lua_require_lua() {
    let cvl = NeocivRuntime::default();
    assert_eq!(
        cvl.eval_lua::<u8>("require(\"./tests/resources/include1\").example()")
            .unwrap(),
        42
    );
}

#[test]
fn lua_require_fnl() {
    let cvl = NeocivRuntime::default();
    assert_eq!(
        cvl.eval_lua::<u8>("require(\"./tests/resources/include2\").example()")
            .unwrap(),
        42
    );
}

#[test]
fn lua_require_cvl() {
    let cvl = NeocivRuntime::default();
    assert_eq!(
        cvl.eval_lua::<u8>("require(\"./tests/resources/include3\").example()")
            .unwrap(),
        42
    );
}

#[test]
fn fnl_require_lua() {
    let cvl = NeocivRuntime::default();
    assert_eq!(
        cvl.eval_fnl::<u8>(
            "(local include1 (require \"./tests/resources/include1\"))(include1.example)"
        )
        .unwrap(),
        42
    );
}

#[test]
fn fnl_require_fnl() {
    let cvl = NeocivRuntime::default();
    assert_eq!(
        cvl.eval_fnl::<u8>(
            "(local include2 (require \"./tests/resources/include2\"))(include2.example)"
        )
        .unwrap(),
        42
    );
}

#[test]
fn fnl_require_cvl() {
    let cvl = NeocivRuntime::default();
    assert_eq!(
        cvl.eval_fnl::<u8>(
            "(local include3 (require \"./tests/resources/include3\"))(include3.example)"
        )
        .unwrap(),
        42
    );
}
