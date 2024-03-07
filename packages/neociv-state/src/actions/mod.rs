use crate::actions::supply::SupplyAction;

pub mod common;
pub mod meta;
pub mod supply;
pub mod value;

pub enum StateAction {
    SupplyAction,
}
