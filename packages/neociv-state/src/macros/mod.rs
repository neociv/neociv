#[macro_export]
macro_rules! state_enum {
    ($i:item) => {
        #[derive(
            Copy,
            Clone,
            Debug,
            serde::Serialize,
            serde::Deserialize,
            serde_diff::SerdeDiff,
            neociv_macros::StateEnum,
        )]
        $i
    };
}

#[macro_export]
macro_rules! state_enum_default {
    ($i:item) => {
        #[derive(
            Copy,
            Clone,
            Debug,
            Default,
            serde::Serialize,
            serde::Deserialize,
            serde_diff::SerdeDiff,
            neociv_macros::StateEnum,
        )]
        $i
    };
}

#[macro_export]
macro_rules! state_table {
    ($i:item) => {
        #[derive(
            Clone,
            Debug,
            serde::Serialize,
            serde::Deserialize,
            serde_diff::SerdeDiff,
            neociv_macros::StateTable,
        )]
        $i
    };
}
