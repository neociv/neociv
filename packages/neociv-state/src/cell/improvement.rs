use neociv_macros::StateTable;
use serde::{Deserialize, Serialize};

pub enum ImprovementCategory {
    Building,
    City,
    District,
}

#[derive(Clone, Debug, Serialize, Deserialize, StateTable)]
pub struct Improvement {
    //pub category: ImprovementCategory,
    pub integrity: f32,
    pub integrity_max: f32,
    pub content: String,
}
