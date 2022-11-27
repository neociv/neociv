use crate::state_table;

pub type UnitId = String;
pub type UnitKey = String;

state_table! {
    pub struct StateUnit {
        pub id: UnitId,
        pub title: String,
    }
}

state_table! {
    pub struct StateUnitEntity {
        pub id: UnitId,
        pub key: UnitKey,
    }
}
