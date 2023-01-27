pub mod building;
pub mod unit;

#[macro_export]
macro_rules! neociv_desc {
    ($kind: ident { $($props:tt)* }) => {
        paste::paste! {
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff, bevy_ecs::component::Component, bevy_ecs::system::Resource)]
            pub struct [<Neociv $kind DescProps>] {
                $($props)*
            }

            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff, bevy_ecs::component::Component, bevy_ecs::system::Resource)]
            pub struct [<Neociv $kind Desc>] {
                id: String,
                props: [<Neociv $kind DescProps>],
                content: crate::desc::NeocivDescContent,
            }

            impl [<Neociv $kind Desc>] {
                pub fn new(id: String, props: [<Neociv $kind DescProps>], content: crate::desc::NeocivDescContent) -> Self {
                    Self {
                        id,
                        props,
                        content,
                    }
                }
            }

            impl crate::desc::NeocivDesc<[<Neociv $kind DescProps>]> for [<Neociv $kind Desc>] {
                fn id(&self) -> String {
                    self.id.to_owned()
                }

                fn kind(&self) -> crate::desc::NeocivDescKind {
                    crate::desc::NeocivDescKind::$kind
                }

                fn props(&self) -> [<Neociv $kind DescProps>] {
                    self.props.to_owned()
                }

                fn content(&self) -> crate::desc::NeocivDescContent {
                    self.content.to_owned()
                }
            }
        }
    };
}

pub(crate) use neociv_desc;

pub enum NeocivDescKind {
    Building,
    Unit,
}

/**
 * Define the actual media / resources content of an entity.
 */
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize, serde_diff::SerdeDiff)]
pub struct NeocivDescContent {
    mesh: String,
    materials: Vec<String>,
}

pub trait NeocivDesc<T> {
    fn id(&self) -> String;
    fn kind(&self) -> NeocivDescKind;
    fn props(&self) -> T;
    fn content(&self) -> NeocivDescContent;
}
