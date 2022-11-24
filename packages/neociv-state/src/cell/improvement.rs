use neociv_macros::{StateEnum, StateTable};

use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, SerdeDiff, StateEnum)]
pub enum ImprovementCategory {
    Building,
    City,
    District,
}

#[derive(Clone, Debug, Serialize, Deserialize, SerdeDiff, StateTable)]
pub struct Improvement {
    //pub category: ImprovementCategory,
    pub integrity: f32,
    pub integrity_max: f32,
    pub content: String,
}
