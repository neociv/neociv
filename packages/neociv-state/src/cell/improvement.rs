use crate::macros::{state_enum, state_struct};

state_enum!(ImprovementCategory, Building, City, District);

state_struct!(Improvement) {
    pub category: ImprovementCategory,
    pub integrity: f32,
    pub integrity_max: f32,
    pub content: String
};