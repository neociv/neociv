use neociv_state::*;

#[test]
fn init() {
    let state: state::NeocivState = engine::init();
    assert_eq!(state.revision, 0);
    assert_eq!(state.turn, 0);
    assert_eq!(state.civs.len(), 0);
    assert_eq!(state.grid.cells.len(), 0);
}
