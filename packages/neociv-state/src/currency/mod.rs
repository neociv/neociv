use crate::state_table;

pub type CurrencyId = String;

state_table! {
    pub struct CurrencyDesc {
        id: CurrencyId,
        title: String,
    }
}

state_table! {
    pub struct CurrencyState {
        id: CurrencyId,
        value: u32,
    }
}
