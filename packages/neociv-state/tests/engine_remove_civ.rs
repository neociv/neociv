use neociv_state::*;

#[test]
fn remove_civ() {
    let mut state: state::NeocivState = engine::init();
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.revision, 0);
    state = match engine::add_civ(
        state,
        civ::Civ {
            id: String::from("test.example"),
            title: String::from("Example"),
            ..Default::default()
        },
    ) {
        Ok(state) => state,
        Err(ex) => panic!("{:?}", ex),
    };
    assert_eq!(state.civ_order.len(), 1);
    assert_eq!(state.civs.len(), 1);
    assert_eq!(state.revision, 1);
    state = match engine::remove_civ(state, String::from("test.example[0]")) {
        Ok(state) => state,
        Err(ex) => panic!("{:?}", ex),
    };
    assert_eq!(state.civ_order.len(), 0);
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.revision, 2);
}
