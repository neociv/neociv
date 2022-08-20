use neociv_civil::runtime::{repl::NeocivRepl, NeocivRuntime};
use neociv_state::state::NeocivState;

fn main() {
    NeocivRuntime::default()
        .inject_state(&NeocivState::default())
        .unwrap()
        .lua_repl();
}
