use neociv_state::state::NeocivState;
use rlua::{
    Error as LuaError, MultiValue as LuaMultiValue, Result as LuaResult, Value as LuaValue,
};
use rustyline::Editor;

use crate::runtime::NeocivRuntime;
pub trait NeocivRepl {
    fn lua_repl(&self) -> &Self;
    fn fnl_repl(&self) -> &Self;
    fn lua_repl_line(&self, line: &str) -> LuaResult<String>;
}

impl<C> NeocivRepl for NeocivRuntime<C> where C: Fn(String, LuaValue) -> NeocivState {
    fn lua_repl_line(&self, line: &str) -> LuaResult<String> {
        return self.lua.lock().unwrap().context(move |ctx| {
            let result = ctx.load(line).eval::<LuaMultiValue>();
            match result {
                Ok(values) => Ok(values
                    .iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<_>>()
                    .join("\t")),
                Err(ex) => Err(ex),
            }
        });
    }

    fn lua_repl(&self) -> &Self {
        self.lua.lock().unwrap().context(move |ctx| loop {
            let mut editor = Editor::<()>::new().unwrap();

            loop {
                let mut prompt = "> ";
                let mut line = String::new();

                loop {
                    match editor.readline(prompt) {
                        Ok(input) => line.push_str(&input),
                        Err(_) => return,
                    }

                    match ctx.load(&line).eval::<LuaMultiValue>() {
                        Ok(values) => {
                            editor.add_history_entry(line);
                            println!(
                                "{}",
                                values
                                    .iter()
                                    .map(|value| format!("{:?}", value))
                                    .collect::<Vec<_>>()
                                    .join("\t")
                            );
                            break;
                        }
                        Err(LuaError::SyntaxError {
                            incomplete_input: true,
                            ..
                        }) => {
                            line.push_str("\n");
                            prompt = ">> ";
                        }
                        Err(e) => {
                            eprintln!("error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
        return self;
    }

    fn fnl_repl(&self) -> &Self {
        return self;
    }
}
