use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

use neociv_macros::StateTable;

#[derive(Clone, Debug, Serialize, Deserialize, Default, SerdeDiff, StateTable)]
pub struct Alignment {
    pub id: String,
    pub left: String,
    pub right: String,
    pub default: f32,
    pub value: f32,
}

pub type Alignments = HashMap<String, Alignment>;
