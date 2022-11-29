#[macro_export]
macro_rules! state_enum {
    ($i:item) => {
        #[derive(
            Copy,
            Clone,
            Debug,
            bevy_ecs::component::Component,
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
            bevy_ecs::component::Component,
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
            bevy_ecs::component::Component,
            serde::Serialize,
            serde::Deserialize,
            serde_diff::SerdeDiff,
            neociv_macros::StateTable,
        )]
        $i
    };
}

#[macro_export]
macro_rules! state_table_default {
    ($i:item) => {
        #[derive(
            Clone,
            Debug,
            Default,
            bevy_ecs::component::Component,
            serde::Serialize,
            serde::Deserialize,
            serde_diff::SerdeDiff,
            neociv_macros::StateTable,
        )]
        $i
    };
}


