use neociv_macros::{state_enum, state_struct};

state_enum! {
    pub enum ImprovementCategory {
        Building,
        City,
        District
    }
}

state_struct! {
    pub struct Improvement {
        category: ImprovementCategory,
        integrity: f32,
        integrity_max: f32,
        content: String
    }
}
