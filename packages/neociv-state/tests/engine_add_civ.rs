use neociv_state::*;

#[test]
fn add_civ() {
    let mut state: state::NeocivState = engine::init();
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.revision, 0);
    state = match engine::add_civ(
        state,
        civ::Civ {
            id: String::from("example"),
            title: String::from("Example"),
            ..Default::default()
        },
    ) {
            Ok(state) => state,
            Err(ex) => panic!("{:?}", ex),
        };
    assert_eq!(state.civs.len(), 1);
    assert_eq!(state.revision, 1);
}

#[test]
fn err_dup_civ() {
    let mut state: state::NeocivState = engine::init();
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.revision, 0);
    state = match engine::add_civ(
        state,
        civ::Civ {
            id: String::from("example"),
            title: String::from("Example"),
            ..Default::default()
        },
    ) {
            Ok(state) => state,
            Err(ex) => panic!("{:?}", ex),
        };
    assert_eq!(state.civs.len(), 1);
    assert_eq!(state.revision, 1);
    // Add a Civ with the same Id to cause an error
    let error = engine::add_civ(
        state,
        civ::Civ {
            id: String::from("example"),
            title: String::from("Example"),
            ..Default::default()
        },
    );
    assert_eq!(error.is_ok(), false);
    assert_eq!(
        error.unwrap_err().code,
        errors::StateErrorCode::DuplicateCivId
    );
}

