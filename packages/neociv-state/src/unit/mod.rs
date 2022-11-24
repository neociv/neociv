use serde::{Serialize, Deserialize};
use serde_diff::SerdeDiff;

use neociv_macros::StateTable;

pub type UnitId = String;
pub type UnitKey = String;

#[derive(Clone, Debug, Serialize, Deserialize, SerdeDiff, StateTable)]
pub struct StateUnit {
    pub id: UnitId,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, SerdeDiff, StateTable)]
pub struct StateUnitEntity {
    pub id: UnitId,
    pub key: UnitKey,
}


