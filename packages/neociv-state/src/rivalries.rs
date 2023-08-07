use std::collections::HashMap;

use crate::state_table;

state_table! {
    pub struct Rivalry {
        pub id: String,
        pub a: String,
        pub b: String,
        pub modifier: f32,
    }
}

pub type Rivalries = HashMap<String, Rivalry>;
