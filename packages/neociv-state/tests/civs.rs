use neociv_state::*;

#[test]
fn test_init() {
    let state: state::NeocivState = engine::init();
    assert_eq!(state.revision, 0);
    assert_eq!(state.turn, 0);
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.grid.cells.len(), 0);
}

#[test]
fn test_add_civ() {
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
fn test_err_dup_civ() {
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

#[test]
fn test_remove_civ() {
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
    state = match engine::remove_civ(state, String::from("example")) {
        Ok(state) => state,
        Err(ex) => panic!("{:?}", ex),
    };
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.revision, 2);
}
