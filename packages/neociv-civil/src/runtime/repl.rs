use rlua::{Error as LuaError, MultiValue as LuaMultiValue};
use rustyline::Editor;

use crate::runtime::NeocivRuntime;
pub trait NeocivRepl {
    fn lua_repl(&self) -> &Self;
    fn fnl_repl(&self) -> &Self;
}

impl NeocivRepl for NeocivRuntime {
    fn lua_repl(&self) -> &Self {
        self.lua.context(move |ctx| loop {
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
                                values.iter()
                                .map(|value| format!("{:?}", value))
                                .collect::<Vec<_>>()
                                .join("\t")
                            );
                            break;
                        }
                        Err(LuaError::SyntaxError { incomplete_input: true, .. }) => {
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
