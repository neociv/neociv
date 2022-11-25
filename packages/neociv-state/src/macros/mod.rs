#[macro_export]
macro_rules! state_struct {
    ($struct: ident) => {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff, neociv_macros::StateTable)]
        pub struct $struct
    }
}
pub(crate) use state_struct;

#[macro_export]
macro_rules! state_enum {
    ($enum: ident, $($element: ident),*) => {
        #[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff, neociv_macros::StateEnum)]
        pub enum $enum {
            $($element),*
        }
    }
}
pub(crate) use state_enum;