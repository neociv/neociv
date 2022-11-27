use std::collections::HashMap;

use crate::state_enum;

state_enum! {
    pub struct Alignment {
        pub id: String,
        pub left: String,
        pub right: String,
        pub default: f32,
        pub value: f32,
    }
}

pub type Alignments = HashMap<String, Alignment>;
