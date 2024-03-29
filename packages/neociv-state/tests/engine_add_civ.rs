use neociv_state::civ::Civ;
use neociv_state::engine;
use neociv_state::state::NeocivState;

#[test]
fn add_civ() {
    let mut state: NeocivState = engine::init();
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.revision, 0);
    state = match engine::add_civ(
        state,
        Civ {
            id: String::from("test.example"),
            title: String::from("Example"),
            ..Default::default()
        },
    ) {
        Ok(state) => state,
        Err(ex) => panic!("{:?}", ex),
    };
    assert!(state.civs.contains_key("test.example[0]"));
    assert_eq!(state.civs.len(), 1);
    assert_eq!(state.revision, 1);
}
