use std::collections::HashMap;

use crate::state_table;

state_table! {
    pub struct Alignment {
        pub id: String,
        pub left: String,
        pub right: String,
        pub default: f32,
        pub value: f32,
    }
}

pub type Alignments = HashMap<String, Alignment>;
