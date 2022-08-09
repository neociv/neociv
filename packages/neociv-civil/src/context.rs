use rlua::Lua;

use neociv_state::state::NeocivState;

use crate::cvl;

pub struct NeocivContext {
    pub state: NeocivState,
    pub context: Lua,
}

impl Default for NeocivContext {
    fn default() -> Self {
        return NeocivContext {
            state: NeocivState::default(),
            context: match cvl::init() {
                Ok(lua) => lua,
                Err(ex) => panic!("{:?}", ex),
            },
        };
    }
}
