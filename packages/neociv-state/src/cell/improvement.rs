use crate::{state_enum, state_table};

state_enum! {
    pub enum ImprovementCategory {
        Building,
        City,
        District
    }
}

state_table! {
    pub struct Improvement {
        category: ImprovementCategory,
        integrity: f32,
        integrity_max: f32,
        content: String
    }
}
